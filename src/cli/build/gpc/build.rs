use crate::cli::console;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Represents the structure of a project.json file
#[derive(Serialize, Deserialize, Debug)]
struct ProjectJson {
    name: String,
    version: String,
    entry: String,
    lib: Option<String>,
}

/// Context for the import processing to avoid passing many parameters
struct ImportContext<'a> {
    project_path: &'a Path,
    lib_dir: Option<&'a str>,
    processed_files: &'a mut HashSet<PathBuf>,
    has_errors: &'a mut bool,
}

/// Reads and parses the project.json file
fn read_project_json(path: &str) -> Result<ProjectJson, String> {
    let path_obj = Path::new(path);
    if !path_obj.exists() {
        return Err(format!("Project file `{}` does not exist", path));
    }

    let content = std::fs::read_to_string(path_obj)
        .map_err(|e| format!("Failed to read `{}`: {}", path, e))?;

    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse JSON from `{}`: {}", path, e))
}

/// Converts import path notation (A::B::C) to file system path (A/B/C)
fn convert_import_path_to_file_path(import_path: &str) -> String {
    import_path.replace("::", "/")
}

/// Resolves an import path to the actual file path on disk
fn resolve_import_file_path(import_path: &str, context: &ImportContext) -> PathBuf {
    if import_path.starts_with("local::") {
        // Handle local imports - use project root as base
        let local_path = import_path.strip_prefix("local::").unwrap();
        let converted_path = convert_import_path_to_file_path(local_path);
        context
            .project_path
            .join(converted_path)
            .with_extension("gpc")
    } else {
        // Handle library imports - use lib directory and look for lib.gpc
        let converted_path = convert_import_path_to_file_path(import_path);
        if let Some(lib) = context.lib_dir {
            // Use absolute path if lib_dir starts with drive letter or is absolute
            let lib_path = if Path::new(lib).is_absolute() {
                PathBuf::from(lib)
            } else {
                context.project_path.join(lib)
            };
            lib_path.join(converted_path).join("gpc/lib.gpc")
        } else {
            // Fallback to project root if no lib directory specified
            context
                .project_path
                .join(converted_path)
                .with_extension("gpc")
        }
    }
}

/// Extracts all import statements from the given content
fn extract_imports(content: &str) -> Vec<String> {
    let import_regex = Regex::new(r"use\s+([^;]+);").unwrap();
    import_regex
        .captures_iter(content)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        .collect()
}

/// Processes a single import and returns the processed content for that import
fn process_single_import(import_path: &str, context: &mut ImportContext) -> Option<String> {
    let import_file_path = resolve_import_file_path(import_path, context);

    // Canonicalize the path to handle relative paths and ensure uniqueness
    let canonical_path = match import_file_path.canonicalize() {
        Ok(path) => path,
        Err(_) => {
            console::err(&format!(
                "Import file `{}` does not exist",
                import_file_path.display()
            ));
            *context.has_errors = true;
            return None;
        }
    };

    // Check for circular imports BEFORE processing
    if context.processed_files.contains(&canonical_path) {
        console::err(&format!(
            "Circular import detected: `{}`",
            canonical_path.display()
        ));
        *context.has_errors = true;
        return Some(format!("// Circular import skipped: {}", import_path));
    }

    if !import_file_path.exists() {
        console::err(&format!(
            "Import file `{}` does not exist",
            import_file_path.display()
        ));
        *context.has_errors = true;
        return None;
    }

    console::info(&format!(
        "Processing import: {} -> {}",
        import_path,
        import_file_path.display()
    ));

    // Add this file to the processed set BEFORE reading/processing to prevent circular imports
    context.processed_files.insert(canonical_path);

    // Read the import file content
    let import_content = match std::fs::read_to_string(&import_file_path) {
        Ok(content) => content,
        Err(e) => {
            console::err(&format!(
                "Failed to read import file `{}`: {}",
                import_file_path.display(),
                e
            ));
            *context.has_errors = true;
            return None;
        }
    };

    // Recursively process imports in the imported file
    Some(process_imports_recursively(&import_content, context))
}

fn process_imports_recursively(content: &str, context: &mut ImportContext) -> String {
    let mut processed_content = content.to_string();

    // Find all import statements
    let imports = extract_imports(content);

    for import_path in imports {
        if let Some(processed_import_content) = process_single_import(&import_path, context) {
            // Replace the import statement with the processed content of the imported file
            let import_statement = format!("use {};", import_path);
            processed_content =
                processed_content.replace(&import_statement, &processed_import_content);
        }
    }

    processed_content
}

/// Main build function that orchestrates the entire build process
fn build(proj_path: &str) -> Result<(), String> {
    let project_json = read_project_json(proj_path)?;

    let project_path = Path::new(proj_path)
        .parent()
        .ok_or("Failed to get parent directory of project.json")?;

    console::info(&format!(
        "Building project: `{}` from `{}`",
        project_json.name, proj_path
    ));
    console::info(&format!("Version: `{}`", project_json.version));
    console::info(&format!("Entry point: `{}`", project_json.entry));

    if let Some(ref lib) = project_json.lib {
        console::info(&format!("Library: `{}`", lib));
    } else {
        console::info("No library specified.");
    }

    // Process the Entry File
    let entry_path = project_path.join(&project_json.entry);
    if !entry_path.exists() {
        return Err(format!(
            "Entry file `{}` does not exist",
            entry_path.display()
        ));
    }

    let entry_content = std::fs::read_to_string(&entry_path)
        .map_err(|e| format!("Failed to read entry file: {}", e))?;

    // Process all imports recursively
    let mut processed_files = HashSet::new();
    let mut has_errors = false;
    let mut context = ImportContext {
        project_path,
        lib_dir: project_json.lib.as_deref(),
        processed_files: &mut processed_files,
        has_errors: &mut has_errors,
    };

    let processed_content = process_imports_recursively(&entry_content, &mut context);

    // Check for errors and cancel build if any were found
    if has_errors {
        return Err("Build failed due to errors. Aborting build process.".to_string());
    }

    // Create output directory structure: {project root}/build/{version}/
    let build_dir = project_path.join("build").join(&project_json.version);
    std::fs::create_dir_all(&build_dir).map_err(|e| {
        format!(
            "Failed to create build directory `{}`: {}",
            build_dir.display(),
            e
        )
    })?;

    // Create output file: {project_name}.gpc
    let output_file = build_dir.join(format!("{}.gpc", project_json.name));

    // Write processed content to output file (overwrite if exists)
    std::fs::write(&output_file, &processed_content).map_err(|e| {
        format!(
            "Failed to write output file `{}`: {}",
            output_file.display(),
            e
        )
    })?;

    console::success(&format!(
        "Build completed successfully: {}",
        output_file.display()
    ));
    console::info(&format!(
        "Processed content length: {} characters",
        processed_content.len()
    ));

    Ok(())
}

/// Public interface for the build functionality
pub fn run_build(proj_path: &str) -> Result<(), String> {
    build(proj_path)
}
