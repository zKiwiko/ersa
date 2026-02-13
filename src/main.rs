use clap::Parser;
pub mod cli;
pub mod log;
pub mod network;

#[derive(Parser)]
#[command(name = "ersa", version = env!("CARGO_PKG_VERSION"), about = "GPC/GPX Language Tooling")]
struct Cli {
    #[arg(long, global = true, help = "Enable verbose output")]
    verbose: bool,

    #[command(subcommand)]
    command: cli::Command,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        unsafe {
            std::env::set_var("ERSA_VERBOSE", "1");
        }
    }

    match cli::run(cli.command).await {
        Ok(_) => (),
        Err(e) => {
            log::error(&format!("{}", e));
            std::process::exit(1);
        }
    }
}
