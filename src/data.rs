use serde::Serialize;
use std::{
    error::Error,
    path::{Path, PathBuf},
};

#[derive(Serialize)]
pub struct Directory {
    path: Option<String>,
    directories: Vec<String>,
    manifests: Vec<String>,
}

pub fn get_namespaces(path: Option<PathBuf>) -> Result<Directory, Box<dyn Error>> {
    let mut directories = Vec::new();
    let mut manifests = Vec::new();

    let dir_path = match &path {
        Some(path) => Path::new("../container-cubby/data/containers/").join(path),
        None => Path::new("../container-cubby/data/containers/").to_path_buf(),
    };

    let entries = std::fs::read_dir(dir_path)?;
    entries.for_each(|entry| {
        if let Ok(entry) = entry {
            let name = entry.file_name().to_string_lossy().to_string();
            if let Ok(metadata) = entry.metadata() {
                let file_path = match &path {
                    Some(path) => format!("{}/{}", path.to_string_lossy(), name),
                    None => name.clone(),
                };
                if metadata.is_dir() {
                    directories.push(file_path);
                } else if metadata.is_file() && name.ends_with(".json") {
                    manifests.push(file_path);
                }
            }
        }
    });

    Ok(Directory {
        path: path.map(|p| p.to_string_lossy().to_string()),
        directories,
        manifests,
    })
}
