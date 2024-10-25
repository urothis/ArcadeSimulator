use clap::Args;

#[derive(Debug, Args)]
pub struct TestingArgs {
}

impl TestingArgs {
	pub fn run(
		&self,
		_workspace_path: String
	) -> color_eyre::Result<()> {
		Ok(())
	}
}
