use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;

#[derive(Debug)]
pub enum BadGitError {
    NoArgumentsProvided,
    InvalidCommand,
    DidNotProvideFilesToAdd,
    HasNotBeenInitialized,
    FileDoesNotExists(String),
}

const ROOT_DIR_PATH: &str = "./.bad-git";
const ADD_DIR_PATH: &str = "./.bad-git/adds";
const SNAPSHOT_DIR_PATH: &str = "./.bad-git/snapshots";

fn exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn is_initialized() -> bool {
    exists(ROOT_DIR_PATH)
}

pub fn init() {
    if is_initialized() {
        // removing the old bad-git directory
        fs::remove_dir_all(ROOT_DIR_PATH).unwrap_or_else(|error| {
            println!("could not initialize bad git: Error: {}", error);
        });
    }

    // create root directory and all it's subdirectories
    fs::create_dir_all(ROOT_DIR_PATH).unwrap_or_else(|error| {
        println!("could not initialize bad git: Error: {}", error);
    });

    fs::create_dir_all(ADD_DIR_PATH).unwrap_or_else(|error| {
        println!("could not initialize bad git: Error: {}", error);
    });

    fs::create_dir_all(SNAPSHOT_DIR_PATH).unwrap_or_else(|error| {
        println!("could not initialize bad git: Error: {}", error);
    });
}

// TODO: maybe do some meaningful error
fn get_dir_contents_recursive(dir_path: &str, files: &mut Vec<String>) {
    match fs::read_dir(dir_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();

                    if entry_path.is_dir() {
                        get_dir_contents_recursive(&entry_path.to_string_lossy(), files);
                    } else {
                        files.push(entry_path.to_string_lossy().into_owned());
                    }
                }
            }
        }
        Err(_) => panic!("{dir_path} does not exists"),
    }
}

fn get_dir_contents(dir_path: &str) -> Option<Vec<String>> {
    let mut files = vec![];
    get_dir_contents_recursive(dir_path, &mut files);
    Some(files)
}

fn copy_file(from: &str, to: &str) {
    if let Some(parent_dir) = Path::new(to).parent() {
        fs::create_dir_all(parent_dir)
            .unwrap_or_else(|e| panic!("Error creating directory {}: {}", parent_dir.display(), e));
    }
    match fs::copy(from, to) {
        Ok(_) => (),
        Err(err) => println!("could not copy file from: {from} to: {to}, because fo error: {err}"),
    }
}

fn get_files_to_ignore() -> HashSet<String> {
    let mut files = HashSet::new();

    // ignoring bad git files
    let bad_git_files =
        get_dir_contents(ROOT_DIR_PATH).expect("Bad git should have been initialized");

    bad_git_files.iter().for_each(|file| {
        files.insert(file.to_string());
    });

    files.insert("./.badignore".to_string());

    let contents = match fs::read_to_string(".badignore") {
        Ok(content) => content,
        Err(error) => {
            println!("could not read .badignore; error: {error}");
            return files;
        }
    };

    contents.split_whitespace().for_each(|file| {
        if Path::is_dir(&PathBuf::from(file)) {
            let dir_content = get_dir_contents(file).unwrap();
            dir_content.iter().for_each(|file| {
                files.insert(normalize_path(file));
            });
        } else {
            files.insert(normalize_path(file));
        }
    });

    files
}

fn grant_permissions(filename: &str) {
    if !exists(filename) {
        return;
    }

    let source_permissions = fs::metadata(PathBuf::from(filename)).unwrap().permissions();
    if !source_permissions.mode() & 0o200 != 0 {
        let new_permissions = Permissions::from_mode(source_permissions.mode() | 0o200);
        fs::set_permissions(filename, new_permissions).unwrap();
    }
}

fn normalize_path(path: &str) -> String {
    let normalized_path: PathBuf = PathBuf::from(path).components().collect();
    normalized_path.to_str().unwrap().to_string()
}

pub fn add(paths: &Vec<String>) -> Result<(), BadGitError> {
    let mut sources: Vec<String> = vec![];
    let files_to_ignore = get_files_to_ignore();

    for path in paths {
        if !exists(path) {
            return Err(BadGitError::FileDoesNotExists(path.to_string()));
        }

        if Path::is_dir(&PathBuf::from(path)) {
            sources.append(&mut get_dir_contents(path).unwrap());
            continue;
        }

        sources.push(path.to_string());
    }

    let sources: Vec<String> = sources.iter().map(|s| normalize_path(s)).collect();

    let sources: Vec<String> = sources
        .iter()
        // this allocation is bad, we don't care
        .filter(|s| !files_to_ignore.contains(&s[2..].to_string()))
        .map(|s| s.to_string())
        .collect();

    let destinations: Vec<String> = sources
        .iter()
        .map(|d| format!("{}/{}", ADD_DIR_PATH, d))
        .collect();

    destinations.iter().for_each(|d| grant_permissions(d));

    for (src, dst) in sources.iter().zip(destinations.iter()) {
        copy_file(src, dst);
    }

    Ok(())
}
