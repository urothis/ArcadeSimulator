use std::process::Command;

use clap::{Args, ValueEnum};

use crate::{constants::App, run_command};

#[derive(Debug, Args)]
pub struct RunArgs {
    #[arg(default_value_t, value_enum)]
    platform: App,
}

impl RunArgs {
	pub fn run(
		&self,
		workspace_path: String
	) -> color_eyre::Result<()> {
		match self.platform {
			App::Desktop => {
				let mut command = Command::new("cargo");
				let path = format!("{}/bin/desktop", &workspace_path);
				command.args(["run", "--release"]).current_dir(&path);
				run_command(command)?;
			},
			App::Wasm => {
				let mut command = Command::new("trunk");
				let path = format!("{}/bin/wasm", &workspace_path);
				command.args(["serve", "--release"]).current_dir(&path);
				run_command(command)?;
			},
		}

		Ok(())
	}
}
