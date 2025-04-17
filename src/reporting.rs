use crate::comparison::{ComparisonDifference, DifferenceKind};
use std::path::PathBuf;

pub fn report_validation_errors(errors: &[(PathBuf, String)]) {
    if errors.is_empty() {
        println!("All JSON files are valid.");
    } else {
        println!("Validation errors:");
        for (path, error) in errors {
            println!("- {}: {}", path.display(), error);
        }
    }
}

pub fn report_comparison_diffs(diffs: &[ComparisonDifference]) {
    if diffs.is_empty() {
        println!("No differences found between prod and dev.");
    } else {
        println!("Comparison differences:");
        for diff in diffs {
            match diff.kind {
                DifferenceKind::Added => println!("- Added in dev: {}", diff.path.display()),
                DifferenceKind::Removed => println!("- Removed in dev: {}", diff.path.display()),
                DifferenceKind::Modified => println!("- Modified: {}", diff.path.display()),
            }
        }
    }
}
