use clap::Args;

#[derive(Debug, Args)]
pub struct TranslateArgs {
    /// AWS Profile to use
    #[arg(long, default_value = "default")]
    aws_profile: String,

    /// AWS Region to use
    #[arg(long, default_value = "us-east-1")]
    aws_region: String,

    /// The path to the RON file to use as the base locale
    #[arg(long, default_value = "./assets/locales/base.ftl.ron")]
    ron_file: std::path::PathBuf,
}

impl TranslateArgs {
    pub fn run(&self, _workspace_path: String) -> color_eyre::Result<()> {
        Ok(())
    }
}
