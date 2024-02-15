use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

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
    fs::copy(from, to).unwrap();
}

fn get_files_to_ignore() -> HashSet<String> {
    let ignore_file_path = "./.badignore";
    let mut files = HashSet::new();

    let bad_git_files =
        get_dir_contents(ROOT_DIR_PATH).expect("Bad git should have been initialized");

    bad_git_files.iter().for_each(|file| {
        files.insert(file.to_string());
    });

    let contents = match fs::read_to_string(ignore_file_path) {
        Ok(content) => content,
        Err(_) => return files,
    };

    contents.split_whitespace().for_each(|file| {
        if Path::is_dir(&PathBuf::from(file)) {
            let dir_content = get_dir_contents(file).unwrap();
            dir_content.iter().for_each(|file| {
                files.insert(file.to_string());
            });
        } else {
            files.insert(file.to_string());
        }
    });

    files
}

pub fn add(paths: &Vec<String>) -> Result<(), BadGitError> {
    let mut sources: Vec<String> = vec![];
    let files_to_ignore = get_files_to_ignore();

    for path in paths {
        if !exists(path) {
            return Err(BadGitError::FileDoesNotExists(path.to_string()));
        }

        if files_to_ignore.contains(path) {
            continue;
        }

        if Path::is_dir(&PathBuf::from(path)) {
            sources.append(&mut get_dir_contents(path).unwrap());
            continue;
        }

        sources.push(path.to_string());
    }

    let destinations: Vec<String> = sources
        .iter()
        .map(|e| format!("{}/{}", ADD_DIR_PATH, e))
        .collect();

    println!("{:?}", sources);

    for (src, dst) in sources.iter().zip(destinations.iter()) {
        copy_file(src, dst);
    }

    Ok(())
}
