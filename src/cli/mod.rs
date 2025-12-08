use clap::Subcommand;

pub mod add;
pub mod new;

#[derive(Subcommand, Debug)]
pub enum Command {
    New(self::new::NewArgs),
    Add(self::add::AddArgs),
}

pub fn run(command: Command) -> Result<(), String> {
    match command {
        Command::New(args) => self::new::run(args),
        Command::Add(args) => self::add::run(args),
    }
}
