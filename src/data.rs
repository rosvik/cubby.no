use crate::utils;
use serde::Serialize;
use std::{env, error::Error, path::PathBuf};

#[derive(Serialize)]
pub struct Directory {
    path: Option<String>,
    name: Option<String>,
    directories: Vec<Directory>,
    manifests: Vec<Manifest>,
}
#[derive(Serialize)]
pub struct Manifest {
    pub path: String,
    pub name: String,
    pub content: Option<String>,
}

pub fn get_directory(path: Option<PathBuf>) -> Result<Directory, Box<dyn Error>> {
    let container_dir = env::var("CONTAINER_DIR")?;

    let mut directories = Vec::new();
    let mut manifests = Vec::new();

    let dir_path = match &path {
        Some(path) => PathBuf::from(container_dir).join(path),
        None => PathBuf::from(container_dir),
    };

    let entries = std::fs::read_dir(dir_path)?;
    entries.for_each(|entry| {
        if let Ok(entry) = entry {
            let name = utils::gt_dir_entry(&entry);
            if let Ok(metadata) = entry.metadata() {
                let file_path = match &path {
                    Some(path) => format!("{}/{}", path.to_string_lossy(), name),
                    None => name.clone(),
                };
                if metadata.is_dir() {
                    directories.push(Directory {
                        path: Some(file_path),
                        name: Some(name),
                        directories: Vec::new(),
                        manifests: Vec::new(),
                    });
                } else if metadata.is_file() && name.ends_with(".json") {
                    manifests.push(Manifest {
                        path: file_path,
                        name,
                        content: None,
                    })
                }
            }
        }
    });

    Ok(Directory {
        path: path.as_ref().map(|p| p.to_string_lossy().to_string()),
        name: path.as_ref().map(|p| utils::gt_file_name(p)),
        directories,
        manifests,
    })
}

pub fn get_manifest(path: PathBuf) -> Result<Manifest, Box<dyn Error>> {
    let container_dir = env::var("CONTAINER_DIR")?;
    let manifest_path = PathBuf::from(container_dir).join(&path);
    let manifest_content = std::fs::read_to_string(manifest_path)?;
    Ok(Manifest {
        path: utils::gt_path(&path),
        name: utils::gt_file_name(&path),
        content: Some(manifest_content),
    })
}
