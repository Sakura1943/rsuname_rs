#[path = "./src/cli.rs"]
mod cli;
use std::{fs::create_dir_all, path::Path};

use clap_complete::{
    generate_to,
    shells::{Bash, Fish, Zsh},
};
use clap_complete_nushell::Nushell;
use cli::Cli;

fn main() -> Result<(), std::io::Error> {
    let cmd = &mut Cli::cmd();
    let bin_name = "rsuname";
    let out_dir = "completions";

    if !Path::new(out_dir).exists() {
        create_dir_all(out_dir)?;
    }

    generate_to(Bash, cmd, bin_name, out_dir)?;
    generate_to(Fish, cmd, bin_name, out_dir)?;
    generate_to(Zsh, cmd, bin_name, out_dir)?;
    generate_to(Nushell, cmd, bin_name, out_dir)?;

    Ok(())
}
