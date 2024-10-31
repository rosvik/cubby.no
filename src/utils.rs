use path_clean::PathClean;
use std::{
    ffi::OsStr,
    fs,
    path::{Component, Path, PathBuf},
};

pub fn gt_file_name(path: &Path) -> String {
    path.file_name()
        .unwrap_or(OsStr::new("UNKNOWN"))
        .to_string_lossy()
        .to_string()
}
pub fn gt_path(path: &Path) -> String {
    path.to_string_lossy().to_string()
}
pub fn gt_dir_entry(entry: &fs::DirEntry) -> String {
    entry.file_name().to_string_lossy().to_string()
}

pub fn is_tag(metadata: fs::Metadata, file_name: &str) -> bool {
    if !(metadata.is_file() || metadata.is_symlink()) {
        return false;
    }
    if !file_name.ends_with(".json") {
        return false;
    }
    if file_name.starts_with("sha256@") {
        return false;
    }
    true
}

fn contains_parent_directory(path: &Path) -> bool {
    path.components().any(|comp| comp == Component::ParentDir)
}
pub fn safely_join(base_dir: PathBuf, path: PathBuf) -> Option<PathBuf> {
    // Lexically clean the paths to remove any redundant components
    let input_path = path.clean();

    // Check if the input path contains a parent directory
    if contains_parent_directory(&input_path) {
        return None;
    }

    // Join the base directory with the input path, and verify that it exists
    let joined_path = &base_dir.join(&input_path);
    if fs::canonicalize(joined_path.as_path()).is_err() {
        return None;
    }

    // Verify that the joined path is within the base directory
    if !joined_path.starts_with(base_dir) {
        return None;
    }
    Some(joined_path.into())
}
