pub mod build;
pub mod deploy;
pub mod run;
pub mod test;
pub mod translate;

use std::{env, path::PathBuf, str::FromStr};

use clap::Parser;
use color_eyre::Result;

use build::BuildArgs;
use deploy::DeployArgs;
use run::RunArgs;
use test::TestingArgs;
use translate::TranslateArgs;

#[derive(Debug, Parser)]
#[command(about)]
pub enum Cli {
	#[command(about = "Build", aliases = &["b"])]
	Build(BuildArgs),
	#[command(about = "Deploy", aliases = &["d"])]
	Deploy(DeployArgs),
	#[command(about = "Run", aliases = &["r"])]
	Run(RunArgs),
	#[command(about = "Testing", arg_required_else_help = true, aliases = &["t"])]
	Test(TestingArgs),
	#[command(about = "Translate Fluent assets", arg_required_else_help = true, aliases = &["tr"])]
	Translate(TranslateArgs),
}

impl Cli {
	pub fn run(self) -> Result<()> {
		let workspace_path = PathBuf::from_str(&env::var("CARGO_WORKSPACE_DIR")?)?.display().to_string();
		match self {
			Self::Build(args) => args.run(workspace_path),
			Self::Deploy(args) => args.run(workspace_path),
			Self::Run(args) => args.run(workspace_path),
			Self::Test(args) => args.run(workspace_path),
			Self::Translate(args) => args.run(workspace_path),
		}
	}
}
