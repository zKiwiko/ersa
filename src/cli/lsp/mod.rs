use clap::Args;

mod install;
mod update;

#[derive(Args, Debug)]
pub struct LspArgs {
    /// Install the LSP server
    #[arg(long)]
    pub install: bool,

    /// Update the LSP server to the latest version
    #[arg(long)]
    pub update: bool,

    /// Check if an update is available
    #[arg(long)]
    pub check_update: bool,
}

pub async fn run(args: LspArgs) -> Result<(), String> {
    if args.install {
        install::install().await
    } else if args.update {
        update::update().await
    } else if args.check_update {
        update::check_update().await.map(|_| ())
    } else {
        Err(
            "No valid LSP command provided. Use --install, --update, or --check-update."
                .to_string(),
        )
    }
}
