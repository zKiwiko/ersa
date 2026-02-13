use clap::Args;
use std::fs;
use std::path::PathBuf;

pub mod preprocess;

#[derive(Args, Debug)]
pub struct BuildArgs {
    #[arg(long, short = 'f')]
    file: Option<String>,

    #[arg(long, short = 'o')]
    output: Option<String>,
}

pub async fn run(args: BuildArgs) -> Result<(), String> {
    // Determine input file
    let input_path = if let Some(file) = args.file {
        PathBuf::from(file)
    } else {
        // Default to main.gpc in current directory
        let cwd = std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?;
        cwd.join("main.gpc")
    };

    // Check if input file exists
    if !input_path.exists() {
        return Err(format!("Input file not found: {}", input_path.display()));
    }

    crate::log::info(&format!("Building file: {}", input_path.display()));

    // Read input file
    let code =
        fs::read_to_string(&input_path).map_err(|e| format!("Failed to read input file: {}", e))?;

    // Preprocess the code
    let base_path = input_path.parent().unwrap_or(std::path::Path::new("."));
    let preprocessed = preprocess::preprocess(&code, base_path)?;

    // Determine output path
    let output_path = if let Some(output) = args.output {
        PathBuf::from(output)
    } else {
        // Default to pwd/build/build.gpc
        let cwd = std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?;
        cwd.join("build").join("build.gpc")
    };

    // Create output directory if it doesn't exist
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
    }

    // Write output
    fs::write(&output_path, preprocessed)
        .map_err(|e| format!("Failed to write output file: {}", e))?;

    crate::log::success(&format!("Build complete: {}", output_path.display()));

    Ok(())
}
