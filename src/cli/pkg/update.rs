use crate::cli::console;
use crate::cli::pkg::git::{download_and_extract_repo, Lib};
use crate::cli::pkg::utils::{http_utils, PackageManager};
use semver::Version;

/// Update a package to the latest version
pub async fn update_package(package_name: &str) -> Result<(), String> {
    let package_dir = PackageManager::get_package_directory(package_name)?;

    if !package_dir.exists() {
        return Err(format!("Package '{}' not found", package_name));
    }

    // Read local package information
    let local_lib = PackageManager::read_package_info(&package_dir)?;

    if local_lib.url.is_empty() {
        return Err(format!("Package '{}' does not have a URL", package_name));
    }

    console::info(&format!(
        "Checking for updates for package '{}'...",
        package_name
    ));

    // Fetch remote package information
    let remote_lib_content = http_utils::fetch_remote_lib_json(&local_lib.url).await?;
    let remote_lib: Lib = serde_json::from_str(&remote_lib_content)
        .map_err(|e| format!("Failed to parse remote lib.json: {}", e))?;

    // Parse and compare versions
    let local_version = Version::parse(&local_lib.version)
        .map_err(|e| format!("Invalid local version format: {}", e))?;
    let remote_version = Version::parse(&remote_lib.version)
        .map_err(|e| format!("Invalid remote version format: {}", e))?;

    if remote_version > local_version {
        console::info(&format!(
            "New version available: {} -> {}",
            local_lib.version, remote_lib.version
        ));

        // Download and extract the updated package
        download_and_extract_repo(&local_lib.url, &package_dir).await?;

        console::success(&format!(
            "Package '{}' updated successfully to version {}",
            package_name, remote_lib.version
        ));
    } else {
        console::info(&format!(
            "Package '{}' is already at the latest version ({})",
            package_name, local_lib.version
        ));
    }

    Ok(())
}
