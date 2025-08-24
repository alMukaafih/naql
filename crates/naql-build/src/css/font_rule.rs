use naql_shared::manifest::vscode::icon_theme::FontProperties;
use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FontRule(pub FontProperties);

impl Display for FontRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut srcs = vec![];
        for src in &self.src {
            srcs.push(format!(
                "url({})format('{}')",
                src.path.to_string_lossy(),
                src.format
            ));
        }

        write!(f, "@font-face{{")?;
        // write!(f, "font-display:block;")?;
        write!(f, "font-family:'{}';", self.id)?;
        write!(f, "src:{};", srcs.join(","))?;

        if let Some(size) = &self.size {
            write!(f, "font-size:{size};")?;
        }

        if let Some(style) = &self.style {
            write!(f, "font-style:{style};")?;
        }

        if let Some(weight) = &self.weight {
            write!(f, "font-weight:{weight};")?;
        }

        write!(f, "}}")
    }
}

impl Deref for FontRule {
    type Target = FontProperties;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FontRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
