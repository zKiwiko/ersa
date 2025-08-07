use crate::cli::console;
use crate::cli::pkg::git::Lib;
use crate::cli::pkg::utils::PackageManager;
use std::fs;
use std::path::Path;

/// List all installed packages
pub fn list_all_packages() -> Result<(), String> {
    let lib_dir = PackageManager::get_lib_directory()?;

    if !lib_dir.exists() {
        console::info("No packages installed - library directory not found");
        return Ok(());
    }

    let mut found_packages = false;

    for entry in fs::read_dir(&lib_dir)
        .map_err(|e| format!("Failed to read library directory: {}", e))?
    {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();

        if path.is_dir() {
            found_packages = true;
            display_package_details(&path)?;
            println!();
        }
    }

    if !found_packages {
        console::info("No packages installed");
    }

    Ok(())
}

/// List details for a specific package
pub fn list_specific_package(package_name: &str) -> Result<(), String> {
    let package_dir = PackageManager::get_package_directory(package_name)?;

    if !package_dir.exists() {
        return Err(format!("Package '{}' not found", package_name));
    }

    display_package_details(&package_dir)
}

/// Display detailed information about a package
fn display_package_details(package_dir: &Path) -> Result<(), String> {
    let package_name = PackageManager::get_package_name(package_dir)?;
    
    console::log(&format!("\n    Package: {}", package_name));

    // Try to read and display package information
    match PackageManager::read_package_info(package_dir) {
        Ok(lib) => display_lib_info(&lib),
        Err(e) => {
            console::warn(&format!("    Error reading package info: {}", e));
        }
    }

    Ok(())
}

/// Display library information in a formatted way
fn display_lib_info(lib: &Lib) {
    println!("    Version: {}", lib.version);
    println!("    URL: {}", lib.url);
    println!("    Dependencies:");
    
    if lib.dependencies.is_empty() {
        println!("        (none)");
    } else {
        for dep in &lib.dependencies {
            println!("        - {}", dep);
        }
    }
}
