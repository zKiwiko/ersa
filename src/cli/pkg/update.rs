#![allow(deprecated)]
use crate::cli::console;
use crate::cli::pkg::git::*;
use base64::decode;
use reqwest::Client;
use semver::Version;
use std::fs;

pub async fn update(package_name: &str) -> Result<(), String> {
    let app_dir = get_app_directory()?;
    let package_dir = app_dir.join("bin").join("lib").join(package_name);

    if !package_dir.exists() {
        return Err(format!("Package '{}' not found", package_name));
    }

    let lib_json_path = package_dir.join("lib.json");
    if !lib_json_path.exists() {
        return Err(format!("Missing lib.json in package '{}'", package_name));
    }

    let lib_content = fs::read_to_string(&lib_json_path)
        .map_err(|e| format!("Failed to read lib.json: {}", e))?;

    let local_lib: Lib = serde_json::from_str(&lib_content)
        .map_err(|e| format!("Failed to parse lib.json: {}", e))?;

    let git_url = if !local_lib.url.is_empty() {
        local_lib.url
    } else {
        return Err(format!("Package '{}' does not have a URL", package_name));
    };

    let client = Client::new();
    console::info(&format!(
        "Checking for updates for package '{}'...",
        package_name
    ));

    let response = client
        .get(api_url(&git_url))
        .header("User-Agent", "ersa")
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to fetch remote lib.json. Status: {}",
            response.status()
        ));
    }

    let github_file: GithubFile = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let remote_lib_content: String;
    if github_file.encoding == "base64" {
        let decoded = decode(github_file.content.replace("\n", ""))
            .map_err(|e| format!("Failed to decode base64: {}", e))?;
        remote_lib_content =
            String::from_utf8(decoded).map_err(|e| format!("Failed to convert to UTF-8: {}", e))?;
    } else {
        return Err("Unsupported encoding".to_string());
    }

    let remote_lib: Lib = serde_json::from_str(&remote_lib_content)
        .map_err(|e| format!("Failed to parse remote lib.json: {}", e))?;

    let local_version = Version::parse(&local_lib.version)
        .map_err(|e| format!("Invalid local version format: {}", e))?;
    let remote_version = Version::parse(&remote_lib.version)
        .map_err(|e| format!("Invalid remote version format: {}", e))?;

    if remote_version > local_version {
        console::info(&format!(
            "New version available: {} -> {}",
            local_lib.version, remote_lib.version
        ));

        download_and_extract_repo(&git_url, &package_dir).await?;

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
