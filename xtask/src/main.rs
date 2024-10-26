/// This is the main entry point for the `xtask` binary. It is used to define custom tasks for the workspace
/// and can be invoked using `cargo xtask <task-name>`.
pub mod cli;
pub mod constants;
pub mod util;

use std::{ffi::OsStr, process::Command};

use clap::Parser;
use cli::Cli;
use color_eyre::{eyre::bail, Result};
use env_logger::Env;

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    color_eyre::install()?;
    let cli = Cli::parse();
    cli.run()?;
    Ok(())
}

fn run_command(mut command: Command) -> Result<()> {
    println!(
        "Running {} {}",
        command.get_program().to_string_lossy(),
        command
            .get_args()
            .map(OsStr::to_string_lossy)
            .collect::<Vec<_>>()
            .join(" ")
    );

    let exit_code = command.spawn()?.wait()?;
    if !exit_code.success() {
        bail!("Command failed!");
    }

    Ok(())
}
