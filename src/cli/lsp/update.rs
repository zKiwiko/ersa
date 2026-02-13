use std::process::Command;

const REPO_API_URL: &str = "https://api.github.com/repos/zKiwiko/ersa-lsp-core/releases/latest";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Version {
    fn parse(s: &str) -> Result<Self, String> {
        let s = s.trim().trim_start_matches('v');
        let parts: Vec<&str> = s.split('.').collect();

        if parts.len() != 3 {
            return Err(format!("Invalid version format: {}", s));
        }

        let major = parts[0]
            .parse()
            .map_err(|_| format!("Invalid major version: {}", parts[0]))?;
        let minor = parts[1]
            .parse()
            .map_err(|_| format!("Invalid minor version: {}", parts[1]))?;
        let patch = parts[2]
            .parse()
            .map_err(|_| format!("Invalid patch version: {}", parts[2]))?;

        Ok(Version {
            major,
            minor,
            patch,
        })
    }
}

/// Get the currently installed version by running `ersa_lsp --version`
fn get_installed_version() -> Result<Version, String> {
    let lsp_path = super::install::get_lsp_path();

    if !lsp_path.exists() {
        return Err("LSP not installed".to_string());
    }

    let output = Command::new(&lsp_path)
        .arg("--version")
        .output()
        .map_err(|e| format!("Failed to run LSP binary: {}", e))?;

    let version_str = String::from_utf8_lossy(&output.stdout);
    let version_str = version_str.trim();

    Version::parse(version_str)
}

/// Check for updates and return true if an update is available
pub async fn check_update() -> Result<bool, String> {
    crate::log::info("Checking for LSP server updates...");

    let latest_version_str = crate::network::get_latest_version(REPO_API_URL)
        .await
        .map_err(|e| format!("Failed to fetch latest version: {}", e))?;

    let latest_version = Version::parse(&latest_version_str)?;

    match get_installed_version() {
        Ok(installed_version) => {
            crate::log::info(&format!(
                "Installed version: {}.{}.{}",
                installed_version.major, installed_version.minor, installed_version.patch
            ));
            crate::log::info(&format!(
                "Latest version: {}.{}.{}",
                latest_version.major, latest_version.minor, latest_version.patch
            ));

            if latest_version > installed_version {
                crate::log::info("Update available!");
                Ok(true)
            } else {
                crate::log::info("LSP is up to date.");
                Ok(false)
            }
        }
        Err(_) => {
            crate::log::info("No installation found. Install first.");
            Ok(false)
        }
    }
}

/// Update the LSP server to the latest version
pub async fn update() -> Result<(), String> {
    crate::log::info("Updating LSP server...");

    // Download and replace the binary
    crate::network::download_latest_release(REPO_API_URL)
        .await
        .map_err(|e| format!("Failed to download update: {}", e))?;

    crate::log::info("LSP updated successfully!");
    Ok(())
}
