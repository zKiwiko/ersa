mod build;
pub(crate) mod console;
mod pkg;
mod project;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "ersa", version, about = "GPC/GPX Package Manager & Utility.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Build(BuildArgs),
    Pkg(PkgArgs),
    Project(ProjectArgs),
    Debug(DebugArgs),
}

#[derive(Args, Debug)]
struct BuildArgs {
    #[arg(long, short, group = "input", num_args = 1, value_name = "FILEPATH")]
    file: Option<String>,

    #[arg(long, short, group = "input", num_args = 1, value_name = "DIRECTORY")]
    dir: Option<String>,

    #[arg(long, short)]
    output: Option<String>,
}

#[derive(Args, Debug)]
struct PkgArgs {
    #[arg(long, short, group = "input", num_args = 1, value_name = "PACKAGE")]
    install: Option<String>,

    #[arg(long, short, group = "input", num_args = 1, value_name = "PACKAGE")]
    update: Option<String>,

    #[arg(long, short, group = "input", num_args = 0..=1, value_name = "PACKAGE?")]
    list: Option<Option<String>>,

    #[arg(long, short, group = "input", num_args = 1, value_name = "PACKAGE")]
    remove: Option<String>,
}

#[derive(Args, Debug)]
struct ProjectArgs {
    #[arg(long, short, num_args = 1, value_name = "NAME", requires = "language")]
    create: Option<String>,

    #[arg(long, short, num_args = 1, value_name = "LANGUAGE")]
    language: Option<String>,

    #[arg(long, short, group = "input", num_args = 1, value_name = "DIRECTORY")]
    output: Option<String>,
}

#[derive(Args, Debug)]
struct DebugArgs {
    #[arg(long)]
    dirs: bool,
}

pub async fn run() -> Result<(), String> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build(args) => {
            if let Some(file) = &args.file {
                console::log(&format!("Building file: {}", file));
            } else if let Some(dir) = &args.dir {
                console::log(&format!("Building directory: {}", dir));
            } else {
                console::err("No input file or directory specified.")
            }

            if let Some(output) = &args.output {
                console::log(&format!("Output will be saved to: {}", output));
            }
            Ok(())
        }
        Commands::Pkg(args) => {
            if let Some(url) = &args.install {
                if url == "core" {
                    console::log("Installing Core libraries");
                    pkg::download(&"https://github.com/zKiwiko/gpx-stdlib.git").await?;
                } else {
                    pkg::download(&url).await?;
                }
            }
            if let Some(package) = &args.update {
                console::log(&format!("Updating package: {}", package));
                pkg::update(&package).await?;
            }
            if let Some(package) = &args.list {
                let _ = pkg::list(package);
            }

            if let Some(package) = &args.remove {
                console::log(&format!("Removing package: {}", package));
                let _ = pkg::remove(package);
            }
            Ok(())
        }
        Commands::Debug(args) => {
            if args.dirs {
                let app_dir = pkg::git::get_app_directory()?;
                let lib_dir = app_dir.join("bin").join("lib");
                console::info(&format!("Install directory: {}", app_dir.display()));
                console::info(&format!("Package directory: {}", lib_dir.display()));
                console::info(&format!(
                    "Working directory: {}",
                    std::env::current_dir().unwrap().display()
                ));
            }
            Ok(())
        }
        Commands::Project(args) => {
            if let Some(name) = &args.create {
                if let Some(lang) = &args.language {
                    if let Some(output) = &args.output {
                        project::create(name, lang, Some(output))?;
                    } else {
                        project::create(name, lang, None)?;
                    }
                }
            }

            Ok(())
        }
    }
}
