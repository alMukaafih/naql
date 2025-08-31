use anyhow::Result;
use naql_shared::join;
use naql_shared::node::find_binary;
use std::env::current_dir;
use std::fs::{create_dir_all, remove_dir_all};
use std::path::{Path, PathBuf};
use std::process::Command;
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
    let contrib = join!(&path, "src", name);

    debug!("mkdir -p {}", contrib.to_string_lossy());
    create_dir_all(&contrib)?;

    let assets = join!(path, "dist", "assets");
    debug!("mkdir -p {}", assets.to_string_lossy());
    create_dir_all(assets)?;
    Ok(contrib)
}

pub fn esbuild(build_dir: &Path) -> anyhow::Result<()> {
    Command::new(find_binary("esbuild")?)
        .args([
            "src/main.js",
            "--bundle",
            "--minify",
            "--outfile=dist/main.js",
        ])
        .current_dir(build_dir)
        .output()?;

    Ok(())
}
