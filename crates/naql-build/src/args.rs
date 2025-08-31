use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct BuildArgs {
    /// Path to .vsix or directory
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Output file
    #[arg(long, default_value = "dist.zip")]
    pub outfile: PathBuf,

    /// Output directory
    #[arg(long, default_value = "./")]
    pub outdir: PathBuf,

    /// Path to Acode manifest
    #[arg(long)]
    pub manifest: Option<PathBuf>,
}
