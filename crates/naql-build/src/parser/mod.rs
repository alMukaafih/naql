pub mod icon_theme;

pub trait Parser {
    type Output;
    fn parse(&mut self) -> anyhow::Result<Self::Output>;
}
