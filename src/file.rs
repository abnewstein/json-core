use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug)]
pub enum FileError {
    IoError(std::io::Error),
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileError::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for FileError {}

pub fn find_json_files(dir: &str) -> Result<Vec<PathBuf>, FileError> {
    let paths = WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
        .map(|e| e.path().to_path_buf())
        .collect();
    Ok(paths)
}

pub fn compute_hash(path: &Path) -> Result<String, FileError> {
    let content = fs::read(path).map_err(FileError::IoError)?;
    let hash = Sha256::digest(&content);
    Ok(format!("{:x}", hash))
}
