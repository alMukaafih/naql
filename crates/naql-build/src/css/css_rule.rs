use super::{font_rule::FontRule, style_rule::StyleRule};
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CssRule {
    FontRule(FontRule),
    StyleRule(StyleRule),
}

impl Display for CssRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CssRule::FontRule(font_rule) => font_rule.fmt(f),
            CssRule::StyleRule(style_rule) => style_rule.fmt(f),
        }
    }
}
