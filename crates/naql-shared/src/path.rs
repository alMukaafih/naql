use std::path::{Path, PathBuf};

pub fn join<P: AsRef<Path>>(paths: &[P]) -> PathBuf {
    let mut path = PathBuf::from(paths[0].as_ref());
    for i in 1..paths.len() {
        path.push(paths[i].as_ref());
    }

    path
}
