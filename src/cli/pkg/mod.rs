pub mod git;
mod install;
mod list;
mod remove;
mod update;
mod utils;

use utils::PackageManager;

/// Install a package from a Git URL
pub async fn download(url: &str) -> Result<(), String> {
    install::install_from_url(url).await
}

/// Update an existing package to the latest version
pub async fn update(package_name: &str) -> Result<(), String> {
    PackageManager::validate_package_name(package_name)?;
    update::update_package(package_name).await
}

/// List installed packages or show details for a specific package
pub fn list(package_name: &Option<String>) -> Result<(), String> {
    match package_name {
        Some(name) => {
            PackageManager::validate_package_name(name)?;
            list::list_specific_package(name)
        }
        None => list::list_all_packages(),
    }
}

/// Remove an installed package
pub fn remove(package_name: &str) -> Result<(), String> {
    PackageManager::validate_package_name(package_name)?;
    remove::remove_package(package_name)
}
