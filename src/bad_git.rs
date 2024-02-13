use std::fs;

#[derive(Debug)]
pub enum BadGitError {
    NoArgumentsProvided,
    InvalidCommand,
    DidNotProvideFilesToAdd,
    HasNotBeenInitialized,
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
        fs::remove_dir_all(ROOT_DIR_PATH).unwrap_or_else(|error| {
            println!("could not initialize bad git: Error: {}", error);
        });
    }

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
