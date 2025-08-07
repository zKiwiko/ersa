use crate::cli::console;
use crate::cli::pkg::git::get_app_directory;
use serde_json::json;
use std::fs;
use std::path::PathBuf;

/// Supported project languages
#[derive(Debug)]
enum ProjectLanguage {
    Gpc,
    Gpx,
}

impl ProjectLanguage {
    fn from_str(language: &str) -> Result<Self, String> {
        match language.to_lowercase().as_str() {
            "gpc" => Ok(ProjectLanguage::Gpc),
            "gpx" => Ok(ProjectLanguage::Gpx),
            _ => Err(format!("Unsupported language: {}", language)),
        }
    }

    fn get_entry_file(&self) -> &'static str {
        match self {
            ProjectLanguage::Gpc => "src/main.gpc",
            ProjectLanguage::Gpx => "src/main.gpx",
        }
    }
}

/// Project creation configuration
struct ProjectConfig {
    name: String,
    language: ProjectLanguage,
    project_path: PathBuf,
    app_lib_directory: PathBuf,
}

impl ProjectConfig {
    fn new(name: &str, language: &str, output: Option<&str>) -> Result<Self, String> {
        let language = ProjectLanguage::from_str(language)?;
        let project_path = Self::resolve_project_path(name, output)?;
        let app_lib_directory = Self::get_app_lib_directory()?;

        Ok(ProjectConfig {
            name: name.to_string(),
            language,
            project_path,
            app_lib_directory,
        })
    }

    fn resolve_project_path(name: &str, output: Option<&str>) -> Result<PathBuf, String> {
        let base_dir = match output {
            Some(dir) => PathBuf::from(dir),
            None => std::env::current_dir()
                .map_err(|e| format!("Failed to get current directory: {}", e))?,
        };

        let project_path = base_dir.join(name);

        if project_path.exists() {
            return Err(format!(
                "Project '{}' already exists in {}",
                name,
                base_dir.display()
            ));
        }

        Ok(project_path)
    }

    fn get_app_lib_directory() -> Result<PathBuf, String> {
        let app_dir = get_app_directory()?;
        Ok(app_dir.join("bin").join("lib"))
    }
}

/// Creates a new project with the specified configuration
pub fn new(name: &str, language: &str, output: Option<&str>) -> Result<(), String> {
    let config = ProjectConfig::new(name, language, output)?;

    // Create project directory structure
    create_project_structure(&config)?;

    // Create meta.json configuration file
    create_meta_json(&config)?;

    // Create source files
    create_source_files(&config)?;

    console::success(&format!(
        "Project '{}' created successfully at {}",
        config.name,
        config.project_path.display()
    ));

    Ok(())
}

/// Creates the basic project directory structure
fn create_project_structure(config: &ProjectConfig) -> Result<(), String> {
    // Create main project directory
    fs::create_dir_all(&config.project_path)
        .map_err(|e| format!("Failed to create project directory: {}", e))?;

    // Create src directory
    let src_dir = config.project_path.join("src");
    fs::create_dir_all(&src_dir).map_err(|e| format!("Failed to create src directory: {}", e))?;

    Ok(())
}

/// Creates the meta.json configuration file
fn create_meta_json(config: &ProjectConfig) -> Result<(), String> {
    let meta_json = json!({
        "name": config.name,
        "version": "1.0.0",
        "entry": config.language.get_entry_file(),
        "lib": config.app_lib_directory.to_string_lossy()
    });

    let meta_path = config.project_path.join("meta.json");
    let meta_content = serde_json::to_string_pretty(&meta_json)
        .map_err(|e| format!("Failed to serialize meta.json: {}", e))?;

    fs::write(&meta_path, meta_content)
        .map_err(|e| format!("Failed to create meta.json: {}", e))?;

    Ok(())
}

/// Creates the initial source files
fn create_source_files(config: &ProjectConfig) -> Result<(), String> {
    let entry_file_path = config.project_path.join(config.language.get_entry_file());

    // Create an empty entry file
    fs::write(&entry_file_path, "")
        .map_err(|e| format!("Failed to create {}: {}", entry_file_path.display(), e))?;

    Ok(())
}
