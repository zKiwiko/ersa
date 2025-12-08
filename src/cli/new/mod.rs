/*

'new' module holds data and impl for the 'new' subcommand of the CLI.
Creation of Script projects (ersa format) and Ersa Library project templates.

usage:
    ersa new [OPTIONS] <NAME>

Options:
    -h, --help       Print help information
    -l, --lib        Create a library project
    -b, --bin        Create a binary project (default)

*/

use clap::Args;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Args, Debug)]
pub struct NewArgs {
    /// Name of the new project
    pub name: String,

    /// Create a library project
    #[arg(short, long, group = "kind", conflicts_with = "bin")]
    pub lib: bool,

    /// Create a binary project (default)
    #[arg(short, long, group = "kind", conflicts_with = "lib")]
    pub bin: bool,
}

#[derive(Serialize)]
struct ProjectConfig {
    name: String,
    kind: String,
    version: String,
    dependencies: HashMap<String, String>,
}

pub fn run(args: NewArgs) -> Result<(), String> {
    crate::log::info(&format!("Creating new project '{}'", args.name));

    let project_type = if args.lib { "library" } else { "binary" };

    crate::log::vinfo(&format!("Project type: {}", project_type));

    // Create project directory
    fs::create_dir(&args.name)
        .map_err(|e| format!("Failed to create project directory '{}': {}", args.name, e))?;

    match project_type {
        "library" => {
            create_library(&args.name)?;
        }
        "binary" => {
            create_binary(&args.name)?;
        }
        _ => return Err("Invalid project type".to_string()),
    }

    crate::log::success(&format!("Project '{}' created successfully!", args.name));

    Ok(())
}

// Structure
// src/
//  main.gpc
// ersa.toml

fn create_binary(project_name: &str) -> Result<(), String> {
    let src_dir = format!("{}/src", project_name);

    crate::log::vinfo(&format!("Creating source directory."));

    fs::create_dir(&src_dir).map_err(|e| {
        format!(
            "Failed to create src directory for project '{}': {}",
            project_name, e
        )
    })?;

    crate::log::vinfo(&format!("Creating main.gpc file."));

    let main_data = "// Main entry point\nmain {\n    // Hello World!\n}\n";

    fs::write(format!("{}/src/main.gpc", project_name), main_data).map_err(|e| {
        format!(
            "Failed to create main.gpc for project '{}': {}",
            project_name, e
        )
    })?;

    crate::log::vinfo(&format!("String Length: {}", main_data.len()));

    crate::log::vsuccess(&format!("Created main.gpc file."));

    crate::log::vinfo(&format!("Creating 'ersa.json' file."));

    let config_json = serde_json::to_string_pretty(&ProjectConfig {
        name: project_name.to_string(),
        kind: "binary".to_string(),
        version: "0.1.0".to_string(),
        dependencies: HashMap::new(),
    })
    .map_err(|e| format!("Failed to serialize project config: {}", e))?;

    fs::write(format!("{}/ersa.json", project_name), &config_json).map_err(|e| {
        format!(
            "Failed to write ersa.json for project '{}': {}",
            project_name, e
        )
    })?;

    crate::log::vinfo(&format!("String Length: {}", config_json.len()));

    crate::log::vsuccess(&format!("Created 'ersa.json' file."));

    Ok(())
}

fn create_library(project_name: &str) -> Result<(), String> {
    let src_dir = format!("{}/src", project_name);

    crate::log::vinfo(&format!("Creating source directory."));

    fs::create_dir(&src_dir).map_err(|e| {
        format!(
            "Failed to create src directory for project '{}': {}",
            project_name, e
        )
    })?;

    crate::log::vinfo(&format!("Creating lib.gpc file."));

    let main_data = "";

    fs::write(format!("{}/src/lib.gpc", project_name), main_data).map_err(|e| {
        format!(
            "Failed to create lib.gpc for project '{}': {}",
            project_name, e
        )
    })?;

    crate::log::vinfo(&format!("String Length: {}", main_data.len()));

    crate::log::vsuccess(&format!("Created lib.gpc file."));

    crate::log::vinfo(&format!("Creating 'ersa.json' file."));

    let config_json = serde_json::to_string_pretty(&ProjectConfig {
        name: project_name.to_string(),
        kind: "library".to_string(),
        version: "0.1.0".to_string(),
        dependencies: HashMap::new(),
    })
    .map_err(|e| format!("Failed to serialize project config: {}", e))?;

    fs::write(format!("{}/ersa.json", project_name), &config_json).map_err(|e| {
        format!(
            "Failed to write ersa.json for project '{}': {}",
            project_name, e
        )
    })?;

    crate::log::vinfo(&format!("String Length: {}", config_json.len()));

    crate::log::vsuccess(&format!("Created 'ersa.json' file."));

    Ok(())
}
