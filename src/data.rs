use crate::utils;
use serde::Serialize;
use std::{env, error::Error, path::PathBuf};

#[derive(Serialize)]
pub struct Directory {
    path: Option<String>,
    name: Option<String>,
    directories: Vec<Directory>,
    manifests: Vec<ManifestMetadata>,
}
#[derive(Serialize)]
pub struct ManifestMetadata {
    pub path: String,
    pub name: String,
    pub content: Option<String>,
}

pub fn get_path_or_default(partial_path: Option<PathBuf>) -> Result<PathBuf, Box<dyn Error>> {
    let container_dir = env::var("CONTAINER_DIR")?;
    let container_dir = PathBuf::from(container_dir);
    let full_path = match utils::safely_join(container_dir, partial_path.unwrap_or_default()) {
        Some(path) => path,
        None => return Err("Invalid path".into()),
    };
    Ok(full_path)
}

pub fn get_directory(partial_path: Option<PathBuf>) -> Result<Directory, Box<dyn Error>> {
    let full_path = get_path_or_default(partial_path.clone())?;

    let mut directories = Vec::new();
    let mut manifests = Vec::new();
    let entries = std::fs::read_dir(&full_path)?;
    entries.for_each(|entry| {
        if let Ok(entry) = entry {
            let file_name = utils::gt_dir_entry(&entry);
            if let Ok(metadata) = entry.metadata() {
                let file_path = match &partial_path {
                    Some(path) => format!("{}/{}", path.to_string_lossy(), file_name),
                    None => file_name.clone(),
                };
                if metadata.is_dir() {
                    directories.push(Directory {
                        path: Some(file_path),
                        name: Some(file_name),
                        directories: Vec::new(),
                        manifests: Vec::new(),
                    });
                } else if utils::is_tag(metadata, &file_name) {
                    manifests.push(ManifestMetadata {
                        path: file_path,
                        name: file_name.trim_end_matches(".json").into(),
                        content: None,
                    })
                }
            }
        }
    });

    Ok(Directory {
        path: partial_path.as_ref().map(|p| utils::gt_path(p)),
        name: partial_path.as_ref().map(|p| utils::gt_file_name(p)),
        directories,
        manifests,
    })
}

pub fn get_manifest(partial_path: PathBuf) -> Result<ManifestMetadata, Box<dyn Error>> {
    let full_path = get_path_or_default(Some(partial_path.clone()))?;

    let manifest = std::fs::read_to_string(&full_path)?;
    let manifest: serde_json::Value = serde_json::from_str(&manifest)?;
    let manifest_string = serde_json::to_string_pretty(&manifest)?;

    Ok(ManifestMetadata {
        path: utils::gt_path(&partial_path),
        name: utils::gt_file_name(&full_path)
            .trim_end_matches(".json")
            .into(),
        content: Some(manifest_string),
    })
}
