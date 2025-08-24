use naql_shared::{manifest::vscode::icon_theme::DefinitionProperties, own};
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StyleRule {
    pub selectors: Vec<String>,
    pub definition: DefinitionProperties,
}

impl StyleRule {
    pub fn new(selector: &str, definition: DefinitionProperties) -> Self {
        Self {
            selectors: vec![own!(selector)],
            definition,
        }
    }
}

impl Display for StyleRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.selectors.join(","))?;
        write!(f, "{{")?;
        write!(
            f,
            "content:'{}'!important;",
            self.definition
                .font_character
                .as_ref()
                .map_or("", |s| s.as_str())
        )?;

        if self.definition.icon_path.is_some() {
            write!(f, "display:inline-block;")?;
            write!(f, "background-size:contain;")?;
            write!(f, "background-repeat:no-repeat;")?;
            write!(f, "height:1em;")?;
            write!(f, "width:1em;")?;
        }

        if let Some(icon_path) = &self.definition.icon_path {
            write!(f, "background-image:url({});", icon_path.to_string_lossy())?;
        }

        if let Some(font_color) = &self.definition.font_color {
            write!(f, "color:{font_color};")?;
        }

        if let Some(font_id) = &self.definition.font_id {
            write!(f, "font-family:{font_id};")?;
        }

        if let Some(font_size) = &self.definition.font_size {
            write!(f, "font-size:{font_size};")?;
        }

        write!(f, "}}")
    }
}
