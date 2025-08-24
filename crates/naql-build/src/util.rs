use anyhow::Result;
use naql_shared::join;
use std::env::current_dir;
use std::fs::{create_dir_all, remove_dir_all};
use std::path::{Path, PathBuf};
use tracing::debug;

pub fn build_dir(id: &str) -> Result<PathBuf> {
    let pwd = current_dir()?;
    let build = join!(pwd, ".naql", id);
    if build.exists() {
        debug!("rm -rf {}", build.to_string_lossy());
        remove_dir_all(&build)?;
    }

    debug!("mkdir -p {}", build.to_string_lossy());
    create_dir_all(&build)?;
    Ok(build)
}

pub fn contrib_dir<P: AsRef<Path>>(path: P, name: &str) -> Result<PathBuf> {
    let contrib = join!(path, "src", name);

    debug!("mkdir -p {}", contrib.to_string_lossy());
    create_dir_all(&contrib)?;

    let assets = join!(&contrib, "assets");
    debug!("mkdir -p {}", assets.to_string_lossy());
    create_dir_all(assets)?;
    Ok(contrib)
}
