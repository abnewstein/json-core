use crate::file::{FileError, compute_hash};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[derive(Debug)]
pub enum ComparisonError {
    FileError(FileError),
}

impl std::fmt::Display for ComparisonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ComparisonError::FileError(e) => write!(f, "File error: {}", e),
        }
    }
}

impl std::error::Error for ComparisonError {}

#[derive(Clone, Debug)]
pub enum DifferenceKind {
    Added,
    Removed,
    Modified,
}

#[derive(Clone, Debug)]
pub struct ComparisonDifference {
    pub path: PathBuf,
    pub kind: DifferenceKind,
}

pub fn compare_files(
    prod_files: &[PathBuf],
    dev_files: &[PathBuf],
    prod_dir: &str,
    dev_dir: &str,
) -> Result<Vec<ComparisonDifference>, ComparisonError> {
    let prod_hashes: HashMap<PathBuf, String> = prod_files
        .iter()
        .filter_map(|path| {
            let relative = path.strip_prefix(prod_dir).ok()?;
            Some((
                relative.to_path_buf(),
                compute_hash(path)
                    .map_err(ComparisonError::FileError)
                    .ok()?,
            ))
        })
        .collect();

    let dev_hashes: HashMap<PathBuf, String> = dev_files
        .iter()
        .filter_map(|path| {
            let relative = path.strip_prefix(dev_dir).ok()?;
            Some((
                relative.to_path_buf(),
                compute_hash(path)
                    .map_err(ComparisonError::FileError)
                    .ok()?,
            ))
        })
        .collect();

    let prod_set: HashSet<_> = prod_hashes.keys().collect();
    let dev_set: HashSet<_> = dev_hashes.keys().collect();

    let added: Vec<_> = dev_set
        .difference(&prod_set)
        .map(|&p| ComparisonDifference {
            path: p.clone(),
            kind: DifferenceKind::Added,
        })
        .collect();

    let removed: Vec<_> = prod_set
        .difference(&dev_set)
        .map(|&p| ComparisonDifference {
            path: p.clone(),
            kind: DifferenceKind::Removed,
        })
        .collect();

    let modified: Vec<_> = prod_set
        .intersection(&dev_set)
        .filter_map(|&p| {
            if prod_hashes[p] != dev_hashes[p] {
                Some(ComparisonDifference {
                    path: p.clone(),
                    kind: DifferenceKind::Modified,
                })
            } else {
                None
            }
        })
        .collect();

    Ok([added, removed, modified].concat())
}
