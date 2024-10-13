use std::{ffi::OsStr, fs::DirEntry, path::Path};

pub fn gt_file_name(path: &Path) -> String {
    path.file_name()
        .unwrap_or(OsStr::new("UNKNOWN"))
        .to_string_lossy()
        .to_string()
}
pub fn gt_path(path: &Path) -> String {
    path.to_string_lossy().to_string()
}
pub fn gt_dir_entry(entry: &DirEntry) -> String {
    entry.file_name().to_string_lossy().to_string()
}
