use serde::Serialize;
use std::{
    env,
    error::Error,
    path::{Path, PathBuf},
};

#[derive(Serialize)]
pub struct Directory {
    path: Option<String>,
    directories: Vec<String>,
    manifests: Vec<Manifest>,
}

pub fn get_directory(path: Option<PathBuf>) -> Result<Directory, Box<dyn Error>> {
    let container_dir = env::var("CONTAINER_DIR")?;

    let mut directories = Vec::new();
    let mut manifests = Vec::new();

    let dir_path = match &path {
        Some(path) => Path::new(container_dir.as_str()).join(path),
        None => Path::new(container_dir.as_str()).to_path_buf(),
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
        manifests: manifests
            .iter()
            .map(|path| Manifest {
                name: path.split('/').last().unwrap().to_string(),
                path: path.to_string(),
                content: None,
            })
            .collect(),
    })
}

#[derive(Serialize)]
pub struct Manifest {
    pub path: String,
    pub name: String,
    pub content: Option<String>,
}
pub fn get_manifest(path: PathBuf) -> Result<Manifest, Box<dyn Error>> {
    let container_dir = env::var("CONTAINER_DIR")?;
    let manifest_path = Path::new(container_dir.as_str()).join(&path);
    let manifest = std::fs::read_to_string(manifest_path)?;
    Ok(Manifest {
        path: path.as_path().to_string_lossy().to_string(),
        name: path.file_name().unwrap().to_string_lossy().to_string(),
        content: Some(manifest),
    })
}
