use parser::FileType;
use std::fs;
use std::path::{Path, PathBuf};

fn parse_extension(path: &PathBuf) -> Option<FileType> {
    let ext = path.extension()?.to_str()?;
    match ext {
        "xlsx" => Some(FileType::Xlsx),
        _ => None,
    }
}

pub fn collect_files_recursively(path: &Path) -> Vec<(FileType, PathBuf)> {
    let mut files = Vec::new();
    tracing::debug!("Collecting files from: {:?}", path.is_dir());
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                // Recursively collect files from subdirectories
                files.extend(collect_files_recursively(&path));
            } else if path.is_file() {
                // Add file to the list
                if let Some(ext) = parse_extension(&path) {
                    tracing::debug!("Found file: {:?}", path);
                    files.push((ext, path));
                } else {
                    tracing::error!("Unsupported file type: {:?}", path);
                }
            }
        }
    } else {
        let path = path.to_path_buf();
        if let Some(ext) = parse_extension(&path) {
            tracing::debug!("Found file: {:?}", path);
            files.push((ext, path));
        } else {
            tracing::error!("Unsupported file type: {:?}", path);
        }
    }

    files
}
