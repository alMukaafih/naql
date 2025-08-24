#![allow(dead_code)]

use anyhow::Result;
pub use args::*;
use naql_shared::manifest::acode::AcodeManifest;
use naql_shared::manifest::vscode::VsCodeManifest;
use naql_shared::traits::ReadFromFile;
use naql_shared::zip::unzip;
use naql_shared::{join, manifest::vscode::icon_theme::IconThemeManifest};
use rayon::prelude::*;
use tempfile::TempDir;

use naql_shared::{ok, own};
use parser::Parser;
use parser::icon_theme::IconThemeParser;
use tracing::{debug, instrument};

mod args;
mod css;
mod parser;
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

        let mut tmp_dir = TempDir::with_prefix_in(".naql-", ".")?;
        tmp_dir.disable_cleanup(true);

        let src_dir = if input_path.is_file() {
            unzip(input_path, tmp_dir.path())?;
            join!(tmp_dir.path(), "extension")
        } else {
            input_path
        };

        let vs_manifest = VsCodeManifest::read_from_file(join!(&src_dir, "package.json"))?;
        debug!("Building plugin for {}", vs_manifest.display_name);

        let mut manifest: AcodeManifest = vs_manifest.clone().into();
        if let Some(path) = &self.args.manifest {
            manifest.merge(AcodeManifest::read_from_file(path)?);
        }

        let contributes = vs_manifest.contributes;

        let build_dir = util::build_dir(ok!(manifest.id.as_ref()))?;

        let mut include = Include::default();

        if let Some(icon_themes) = contributes.icon_themes {
            include.icon_theme = true;

            let contrib_dir = util::contrib_dir(build_dir, "iconThemes")?;
            icon_themes
                .into_par_iter()
                .map(|info| -> Result<()> {
                    let src = join!(&src_dir, info.path);
                    let manifest = IconThemeManifest::read_from_file(&src)?;
                    let mut parser = IconThemeParser::new(
                        info.id,
                        own!(src.parent().unwrap()),
                        join!(&contrib_dir),
                        manifest,
                    );
                    parser.parse()?;

                    Ok(())
                })
                .find_any(Result::is_err)
                .transpose()?;
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct Include {
    icon_theme: bool,
}
