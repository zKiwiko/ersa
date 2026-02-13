use std::path::PathBuf;

const REPO_API_URL: &str = "https://api.github.com/repos/zKiwiko/ersa-lsp-core/releases/latest";

#[cfg(target_os = "windows")]
const LSP_BINARY_NAME: &str = "ersa_lsp.exe";
#[cfg(not(target_os = "windows"))]
const LSP_BINARY_NAME: &str = "ersa_lsp";

/// Get the path where the LSP binary should be installed
pub fn get_lsp_path() -> PathBuf {
    let user_dir = if cfg!(target_os = "windows") {
        PathBuf::from(
            std::env::var("APPDATA")
                .unwrap_or_else(|_| String::from("C:\\Users\\Default\\AppData\\Roaming")),
        )
        .join("ersa")
    } else {
        PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| String::from("/tmp")))
            .join(".local/share/ersa")
    };

    user_dir.join(LSP_BINARY_NAME)
}

/// Check if the LSP is currently installed
#[allow(dead_code)]
pub fn is_installed() -> bool {
    get_lsp_path().exists()
}

/// Install the LSP server
pub async fn install() -> Result<(), String> {
    crate::log::info("Installing LSP server...");

    crate::network::download_latest_release(REPO_API_URL)
        .await
        .map_err(|e| format!("Failed to download LSP: {}", e))?;

    crate::log::info(&format!(
        "LSP installed successfully at: {}",
        get_lsp_path().display()
    ));
    Ok(())
}
