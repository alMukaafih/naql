use crate::join;
use anyhow::{Result, anyhow};
use std::{env::current_dir, path::PathBuf};
use which::which;

pub fn find_binary(binary_name: &str) -> Result<PathBuf> {
    match which(binary_name) {
        Ok(v) => return Ok(v),
        Err(_) => {}
    }

    let pwd = current_dir()?;
    for p in pwd.ancestors() {
        match which(join!(p, "node_modules", ".bin", binary_name)) {
            Ok(v) => return Ok(v),
            Err(_) => {}
        }
    }

    Err(anyhow!("{binary_name}: command not found"))
}
