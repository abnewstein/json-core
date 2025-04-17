use jsonschema::Validator;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum ValidationError {
    Io(std::io::Error),
    Json(serde_json::Error),
    Schema(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ValidationError::Io(e) => write!(f, "I/O error: {}", e),
            ValidationError::Json(e) => write!(f, "JSON parsing error: {}", e),
            ValidationError::Schema(e) => write!(f, "Schema validation error: {}", e),
        }
    }
}

impl std::error::Error for ValidationError {}

pub fn validate_files(
    files: &[PathBuf],
    base_dir: &str,
    schema: Option<&Validator>,
) -> Result<Vec<(PathBuf, String)>, ValidationError> {
    let errors = files
        .iter()
        .filter_map(|path| {
            let relative_path = path.strip_prefix(base_dir).ok()?;
            match validate_json_file(path, schema) {
                Ok(_) => None,
                Err(e) => Some((relative_path.to_path_buf(), e.to_string())),
            }
        })
        .collect();
    Ok(errors)
}

pub fn validate_json_file(path: &Path, schema: Option<&Validator>) -> Result<(), ValidationError> {
    let content = fs::read_to_string(path).map_err(ValidationError::Io)?;
    let json: Value = serde_json::from_str(&content).map_err(ValidationError::Json)?;
    if let Some(schema) = schema {
        schema
            .validate(&json)
            .map_err(|e| ValidationError::Schema(e.to_string()))?;
    }
    Ok(())
}
