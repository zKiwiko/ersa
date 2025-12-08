use clap::Parser;
mod cli;
mod log;

#[cfg(target_os = "windows")]
pub const ERSA_USER_DIR: &str = concat!(env!("APPDATA"), "\\ersa");
#[cfg(not(target_os = "windows"))]
pub const ERSA_USER_DIR: &str = concat!(env!("HOME"), "/.local/share/ersa");

#[derive(Parser)]
#[command(name = "ersa", version = env!("CARGO_PKG_VERSION"), about = "GPC/GPX Language Tooling")]
struct Cli {
    #[arg(long, global = true, help = "Enable verbose output")]
    verbose: bool,

    #[command(subcommand)]
    command: cli::Command,
}

fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        unsafe {
            std::env::set_var("ERSA_VERBOSE", "1");
        }
    }

    match cli::run(cli.command) {
        Ok(_) => (),
        Err(e) => {
            log::error(&format!("{}", e));
            std::process::exit(1);
        }
    }
}
