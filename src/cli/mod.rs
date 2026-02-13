use clap::Subcommand;

pub mod build;
pub mod lsp;

#[derive(Subcommand, Debug)]
pub enum Command {
    Lsp(self::lsp::LspArgs),
    Build(self::build::BuildArgs),
}

pub async fn run(command: Command) -> Result<(), String> {
    match command {
        Command::Lsp(args) => self::lsp::run(args).await,
        Command::Build(args) => self::build::run(args).await,
    }
}
