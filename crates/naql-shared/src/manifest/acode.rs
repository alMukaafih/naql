use super::vscode::VsCodeManifest;
use crate::{own, path};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Plugin.json is a manifest file that contains information about the plugin,
/// such as name, description, author, etc. It is required for every plugin.
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcodeManifest {
    /// ID of the plugin, reverse domain name format
    pub id: Option<String>,
    /// Name of the plugin
    pub name: Option<String>,
    /// Path to the main.js file
    pub main: Option<String>,
    /// Version of the plugin
    pub version: Option<String>,
    /// Path to the readme.md file
    pub readme: Option<PathBuf>,
    /// Path to the icon.png file
    pub icon: Option<PathBuf>,
    /// List of files to be included in the plugin zip file
    pub files: Option<Vec<String>>,
    /// Price of the plugin in INR (min. 10 and max. 10000), if 0 or omitted, plugin is free, this can be changed later.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<u16>,
    /// Minimum acode version code required to run the plugin
    pub min_version_code: Option<i32>,
    /// Author
    pub author: Option<Author>,
    /// Dependencies
    pub dependencies: Option<Vec<String>>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,
}
impl AcodeManifest {
    pub fn merge(&mut self, with: Self) {
        macro_rules! merge {
            ( $f:tt ) => {
                if with.$f.is_some() {
                    self.$f = with.$f
                }
            };

            ( $f:tt, $v:expr ) => {
                let val = self.$f.get_or_insert($v);
                if let Some(v) = &with.$f {
                    val.extend_from_slice(v);
                }
            };
        }

        merge!(id);
        merge!(name);
        merge!(main);
        merge!(version);
        merge!(readme);
        merge!(icon);
        merge!(price);
        merge!(min_version_code);
        merge!(author);
        merge!(files, vec![]);
        merge!(dependencies, vec![]);
    }
}

impl From<VsCodeManifest> for AcodeManifest {
    fn from(manifest: VsCodeManifest) -> Self {
        Self {
            id: Some(manifest.id()),
            name: Some(manifest.display_name),
            main: Some(own!("main.js")),
            version: Some(manifest.version),
            readme: Some(path!("readme.md")),
            icon: Some(path!("icon.png")),
            files: None,
            price: None,
            min_version_code: Some(955),
            author: Some(Author {
                name: manifest.author.name,
                email: manifest.author.email,
                url: manifest.author.url,
                github: None,
            }),
            dependencies: Some(vec![own!("almukaafih.vscode_api")]),
        }
    }
}
