use crate::cli::console;
use crate::cli::pkg::git::{download_and_extract_repo, Lib};
use crate::cli::pkg::utils::{http_utils, PackageManager};

/// Install a package from a Git URL
pub async fn install_from_url(git_url: &str) -> Result<(), String> {
    // Fetch and parse the remote lib.json to get package information
    let lib_content = http_utils::fetch_remote_lib_json(git_url).await?;
    let lib: Lib = serde_json::from_str(&lib_content)
        .map_err(|e| format!("Failed to parse lib.json: {}", e))?;

    // Validate package name
    PackageManager::validate_package_name(&lib.name)?;

    // Check if package already exists
    if PackageManager::package_exists(&lib.name)? {
        return Err(format!(
            "Package '{}' already exists. Use 'update' to upgrade it.",
            lib.name
        ));
    }

    // Get target directory for installation
    let target_dir = PackageManager::get_package_directory(&lib.name)?;

    console::log(&format!(
        "Installing package '{}' from {}...",
        lib.name, git_url
    ));

    // Download and extract the repository
    download_and_extract_repo(git_url, &target_dir).await?;

    PackageManager::log_operation_success("installed", &lib.name);
    console::info(&format!(
        "Package location: {}",
        target_dir.to_string_lossy()
    ));
    
    Ok(())
}
