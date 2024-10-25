use std::process::Command;
use clap::Args;
use crate::{constants::{App, Target}, run_command};

#[derive(Debug, Args)]
pub struct BuildArgs {
    #[arg(default_value_t, value_enum)]
    platform: App,
    #[arg(default_value_t, value_enum)]
    target: Target,
    #[clap(long, short, action)]
    artifact: bool,
}

impl BuildArgs {
    pub fn run(
        &self,
        workspace_path: String
    ) -> color_eyre::Result<()> {
        let mut command = Command::new("cargo");
        let path = format!("{}/bin/{}", &workspace_path, self.target);
        let mut args: Vec<String> = vec![
            "build".to_string(),
            "--release".to_string()
        ];

        if self.artifact {
            let target = self.target.clone();
            let platform = self.platform.clone();
            let build_path = format!("--target-dir={}/dist/{}/{}", &workspace_path, &target, &platform);
            args.push(build_path);
        }

        match self.platform {
            App::Desktop => {
                command.args(&args).current_dir(&path);
                run_command(command)?;
            },
            App::Wasm => {
                args.push("--target".to_string());
                args.push("wasm32-unknown-unknown".to_string());
                command.args(&args).current_dir(&path);
                run_command(command)?;
            },
        }
        Ok(())
    }
}
