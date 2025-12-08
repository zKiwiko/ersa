use clap::Args;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Args, Debug)]
pub struct AddArgs {
    // Name of a package to add
    // Can either a name in the pkg repo
    // or a git repo with the --git flag
    pub what: String,

    #[arg(long, default_value_t = false)]
    pub git: bool,
}

#[derive(Serialize, Deserialize)]
struct ProjectConfig {
    name: String,
    kind: String,
    version: String,
    dependencies: HashMap<String, String>,
}

pub fn run(args: AddArgs) -> Result<(), String> {
    crate::log::info(&format!("Adding package '{}'", args.what));

    if args.git {
        crate::log::vinfo(&format!("Adding package from git repository"));
        // Logic to add package from git
    } else {
        crate::log::vinfo(&format!("Adding package from package repository"));
        // Logic to add package from pkg repo
    }

    crate::log::success(&format!("Package '{}' added successfully!", args.what));

    Ok(())
}
