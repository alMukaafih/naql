use super::StyleRule;
use anyhow::{Result, bail};
use cached::Cached;
use cached::proc_macro::cached;
use naql_shared::{join, ok};
use radix_fmt::radix_36;
use rayon::prelude::*;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fmt::Display;
use std::fs::copy;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use tracing::{debug, error};

pub struct StyleSheet {
    rules: HashMap<String, Mutex<StyleRule>>,
    src: PathBuf,
    dest: PathBuf,
    count: Arc<AtomicUsize>,
}

impl StyleSheet {
    pub fn new(src: PathBuf, dest: PathBuf) -> Self {
        Self {
            rules: HashMap::with_capacity(1024),
            src,
            dest,
            count: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn insert(&mut self, k: String, v: StyleRule) {
        if let Some(rule) = self.rules.get_mut(&k) {
            ok!(rule.lock()).selectors.push(v.selectors[0].clone());
        } else {
            self.rules.insert(k, Mutex::new(v));
        }
    }

    pub fn resolve_urls(&mut self) -> Result<()> {
        self.rules
            .par_iter()
            .map(|(_, rule)| -> Result<()> {
                let mut rule = rule.lock().unwrap();
                if let Some(icon_path) = &rule.definition.icon_path {
                    let path = bundle(icon_path.clone(), self.src.clone(), self.dest.clone())?;

                    rule.definition.icon_path = Some(path);
                }

                Ok(())
            })
            .all(|_| true);

        Ok(())
    }
}

impl Display for StyleSheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.rules
                .par_iter()
                .map(|(_, rule)| ok!(rule.lock()).to_string())
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

impl ParallelExtend<(String, StyleRule)> for StyleSheet {
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = (String, StyleRule)>,
    {
        self.rules
            .par_extend(par_iter.into_par_iter().map(|(k, v)| (k, Mutex::new(v))))
    }
}

static COUNT: AtomicUsize = AtomicUsize::new(0);

#[cached(result = true)]
pub fn bundle(path: PathBuf, src: PathBuf, dest: PathBuf) -> Result<PathBuf> {
    let src = join!(src, &path);
    let src = match src.canonicalize() {
        Ok(v) => v,
        Err(_) => {
            error!(
                "cp: cannot stat '{}': No such file or directory",
                src.to_string_lossy()
            );
            bail!("{} does not exist", src.to_string_lossy())
        }
    };
    let path = PathBuf::from(format!(
        "{}.{}",
        radix_36(COUNT.fetch_add(1, Ordering::Relaxed)),
        path.extension()
            .unwrap_or(&OsStr::new("jpg"))
            .to_string_lossy()
    ));

    let d = join!(dest, &path);
    debug!("cp {} {}", src.to_string_lossy(), d.to_string_lossy());
    let _ = copy(src, d);
    Ok(path)
}

pub fn clear_bundle_cache() {
    ok!(BUNDLE.lock()).cache_clear();
    COUNT.store(0, Ordering::Relaxed);
}
