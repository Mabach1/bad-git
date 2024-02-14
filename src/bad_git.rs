use std::{
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
fn push_dir_contents(dir_path: &str) -> Option<Vec<String>> {
    let mut files = vec![];

    match fs::read_dir(dir_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();

                    files.push(entry_path.to_string_lossy().into_owned());

                    if entry_path.is_dir() {
                        push_dir_contents(&entry_path.to_string_lossy())?;
                    }
                }
            }
        }
        Err(_) => return None,
    }

    Some(files)
}

fn copy_file(from: &str, to: &str) {
    if let Some(parent_dir) = Path::new(to).parent() {
        fs::create_dir_all(parent_dir)
            .unwrap_or_else(|e| panic!("Error creating directory {}: {}", parent_dir.display(), e));
    }
    fs::copy(from, to).unwrap();
}

pub fn add(paths: &Vec<String>) -> Result<(), BadGitError> {
    let mut sources: Vec<String> = vec![];

    for path in paths {
        if !exists(path) {
            return Err(BadGitError::FileDoesNotExists(path.to_string()));
        }

        if Path::is_dir(&PathBuf::from(path)) {
            sources.append(&mut push_dir_contents(path).unwrap());
            continue;
        }

        sources.push(path.to_string());
    }

    let destinations: Vec<String> = sources
        .iter()
        .map(|e| format!("{}/{}", ADD_DIR_PATH, e))
        .collect();

    for (src, dst) in sources.iter().zip(destinations.iter()) {
        copy_file(src, dst);
    }

    Ok(())
}
