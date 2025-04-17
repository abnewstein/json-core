use self_update::cargo_crate_version;
use std::error::Error;

pub fn update_application() -> Result<(), Box<dyn Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("abnewstein")
        .repo_name("json-core")
        .bin_name("json-core")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;

    match status {
        self_update::Status::UpToDate(version) => println!("Already up to date: {}", version),
        self_update::Status::Updated(version) => println!("Updated to version: {}", version),
    }
    Ok(())
}
