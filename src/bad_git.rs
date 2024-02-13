use std::fs;

const ROOT_DIR_PATH: &str = "./.bad-git";

fn exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn is_initialized() -> bool {
    exists(ROOT_DIR_PATH)
}
