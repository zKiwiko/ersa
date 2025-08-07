use crate::cli::pkg::utils::PackageManager;
use std::fs;

/// Remove an installed package
pub fn remove_package(package_name: &str) -> Result<(), String> {
    // Ensure library directory exists
    PackageManager::ensure_lib_directory()?;

    // Check if package exists
    if !PackageManager::package_exists(package_name)? {
        return Err(format!("Package '{}' not found", package_name));
    }

    let package_dir = PackageManager::get_package_directory(package_name)?;

    // Remove the package directory
    fs::remove_dir_all(&package_dir)
        .map_err(|e| format!("Failed to remove package directory: {}", e))?;

    PackageManager::log_operation_success("removed", package_name);
    
    Ok(())
}