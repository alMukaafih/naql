#![allow(clippy::pedantic)]
use anyhow::Result;
pub use args::*;
use naql_shared::manifest::acode::AcodeManifest;
use naql_shared::manifest::vscode::VsCodeManifest;
use naql_shared::traits::ReadFromFile;
use naql_shared::zip::{unzip, zip};
use naql_shared::{join, manifest::vscode::icon_theme::IconThemeManifest};
use rayon::prelude::*;
use tempfile::TempDir;

use minijinja::Environment;
use naql_shared::{ok, own};
use parser::Parser;
use parser::icon_theme::IconThemeParser;
use runtime::{Include, include_icon_themes, include_main, js_string};
use tracing::{debug, instrument};
use util::esbuild;

mod args;
mod css;
mod parser;
mod runtime;
mod util;

pub struct Builder {
    args: BuildArgs,
}

impl Builder {
    pub fn new(args: BuildArgs) -> Self {
        Self { args }
    }

    #[instrument(skip_all)]
    pub fn build(&mut self) -> Result<()> {
        let input_path = self.args.path.canonicalize()?;

        let/*  mut */ tmp_dir = TempDir::with_prefix_in(".naql-", ".")?;
        // tmp_dir.disable_cleanup(true);

        let src_dir = if input_path.is_file() {
            unzip(input_path, tmp_dir.path())?;
            join!(tmp_dir.path(), "extension")
        } else {
            input_path
        };

        let vs_manifest = VsCodeManifest::read_from_file(join!(&src_dir, "package.json"))?;
        debug!("Building plugin for {}", vs_manifest.display_name);

        let mut manifest: AcodeManifest = vs_manifest.clone().into();
        manifest.resolve(&src_dir);
        if let Some(path) = &self.args.manifest {
            let mut other = AcodeManifest::read_from_file(&path)?;
            other.resolve(path.parent().unwrap());
            manifest.merge(other);
        }

        let contributes = vs_manifest.contributes;
        let build_dir = util::build_dir(ok!(manifest.id.as_ref()))?;
        let mut include = Include::default();

        // Load templates
        let mut env = Environment::new();
        env.add_filter("js_string", js_string);
        minijinja_embed::load_templates!(&mut env);

        if let Some(icon_themes) = contributes.icon_themes {
            include.icon_themes = true;

            util::contrib_dir(&build_dir, "iconThemes")?;
            let details = icon_themes
                .into_par_iter()
                .map(|info| -> Result<(String, String)> {
                    let src = join!(&src_dir, info.path);
                    let manifest = IconThemeManifest::read_from_file(&src)?;
                    let mut parser = IconThemeParser::new(
                        info.id.clone(),
                        own!(src.parent().unwrap()),
                        own!(&build_dir),
                        manifest,
                    );
                    parser.parse()?;

                    Ok((info.id.clone(), info.label))
                })
                .filter_map(|x| x.ok())
                .collect::<Vec<_>>();

            include_icon_themes(&mut env, details, &build_dir)?;
        }

        include_main(&mut env, include, ok!(manifest.id.as_ref()), &build_dir)?;
        esbuild(&build_dir)?;
        manifest.bundle(join!(&build_dir, "dist"))?;

        let output = join!(&self.args.outdir, &self.args.outfile);
        let size = zip(build_dir, &output)?;

        println!();
        println!(
            "written output to {} with size {size}",
            output.to_string_lossy()
        );

        Ok(())
    }
}
