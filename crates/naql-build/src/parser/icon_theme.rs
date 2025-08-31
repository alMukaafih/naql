use super::Parser;
use crate::css::{FolderType, FontRule, StyleRule, StyleSheet, bundle};
use anyhow::Result;
use naql_shared::traits::WriteToFile;
use naql_shared::{join, manifest::vscode::icon_theme::IconThemeManifest, ok, own};
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::mem::take;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub struct IconThemeParser {
    id: String,
    src: PathBuf,
    build: PathBuf,
    manifest: IconThemeManifest,
}

impl IconThemeParser {
    pub fn new(id: String, src: PathBuf, build: PathBuf, manifest: IconThemeManifest) -> Self {
        Self {
            id,
            src,
            build,
            manifest,
        }
    }
}

impl Parser for IconThemeParser {
    type Output = ();

    fn parse(&mut self) -> anyhow::Result<Self::Output> {
        let mut definitions = take(&mut self.manifest.icon_definitions);

        macro_rules! ensure {
            ($f:tt) => {
                if let Some(file) = &self.manifest.$f {
                    if definitions.get(file).is_none() {
                        self.manifest.$f = None
                    }
                }
            };
        }

        ensure!(file);
        ensure!(folder);
        ensure!(folder_expanded);
        ensure!(root_folder);
        ensure!(root_folder_expanded);

        let mut style_sheet =
            StyleSheet::new(self.src.clone(), join!(&self.build, "dist", "assets"));

        macro_rules! get_or {
            ( $f:tt, $or:expr ) => {{
                if let Some(v) = &self.manifest.$f {
                    own!(v)
                } else {
                    $or
                }
            }};
        }

        let file = get_or!(file, own!(""));
        let folder = get_or!(folder, own!(""));
        let folder_expanded = get_or!(folder_expanded, own!(&folder));

        let root_folder = get_or!(root_folder, own!(&folder));
        let root_folder_expanded = if self.manifest.root_folder.is_some() {
            get_or!(root_folder_expanded, own!(&root_folder))
        } else {
            get_or!(root_folder_expanded, own!(&folder_expanded))
        };

        macro_rules! insert {
            ($key:expr, $selector:expr) => {
                if let Some(definition) = definitions.get_mut(&$key) {
                    definition.is_bundled = true;
                    style_sheet.insert($key, StyleRule::new($selector, definition.clone()));
                }
            };
        }

        insert!(file, ".file_type_default:before");
        insert!(
            folder,
            ".list.collapsible.hidden>.tile[data-type='dir']>.folder:before"
        );
        insert!(folder_expanded, "*[data-type='dir']>.folder:before");
        insert!(
            root_folder,
            ".list.collapsible.hidden>.tile[data-type='root']>.folder:before"
        );
        insert!(root_folder_expanded, "*[data-type='root']>.folder:before");

        let style_sheet = Arc::new(Mutex::new(style_sheet));
        let definitions = Arc::new(definitions);

        self
            .manifest
            .folder_names
            .as_ref()
            .par_iter()
            .flat_map(|x| x.par_iter())
            .map(|x| (FolderType::Normal, x))
            .chain(
                self.manifest
                    .folder_names_expanded
                    .as_ref()
                    .par_iter()
                    .flat_map(|x| x.par_iter())
                    .map(|x| (FolderType::Expanded, x))
                    .chain(
                        self.manifest
                            .root_folder_names
                            .as_ref()
                            .par_iter()
                            .flat_map(|x| x.par_iter())
                            .map(|x| (FolderType::Root, x)),
                    )
                    .chain(
                        self.manifest
                            .root_folder_names_expanded
                            .as_ref()
                            .par_iter()
                            .flat_map(|x| x.par_iter())
                            .map(|x| (FolderType::RootExpanded, x)),
                    ),
            ).for_each_with((style_sheet.clone(), definitions.clone()), |(style_sheet,definitions), (r#type, (k, v))| {
                let selector = match r#type {
                    FolderType::Normal => {
                        format!(
                                ".list.collapsible.hidden>.tile[data-name='{}'i][data-type='dir']>.folder:before",
                                k
                            )
                    }
                    FolderType::Expanded => {
                        format!("*[data-name='{}'i][data-type='dir']>.folder:before", k)
                    }
                    FolderType::Root => {
                        format!(
                            ".list.collapsible.hidden>.tile[data-name='{}'i][data-type='root']>.folder:before",
                            k
                        )
                    }
                    FolderType::RootExpanded => {
                        format!("*[data-name='{}'i][data-type='root']>.folder:before", k)
                    }
                };


                if let Some(definition) = definitions.get(v) {
                    let mut style_sheet = ok!(style_sheet.lock());
                    style_sheet.insert(v.clone(), StyleRule::new(&selector, definition.clone()));
                }
            });

        self.manifest.fonts = None;
        self.manifest.file = None;
        self.manifest.folder = None;
        self.manifest.folder_expanded = None;
        self.manifest.root_folder = None;
        self.manifest.root_folder_expanded = None;
        self.manifest.folder_names = None;
        self.manifest.folder_names_expanded = None;
        self.manifest.root_folder_names = None;
        self.manifest.root_folder_names_expanded = None;

        let mut definitions = Arc::into_inner(definitions).unwrap();

        definitions
            .par_iter_mut()
            .map(|(_, definition)| -> Result<()> {
                if let Some(icon_path) = &definition.icon_path {
                    let path = bundle(
                        icon_path.clone(),
                        self.src.clone(),
                        join!(&self.build, "dist", "assets"),
                    )?;

                    definition.icon_path = Some(path);
                }

                Ok(())
            })
            .all(|_| true);

        let s = if let Some(fonts) = &self.manifest.fonts {
            fonts
                .clone()
                .into_par_iter()
                .map(|mut font| -> Result<String> {
                    for src in &mut font.src {
                        let path = bundle(
                            src.path.clone(),
                            self.src.clone(),
                            join!(&self.build, "dist", "assets"),
                        )?;
                        src.path = path;
                    }
                    Ok(FontRule(font).to_string())
                })
                .flat_map(|x| x.ok())
                .collect::<Vec<_>>()
                .join("")
        } else {
            String::new()
        } + &{
            let mut style_sheet = ok!(style_sheet.lock());
            style_sheet.resolve_urls()?;
            style_sheet.to_string()
        };

        let mut f = BufWriter::new(File::create(join!(
            &self.build,
            "dist",
            "assets",
            format!("{}.iconTheme.css", self.id)
        ))?);

        f.write(s.as_bytes())?;

        self.manifest.icon_definitions = definitions;
        self.manifest.icon_definitions.write_to_file(join!(
            &self.build,
            "src",
            "iconThemes",
            format!("{}.json", self.id)
        ))?;

        Ok(())
    }
}
