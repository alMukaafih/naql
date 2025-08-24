use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct BuildArgs {
    #[arg(default_value = ".")]
    pub path: PathBuf,

    #[arg(long)]
    pub manifest: Option<PathBuf>,
}
