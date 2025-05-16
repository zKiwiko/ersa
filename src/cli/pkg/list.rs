use crate::cli::console;
use crate::cli::pkg::git::*;
use std::fs;
use std::path::Path;

pub fn list(package_name: &Option<String>) -> Result<(), String> {
    let app_dir = get_app_directory()?;
    let lib_dir = app_dir.join("bin").join("lib");

    if !lib_dir.exists() {
        return Err("Library directory not found".to_string());
    }

    if !package_name.is_none() {
        let package_dir = lib_dir.join(package_name.clone().unwrap().to_string());

        if !package_dir.exists() {
            return Err(format!(
                "Package '{}' not found",
                package_name.clone().unwrap().to_string()
            ));
        }

        list_package_details(&package_dir)?;
    } else {
        let mut found_packages = false;

        for entry in fs::read_dir(&lib_dir)
            .map_err(|e| format!("Failed to read library directory: {}", e))?
        {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            if path.is_dir() {
                found_packages = true;
                list_package_details(&path)?;
                println!();
            }
        }

        if !found_packages {
            console::info("No packages installed");
        }
    }

    Ok(())
}

fn list_package_details(package_dir: &Path) -> Result<(), String> {
    let package_name = package_dir
        .file_name()
        .ok_or_else(|| "Invalid package directory".to_string())?
        .to_string_lossy()
        .to_string();

    console::log(&format!("\n    Package: {}", package_name));

    let lib_json_path = package_dir.join("lib.json");
    if !lib_json_path.exists() {
        console::warn(&"    Missing lib.json");
        return Ok(());
    }

    match fs::read_to_string(&lib_json_path) {
        Ok(content) => match serde_json::from_str::<Lib>(&content) {
            Ok(lib) => {
                println!("    Version: {}", lib.version);
                println!("    URL: {}", lib.url);
                println!("    Dependencies:");
                for dep in lib.dependencies {
                    println!("        - {}", dep);
                }
            }
            Err(e) => {
                console::warn(&format!("    Invalid lib.json: {}", e));
            }
        },
        Err(e) => {
            console::warn(&format!("    Failed to read lib.json: {}", e));
        }
    }

    Ok(())
}
