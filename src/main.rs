mod comparison;
mod file;
mod reporting;
mod update;
mod validation;

use clap::{Arg, Command};
use comparison::compare_files;
use file::find_json_files;
use jsonschema::draft7;
use reporting::{report_comparison_diffs, report_validation_errors};
use serde_json::Value;
use std::error::Error;
use std::fs;
use update::update_application;
use validation::validate_files;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("JsonCore")
        .version("0.1.0")
        .subcommand(
            Command::new("validate")
                .about("Validate and compare JSON files between prod and dev")
                .arg(
                    Arg::new("prod")
                        .required(true)
                        .help("Path to production directory"),
                )
                .arg(
                    Arg::new("dev")
                        .required(true)
                        .help("Path to development directory"),
                )
                .arg(Arg::new("schema").help("Path to JSON schema file")),
        )
        .subcommand(Command::new("update").about("Update to the latest version"))
        .get_matches();

    match matches.subcommand() {
        Some(("validate", sub_m)) => {
            let prod_dir = sub_m.get_one::<String>("prod").unwrap();
            let dev_dir = sub_m.get_one::<String>("dev").unwrap();
            let schema_path = sub_m.get_one::<String>("schema");

            let schema = schema_path
                .map(|p| -> Result<_, Box<dyn Error>> {
                    let schema_str = fs::read_to_string(p)?;
                    let schema_json: Value = serde_json::from_str(&schema_str)?;
                    draft7::new(&schema_json).map_err(|e| Box::new(e) as Box<dyn Error>)
                })
                .transpose()?;

            let prod_files = find_json_files(prod_dir)?;
            let dev_files = find_json_files(dev_dir)?;

            let validation_errors = validate_files(&dev_files, dev_dir, schema.as_ref())?;
            let comparison_diffs = compare_files(&prod_files, &dev_files, prod_dir, dev_dir)?;

            report_validation_errors(&validation_errors);
            report_comparison_diffs(&comparison_diffs);
        }
        Some(("update", _)) => {
            update_application()?;
        }
        _ => {
            println!("Invalid command. Use --help for usage information.");
        }
    }

    Ok(())
}
