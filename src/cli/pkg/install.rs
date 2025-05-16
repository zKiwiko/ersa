#![allow(deprecated)]
use crate::cli::console;
use crate::cli::pkg::git::*;
use base64::decode;
use reqwest::Client;

pub async fn url(git_url: &str) -> Result<(), String> {
    let client = Client::new();
    let response = client
        .get(api_url(git_url))
        .header("User-Agent", "ersa")
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?
        .json::<GithubFile>()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let lib: Lib;
    let lib_content: String;

    if response.encoding == "base64" {
        let decoded = decode(response.content.replace("\n", ""));
        lib_content = String::from_utf8(decoded.unwrap()).unwrap();
    } else {
        return Err("Unsupported encoding".to_string());
    }

    lib = serde_json::from_str(&lib_content).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    let app_dir = get_app_directory()?;
    let target_dir = app_dir.join("bin").join("lib").join(lib.name.clone());

    if target_dir.exists() {
        return Err(format!(
            "Package '{}' already exists at {}",
            lib.name,
            target_dir.to_string_lossy().replace('\\', "/")
        ));
    }

    console::log(&format!(
        "Installing package '{}' from {}...",
        lib.name, git_url
    ));

    download_and_extract_repo(git_url, &target_dir).await?;

    console::success(&format!(
        "Package '{}' installed successfully at {}",
        lib.name,
        target_dir.to_string_lossy().replace('\\', "/")
    ));
    Ok(())
}
