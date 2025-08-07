mod build;
pub(crate) mod console;
mod pkg;
mod project;

use clap::{Args, Parser, Subcommand};

/// Main CLI application structure
#[derive(Parser, Debug)]
#[command(name = "ersa", version, about = "GPC/GPX Package Manager & Utility.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Available commands for the CLI
#[derive(Subcommand, Debug)]
enum Commands {
    /// Build GPC/GPX projects
    Build(BuildArgs),
    /// Package management operations
    Pkg(PkgArgs),
    /// Project creation and management
    Project(ProjectArgs),
    /// Debug and diagnostic commands
    Debug(DebugArgs),
}

/// Build command arguments
#[derive(Args, Debug)]
struct BuildArgs {
    /// Path to the project meta.json file
    #[arg(
        long,
        short,
        group = "input",
        num_args = 1,
        value_name = "JSON FILEPATH"
    )]
    path: Option<String>,

    /// Build GPC Project
    #[arg(long, help = "Build GPC Project.")]
    gpc: bool,

    /// Build GPX Project
    #[arg(long, help = "Build GPX Project.")]
    gpx: bool,
}

/// Package management command arguments
#[derive(Args, Debug)]
struct PkgArgs {
    /// Install a package from URL or name
    #[arg(long, short, group = "input", num_args = 1, value_name = "PACKAGE")]
    install: Option<String>,

    /// Update an existing package
    #[arg(long, short, group = "input", num_args = 1, value_name = "PACKAGE")]
    update: Option<String>,

    /// List installed packages (optionally filter by name)
    #[arg(long, short, group = "input", num_args = 0..=1, value_name = "PACKAGE?")]
    list: Option<Option<String>>,

    /// Remove an installed package
    #[arg(long, short, group = "input", num_args = 1, value_name = "PACKAGE")]
    remove: Option<String>,
}

/// Project management command arguments  
#[derive(Args, Debug)]
struct ProjectArgs {
    /// Create a new project with the specified name
    #[arg(long, short, num_args = 1, value_name = "NAME", requires = "language")]
    create: Option<String>,

    /// Programming language for the project (gpc/gpx)
    #[arg(long, short, num_args = 1, value_name = "LANGUAGE")]
    language: Option<String>,

    /// Output directory for the project
    #[arg(long, short, group = "input", num_args = 1, value_name = "DIRECTORY")]
    output: Option<String>,
}

/// Debug command arguments
#[derive(Args, Debug)]
struct DebugArgs {
    /// Show application directories
    #[arg(long)]
    dirs: bool,
}

/// Main entry point for CLI command processing
pub async fn run() -> Result<(), String> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build(args) => handle_build_command(args).await,
        Commands::Pkg(args) => handle_pkg_command(args).await,
        Commands::Project(args) => handle_project_command(args).await,
        Commands::Debug(args) => handle_debug_command(args).await,
    }
}

/// Handle build-related commands
async fn handle_build_command(args: &BuildArgs) -> Result<(), String> {
    let path = args.path.as_ref().ok_or_else(|| {
        "No project file specified. Use --path to specify a project file.".to_string()
    })?;

    if !std::path::Path::new(path).exists() {
        return Err(format!("Project file `{}` does not exist", path));
    }

    match (args.gpc, args.gpx) {
        (true, false) => {
            console::info("Building GPC project...");
            build::gpc_build(path);
        }
        (false, true) => {
            console::info("GPX build functionality not yet implemented.");
            // Placeholder for GPX build logic
        }
        (false, false) => {
            return Err("No build type specified. Use --gpc or --gpx.".to_string());
        }
        (true, true) => {
            return Err("Cannot specify both --gpc and --gpx. Choose one build type.".to_string());
        }
    }

    Ok(())
}

/// Handle package management commands
async fn handle_pkg_command(args: &PkgArgs) -> Result<(), String> {
    // Handle install command
    if let Some(url) = &args.install {
        return handle_pkg_install(url).await;
    }

    // Handle update command
    if let Some(package) = &args.update {
        console::log(&format!("Updating package: {}", package));
        return pkg::update(package).await;
    }

    // Handle list command
    if let Some(package) = &args.list {
        return pkg::list(package).map_err(|e| format!("Failed to list packages: {}", e));
    }

    // Handle remove command
    if let Some(package) = &args.remove {
        console::log(&format!("Removing package: {}", package));
        return pkg::remove(package).map_err(|e| format!("Failed to remove package: {}", e));
    }

    Err("No package operation specified. Use --install, --update, --list, or --remove.".to_string())
}

/// Handle package installation with special case for "core"
async fn handle_pkg_install(package_identifier: &str) -> Result<(), String> {
    match package_identifier {
        "core" => {
            console::info("Installing Core libraries");
            pkg::download("https://github.com/zKiwiko/gpx-stdlib.git").await
        }
        url => {
            console::log(&format!("Installing package: {}", url));
            pkg::download(url).await
        }
    }
}

/// Handle project management commands
async fn handle_project_command(args: &ProjectArgs) -> Result<(), String> {
    if let Some(name) = &args.create {
        let language = args.language.as_ref().ok_or_else(|| {
            "Language is required when creating a project. Use --language.".to_string()
        })?;

        console::log(&format!("Creating {} project: {}", language, name));

        match &args.output {
            Some(output) => project::create(name, language, Some(output)),
            None => project::create(name, language, None),
        }
    } else {
        Err("No project operation specified. Use --create to create a new project.".to_string())
    }
}

/// Handle debug and diagnostic commands
async fn handle_debug_command(args: &DebugArgs) -> Result<(), String> {
    if args.dirs {
        display_debug_directories()?;
    } else {
        console::info("Available debug options: --dirs");
    }

    Ok(())
}

/// Display application directories for debugging
fn display_debug_directories() -> Result<(), String> {
    let app_dir = pkg::git::get_app_directory()?;
    let lib_dir = app_dir.join("bin").join("lib");

    let current_dir =
        std::env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;

    console::info(&format!("Install directory: {}", app_dir.display()));
    console::info(&format!("Package directory: {}", lib_dir.display()));
    console::info(&format!("Working directory: {}", current_dir.display()));

    Ok(())
}
