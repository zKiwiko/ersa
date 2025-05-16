mod build;
pub(crate) mod console;
mod pkg;

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
    Debug(DebugArgs),
}

#[derive(Args, Debug)]
struct BuildArgs {
    #[arg(long, short, group = "input")]
    language: Option<String>,

    #[arg(long, short, group = "input")]
    file: Option<String>,

    #[arg(long, short, group = "input")]
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

    #[arg(long, short, group = "input", num_args = 0..=1, value_name = "PACKAGE")]
    list: Option<String>,

    #[arg(long, short, group = "input", num_args = 1, value_name = "PACKAGE")]
    remove: Option<String>,

    #[arg(long, short)]
    sync: bool,
}

#[derive(Args, Debug)]
struct DebugArgs {
    #[arg(long)]
    env_dir: Option<bool>,
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
                    pkg::download("https://github.com/zKiwiko/gpx-stdlib.git").await?;
                }
                pkg::download(&url).await?;
            }
            if let Some(package) = &args.update {
                console::log(&format!("Updating package: {}", package));
                pkg::update(&package).await?;
            }
            if let Some(package) = &args.list {
                let _ = pkg::list(&package);
            } else {
                let _ = pkg::list("");
            }
            Ok(())
        }
        Commands::Debug(args) => {
            if let Some(env_dir) = &args.env_dir {
                console::log(&format!("Environment directory: {}", env_dir));
            } else {
                console::err("No debug action specified.")
            }
            Ok(())
        }
    }
}
