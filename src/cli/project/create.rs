pub fn new(name: &str, language: &str, output: Option<&str>) -> Result<(), String> {
    let env = std::env::current_dir().unwrap();
    let output_dir = output.unwrap_or(env.to_str().unwrap());
    let project_path = format!("{}/{}", output_dir, name);

    if std::path::Path::new(&project_path).exists() {
        return Err(format!(
            "Project '{}' already exists in {}",
            name, output_dir.replace("\\", "/")
        ));
    }

    std::fs::create_dir_all(&project_path)
        .map_err(|e| format!("Failed to create project directory: {}", e))?;

    match language.to_lowercase().as_str() {
        "gpc" => {
            std::fs::write(format!("{}/meta.json", project_path), format!("{} \n    \"name\": \"{}\",\n    \"version\": \"1.0.0\",\n    \"entry\": \"src/main.gpc\"\n {}", '{', name, '}')).map_err(|e| format!("Failed to create meta.json: {}", e))?;
            std::fs::create_dir_all(format!("{}/src", project_path))
                .map_err(|e| format!("Failed to create src directory: {}", e))?;
            std::fs::write(format!("{}/src/main.gpc", project_path), "")
                .map_err(|e| format!("Failed to create main.gpc: {}", e))?;
        }
        "gpx" => {
            std::fs::write(
                format!("{}/meta.json", project_path),
                format!(
                    "{} \n    \"name\": \"{}\",\n    \"version\": \"1.0.0\",\n     \"entry\" \n{}",
                    '{', name, '}'
                ),
            )
            .map_err(|e| format!("Failed to create meta.json: {}", e))?;
            std::fs::create_dir_all(format!("{}/src", project_path))
                .map_err(|e| format!("Failed to create src directory: {}", e))?;
            std::fs::write(format!("{}/src/main.gpx", project_path), "")
                .map_err(|e| format!("Failed to create main.gpx: {}", e))?;
        }
        _ => return Err(format!("Unsupported language: {}", language)),
    }

    Ok(())
}
