use crate::cli::console;
use crate::cli::pkg::git::{Lib, get_app_directory};
use std::fs;
use std::path::{Path, PathBuf};

/// Common package management utilities and shared functionality
pub struct PackageManager;

impl PackageManager {
    /// Get the library directory path
    pub fn get_lib_directory() -> Result<PathBuf, String> {
        let app_dir = get_app_directory()?;
        Ok(app_dir.join("bin").join("lib"))
    }

    /// Get the path to a specific package directory
    pub fn get_package_directory(package_name: &str) -> Result<PathBuf, String> {
        let lib_dir = Self::get_lib_directory()?;
        Ok(lib_dir.join(package_name))
    }

    /// Check if a package exists
    pub fn package_exists(package_name: &str) -> Result<bool, String> {
        let package_dir = Self::get_package_directory(package_name)?;
        Ok(package_dir.exists())
    }

    /// Ensure the library directory exists
    pub fn ensure_lib_directory() -> Result<PathBuf, String> {
        let lib_dir = Self::get_lib_directory()?;
        if !lib_dir.exists() {
            fs::create_dir_all(&lib_dir)
                .map_err(|e| format!("Failed to create library directory: {}", e))?;
        }
        Ok(lib_dir)
    }

    /// Read and parse lib.json from a package directory
    pub fn read_package_info(package_dir: &Path) -> Result<Lib, String> {
        let lib_json_path = package_dir.join("lib.json");
        if !lib_json_path.exists() {
            return Err("lib.json not found in package directory".to_string());
        }

        let content = fs::read_to_string(&lib_json_path)
            .map_err(|e| format!("Failed to read lib.json: {}", e))?;

        serde_json::from_str(&content).map_err(|e| format!("Failed to parse lib.json: {}", e))
    }

    /// Get package name from directory
    pub fn get_package_name(package_dir: &Path) -> Result<String, String> {
        Ok(package_dir
            .file_name()
            .ok_or_else(|| "Invalid package directory".to_string())?
            .to_string_lossy()
            .to_string())
    }

    /// Validate package name format
    pub fn validate_package_name(name: &str) -> Result<(), String> {
        if name.is_empty() {
            return Err("Package name cannot be empty".to_string());
        }
        if name.contains("..") || name.contains('/') || name.contains('\\') {
            return Err("Invalid package name format".to_string());
        }
        Ok(())
    }

    /// Log package operation success
    pub fn log_operation_success(operation: &str, package_name: &str) {
        console::success(&format!(
            "Package '{}' {} successfully",
            package_name, operation
        ));
    }
}

/// HTTP utility functions for package operations
pub mod http_utils {
    use crate::cli::pkg::git::{GithubFile, api_url};
    use base64::{Engine as _, engine::general_purpose};
    use reqwest::Client;

    /// Create a configured HTTP client
    pub fn create_client() -> Client {
        Client::new()
    }

    /// Fetch and decode lib.json from a Git URL
    pub async fn fetch_remote_lib_json(git_url: &str) -> Result<String, String> {
        let client = create_client();
        let response = client
            .get(api_url(git_url))
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

        decode_github_file_content(&github_file)
    }

    /// Decode GitHub file content (handles base64 encoding)
    pub fn decode_github_file_content(github_file: &GithubFile) -> Result<String, String> {
        match github_file.encoding.as_str() {
            "base64" => {
                let decoded = general_purpose::STANDARD
                    .decode(github_file.content.replace('\n', ""))
                    .map_err(|e| format!("Failed to decode base64: {}", e))?;
                String::from_utf8(decoded).map_err(|e| format!("Failed to convert to UTF-8: {}", e))
            }
            _ => Err("Unsupported encoding".to_string()),
        }
    }
}
