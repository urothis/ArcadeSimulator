use std::process::Command;

//use byte_unit::Byte;
use clap::{Args, ValueEnum};
//use color_eyre::eyre::eyre;
use log::info;

use crate::{constants::App, run_command};

#[derive(Debug, Args)]
pub struct DeployArgs {
    #[arg(default_value_t, value_enum)]
    platform: App,
}

impl DeployArgs {
	pub fn run(
		&self,
		workspace_path: String
	) -> color_eyre::Result<()> {
		match self.platform {
			App::Desktop => {
				let mut command = Command::new("cargo");
				let path = format!("{}/crates/bin/app", &workspace_path);
				command.args(["run", "--release"]).current_dir(&path);
				run_command(command)?;
			},
			App::Wasm => {
                let mut command = Command::new("trunk");
				let path = format!("{}/crates/bin/app", &workspace_path);
				command.args(["build", "--release", "--target", "wasm32-unknown-unknown"]).current_dir(&path);
				run_command(command)?;

                let mut command = Command::new("aws");
                command.args(["s3", "rm", "s3://neurogr.id", "--recursive", "--profile", "personal-root"]);
                run_command(command)?;

                let upload_start = std::time::Instant::now();
                let dist_path = format!("{}/crates/bin/app/dist", &workspace_path);
                let mut command = Command::new("aws");
                command.args(["s3", "cp", "--recursive", ".", "s3://neurogr.id", "--acl", "public-read", "--profile", "personal-root"]).current_dir(&dist_path);
                run_command(command)?;

                let mut command = Command::new("aws");
                command.args(["cloudfront", "create-invalidation", "--distribution-id", "E2A4JGPM4X4CJ4", "--paths", "/*", "--profile", "personal-root"]);
                run_command(command)?;
                info!("Update took: {:?}", upload_start.elapsed());

                info!("Deployed to: https://neurogr.id");
            },
		}

		Ok(())
	}
}
