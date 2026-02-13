use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Process import statements recursively
/// Supports: import file/path; import file/path.gpc; import "file/path";
pub fn process_imports(
    code: &str,
    base_path: &Path,
    visited: &mut HashSet<PathBuf>,
) -> Result<String, String> {
    // Match patterns:
    // - import file/path;
    // - import file/path.gpc;
    // - import "file/path";
    // - import "file/path.gpc";
    let re = Regex::new(r#"import\s+(?:"([^"]+)"|([^\s;]+))\s*;?"#)
        .map_err(|e| format!("Regex compilation error: {}", e))?;

    let mut result = String::new();
    let mut last_end = 0;

    for cap in re.captures_iter(code) {
        let match_pos = cap.get(0).unwrap();

        // Add everything before this import
        result.push_str(&code[last_end..match_pos.start()]);

        // Get the path (either from group 1 (quoted) or group 2 (unquoted))
        let path_str = cap.get(1).or(cap.get(2)).unwrap().as_str();

        // Add .gpc extension if not present
        let path_with_ext = if path_str.ends_with(".gpc") {
            path_str.to_string()
        } else {
            format!("{}.gpc", path_str)
        };

        // Resolve path relative to base_path
        let full_path = base_path.join(&path_with_ext);
        let canonical = full_path.canonicalize().map_err(|e| {
            format!(
                "Failed to resolve import path '{}' (resolved to '{}'): {}",
                path_str,
                full_path.display(),
                e
            )
        })?;

        // Check for circular imports
        if visited.contains(&canonical) {
            return Err(format!("Circular import detected: {}", canonical.display()));
        }
        visited.insert(canonical.clone());

        // Read the imported file
        let imported_code = fs::read_to_string(&canonical).map_err(|e| {
            format!(
                "Failed to read imported file '{}': {}",
                canonical.display(),
                e
            )
        })?;

        // Recursively process imports in the imported file
        let imported_base = canonical
            .parent()
            .ok_or_else(|| format!("Failed to get parent directory of {}", canonical.display()))?;
        let processed_import = process_imports(&imported_code, imported_base, visited)?;

        // Add the processed imported code
        result.push_str(&processed_import);
        result.push('\n'); // Add newline after import

        last_end = match_pos.end();
    }

    // Add remaining code
    result.push_str(&code[last_end..]);

    Ok(result)
}
