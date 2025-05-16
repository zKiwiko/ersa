use std::fs;
use crate::cli::pkg::git::*;
use crate::cli::console;

pub fn remove(package: &str) -> Result<(), String> {
    let app_dir = get_app_directory()?;
    let lib_dir = app_dir.join("bin").join("lib");

    if !lib_dir.exists() {
        return Err("Library directory not found".to_string());
    }

    let package_dir = lib_dir.join(package);

    if !package_dir.exists() {
        return Err(format!("Package '{}' not found", package));
    }

    fs::remove_dir_all(&package_dir)
        .map_err(|e| format!("Failed to remove package directory: {}", e))?;

    console::success(&format!("Package '{}' removed successfully", package));

    Ok(())
}