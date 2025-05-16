use crate::cli::console;
use reqwest::Client;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use zip::ZipArchive;

#[derive(Deserialize)]
pub struct Lib {
    pub name: String,
    pub url: String,
    pub version: String,
    pub dependencies: Vec<String>,
}

#[derive(Deserialize)]
pub struct GithubFile {
    pub content: String,
    pub encoding: String,
}

pub fn get_app_directory() -> Result<PathBuf, String> {
    let exe_path =
        env::current_exe().map_err(|e| format!("Failed to get executable path: {}", e))?;

    let app_dir = exe_path
        .parent()
        .ok_or_else(|| "Failed to get parent directory of executable".to_string())?;

    Ok(app_dir.to_path_buf())
}

pub fn extract_github_info(git_url: &str) -> Result<(String, String), String> {
    let url = git_url.trim_end_matches(".git").trim_end_matches('/');
    let parts: Vec<&str> = url.split('/').collect();

    if parts.len() < 2 {
        return Err("Invalid Git URL".to_string());
    }

    let owner = parts[parts.len() - 2].to_string();
    let repo = parts[parts.len() - 1].to_string();

    Ok((owner, repo))
}

pub async fn download_and_extract_repo(git_url: &str, target_dir: &Path) -> Result<(), String> {
    let (owner, repo) = extract_github_info(git_url)?;

    let download_url = format!(
        "https://github.com/{}/{}/archive/refs/heads/main.zip",
        owner, repo
    );

    console::info(&format!("Downloading repository from {}...", download_url));

    let client = Client::new();

    let response = client
        .get(&download_url)
        .header("User-Agent", "ersa")
        .send()
        .await
        .map_err(|e| format!("Failed to download repository: {}", e))?;

    if !response.status().is_success() {
        let master_url = format!(
            "https://github.com/{}/{}/archive/refs/heads/master.zip",
            owner, repo
        );
        console::info(&"'Main' branch not found. Trying master branch...");

        let response = client
            .get(&master_url)
            .header("User-Agent", "ersa")
            .send()
            .await
            .map_err(|e| format!("Failed to download repository: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "Failed to download repository. Status: {}",
                response.status()
            ));
        }
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    let app_dir = get_app_directory()?;
    let temp_dir = app_dir.join("tmp");

    if !temp_dir.exists() {
        fs::create_dir_all(&temp_dir)
            .map_err(|e| format!("Failed to create temporary directory: {}", e))?;
    }

    let temp_zip = temp_dir.join("repo.zip");

    fs::write(&temp_zip, &bytes).map_err(|e| format!("Failed to write ZIP file: {}", e))?;

    if let Some(parent) = target_dir.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create target directory: {}", e))?;
        }
    }

    let file = fs::File::open(&temp_zip).map_err(|e| format!("Failed to open ZIP file: {}", e))?;

    let mut archive =
        ZipArchive::new(file).map_err(|e| format!("Failed to read ZIP archive: {}", e))?;

    let root_dir_name = if archive.len() > 0 {
        let first_file = archive
            .by_index(0)
            .map_err(|e| format!("Failed to access first file in ZIP archive: {}", e))?;

        let name = first_file.name();
        if let Some(pos) = name.find('/') {
            name[0..pos].to_string()
        } else {
            return Err("Invalid ZIP structure".to_string());
        }
    } else {
        return Err("Empty ZIP archive".to_string());
    };

    archive
        .extract(&temp_dir)
        .map_err(|e| format!("Failed to extract ZIP archive: {}", e))?;

    console::log("Extracting files to a temporary directory...");

    let extracted_dir = temp_dir.join(&root_dir_name);

    console::log("Moving files to target directory...");

    if !target_dir.exists() {
        fs::create_dir_all(target_dir)
            .map_err(|e| format!("Failed to create target directory: {}", e))?;
    }

    copy_dir_contents(&extracted_dir, target_dir)
        .map_err(|e| format!("Failed to move extracted files: {}", e))?;

    fs::remove_dir_all(&temp_dir)
        .map_err(|e| format!("Failed to clean up temporary directory: {}", e))?;

    console::success(&format!(
        "Repository downloaded and extracted to {:?}",
        target_dir
    ));

    Ok(())
}

pub fn copy_dir_contents(src: &Path, dst: &Path) -> Result<(), String> {
    if !dst.exists() {
        fs::create_dir_all(dst)
            .map_err(|e| format!("Failed to create directory {:?}: {}", dst, e))?;
    }

    for entry in
        fs::read_dir(src).map_err(|e| format!("Failed to read directory {:?}: {}", src, e))?
    {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let file_type = entry
            .file_type()
            .map_err(|e| format!("Failed to get file type: {}", e))?;

        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_contents(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path).map_err(|e| {
                format!(
                    "Failed to copy file {:?} to {:?}: {}",
                    src_path, dst_path, e
                )
            })?;
        }
    }

    Ok(())
}

pub fn api_url(git_url: &str) -> String {
    let (owner, repo) = match extract_github_info(git_url) {
        Ok((owner, repo)) => (owner, repo),
        Err(_) => return "Invalid Git URL".to_string(),
    };

    format!(
        "https://api.github.com/repos/{}/{}/contents/lib.json",
        owner, repo
    )
}
