use std::{collections::HashMap, path::PathBuf};

use crate::own;
use serde::{Deserialize, Serialize, ser::SerializeStruct};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct DefinitionProperties {
    #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "0"))]
    pub icon_path: Option<PathBuf>,

    #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "1"))]
    pub font_character: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "2"))]
    pub font_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "3"))]
    pub font_size: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "4"))]
    pub font_id: Option<String>,

    #[serde(skip_serializing, skip_deserializing)]
    pub is_bundled: bool,
}

#[derive(Clone, Deserialize, Serialize, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Src {
    #[serde(rename(serialize = "0"))]
    pub path: PathBuf,

    #[serde(rename(serialize = "1"))]
    pub format: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct FontProperties {
    pub id: String,
    pub src: Vec<Src>,
    pub weight: Option<String>,
    pub style: Option<String>,
    pub size: Option<String>,
}

pub type Defs = HashMap<String, DefinitionProperties>;
pub type Mapping = Option<HashMap<String, String>>;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct IconThemeManifest {
    pub hides_explorer_arrows: Option<bool>,
    pub fonts: Option<Vec<FontProperties>>,
    pub icon_definitions: Defs,
    pub file: Option<String>,
    pub folder: Option<String>,
    pub folder_expanded: Option<String>,
    pub folder_names: Mapping,
    pub folder_names_expanded: Mapping,
    pub root_folder: Option<String>,
    pub root_folder_expanded: Option<String>,
    pub root_folder_names: Mapping,
    pub root_folder_names_expanded: Mapping,
    pub language_ids: Mapping,

    pub file_extensions: Mapping,
    pub file_names: Mapping,
    pub light: Option<HashMap<String, Value>>,
    pub high_contrast: Option<HashMap<String, Value>>,
}

macro_rules! skip_if_none {
    ( $state:expr , $key:expr, $value:expr ) => {
        if $value.is_some() {
            $state.serialize_field($key, &$value)?;
        }
    };

    ( $state:expr , $key:expr, $value:expr, $map:expr ) => {
        if $value.is_some() {
            let new_map = IconThemeManifest::map(&$value.as_ref().unwrap(), &$map);
            $state.serialize_field($key, &new_map)?;
        }
    };
}

type Map = HashMap<String, i32>;
type DefsMap = HashMap<i32, DefinitionProperties>;

impl IconThemeManifest {
    fn split_icon_defs(&self) -> (Map, DefsMap) {
        let mut defsmap = DefsMap::with_capacity(self.icon_definitions.len());
        let mut map = HashMap::with_capacity(self.icon_definitions.len());
        let mut i = 0;
        for (k, v) in &self.icon_definitions {
            map.insert(k.clone(), i);
            defsmap.insert(i, v.clone());
            i += 1;
        }
        (map, defsmap)
    }

    // Maps HashMap<String, String> -> HashMap<String, i32>
    fn map(from: &HashMap<String, String>, to: &HashMap<String, i32>) -> Map {
        from.iter()
            .filter_map(|(k, v)| to.get(v).map(|v| (own!(k), *v)))
            .collect()
    }
}

impl Serialize for IconThemeManifest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let (map, defsmap) = Self::split_icon_defs(&self);
        let mut state = serializer.serialize_struct("IconThemeManifest", 5)?;
        state.serialize_field("0", &defsmap)?;
        skip_if_none!(state, "1", self.file_extensions, map);
        skip_if_none!(state, "2", self.file_names, map);
        skip_if_none!(state, "3", self.language_ids, map);
        state.end()
    }
}
