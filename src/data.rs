use std::error::Error;

pub fn get_namespaces() -> Result<Vec<String>, Box<dyn Error>> {
    let paths = std::fs::read_dir("../container-cubby/data/containers/")?;
    let mut directories = Vec::new();
    paths.for_each(|entry| {
        if let Ok(entry) = entry {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    directories.push(entry.file_name().to_string_lossy().to_string());
                }
            }
        }
    });
    Ok(directories)
}
