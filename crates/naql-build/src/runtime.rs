use anyhow::Result;
use minijinja::{Environment, Value, context};
use naql_shared::join;
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[derive(Default, PartialEq, Serialize)]
pub struct Include {
    pub icon_themes: bool,
}

impl Include {
    fn is_default(&self) -> bool {
        *self == Include::default()
    }
}

pub fn js_string(value: Value) -> String {
    format!("`{}`", value.as_str().unwrap())
}

pub fn include_main(
    env: &mut Environment,
    include: Include,
    id: &str,
    build_dir: &Path,
) -> Result<()> {
    if include.is_default() {
        return Ok(());
    }
    let main = env.get_template("main.js")?;
    let main = main.render(context! {
        include, id
    })?;

    let mut f = BufWriter::new(File::create(join!(build_dir, "src", "main.js"))?);

    f.write(main.as_bytes())?;

    Ok(())
}

pub fn include_icon_themes(
    env: &mut Environment,
    details: Vec<(String, String)>,
    build_dir: &Path,
) -> Result<()> {
    let icon_themes = env.get_template("iconThemes.js")?;
    let icon_themes = icon_themes.render(context! {
        details
    })?;

    let mut f = BufWriter::new(File::create(join!(build_dir, "src", "iconThemes.js"))?);

    f.write(icon_themes.as_bytes())?;

    Ok(())
}
