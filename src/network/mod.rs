pub mod github;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub fn get_ersa_user_dir() -> String {
    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var("APPDATA")
            .unwrap_or_else(|_| "C:\\Users\\Default\\AppData\\Roaming".to_string());
        format!("{}\\ersa", appdata)
    }
    #[cfg(not(target_os = "windows"))]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        format!("{}/.local/share/ersa", home)
    }
}

pub async fn get_latest_version(url: &str) -> Result<String, String> {
    let repo_info = github::get_repoinfo(url).await.map_err(|e| e.to_string())?;
    let json: serde_json::Value =
        serde_json::from_str(&repo_info).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // Check if the API returned an error
    if let Some(message) = json.get("message") {
        return Err(format!(
            "GitHub API error: {}",
            message.as_str().unwrap_or("Unknown error")
        ));
    }

    let tag_name = json["tag_name"]
        .as_str()
        .ok_or("No tag_name field in response")?
        .to_string();
    Ok(tag_name)
}

pub async fn download_latest_release(url: &str) -> Result<(), String> {
    let repo_info = github::get_repoinfo(url).await.map_err(|e| e.to_string())?;
    let json: serde_json::Value =
        serde_json::from_str(&repo_info).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // Check if the API returned an error
    if let Some(message) = json.get("message") {
        return Err(format!(
            "GitHub API error: {}",
            message.as_str().unwrap_or("Unknown error")
        ));
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
    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read bytes: {}", e))?;

    let user_dir = get_ersa_user_dir();
    std::fs::create_dir_all(&user_dir).map_err(|e| format!("Failed to create directory: {}", e))?;

    let file_path = format!("{}/{}", user_dir, asset_name);
    std::fs::write(&file_path, bytes).map_err(|e| format!("Failed to write file: {}", e))?;

    #[cfg(not(target_os = "windows"))]
    {
        let mut perms = std::fs::metadata(&file_path).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&file_path, perms).unwrap();
    }

    Ok(())
}
