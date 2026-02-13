pub mod github;
use std::os::unix::fs::PermissionsExt;

#[cfg(target_os = "windows")]
pub const ERSA_USER_DIR: &str = concat!(env!("APPDATA"), "\\ersa");
#[cfg(not(target_os = "windows"))]
pub const ERSA_USER_DIR: &str = concat!(env!("HOME"), "/.local/share/ersa");

pub async fn get_latest_version(url: &str) -> Result<String, String> {
    let repo_info = github::get_repoinfo(url).await.map_err(|e| e.to_string())?;
    let json: serde_json::Value = serde_json::from_str(&repo_info)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    // Check if the API returned an error
    if let Some(message) = json.get("message") {
        return Err(format!("GitHub API error: {}", message.as_str().unwrap_or("Unknown error")));
    }
    
    let tag_name = json["tag_name"]
        .as_str()
        .ok_or("No tag_name field in response")?
        .to_string();
    Ok(tag_name)
}

pub async fn download_latest_release(url: &str) -> Result<(), String> {
    let repo_info = github::get_repoinfo(url).await.map_err(|e| e.to_string())?;
    let json: serde_json::Value = serde_json::from_str(&repo_info)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // Check if the API returned an error
    if let Some(message) = json.get("message") {
        return Err(format!("GitHub API error: {}", message.as_str().unwrap_or("Unknown error")));
    }

    let assets = json["assets"]
        .as_array()
        .ok_or("No assets field in response")?;

    #[cfg(target_os = "windows")]
    let asset_name = "ersa_lsp.exe";

    #[cfg(not(target_os = "windows"))]
    let asset_name = "ersa_lsp";

    let asset = assets
        .iter()
        .find(|a| a["name"].as_str() == Some(asset_name))
        .ok_or(format!("Asset '{}' not found in release", asset_name))?;

    let download_url = asset["browser_download_url"]
        .as_str()
        .ok_or("No download URL found")?;

    let response = reqwest::Client::new()
        .get(download_url)
        .send()
        .await
        .map_err(|e| format!("Failed to download: {}", e))?;
    let bytes = response.bytes().await.map_err(|e| format!("Failed to read bytes: {}", e))?;

    std::fs::create_dir_all(ERSA_USER_DIR)
        .map_err(|e| format!("Failed to create directory: {}", e))?;

    let file_path = format!("{}/{}", ERSA_USER_DIR, asset_name);
    std::fs::write(&file_path, bytes)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    #[cfg(not(target_os = "windows"))]
    {
        let mut perms = std::fs::metadata(&file_path).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&file_path, perms).unwrap();
    }

    Ok(())
}
