use std::fmt::{Display, Formatter, Result};
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum FallbackMode {
    SOURCE,
    TARGET,
    BIGGER,
    NEWER,
    DIFFERENT,
    INTERVENTION,
    SKIP
}

impl FallbackMode {
    pub fn label(&self) -> &'static str {
        match self {
            FallbackMode::SOURCE => "source",
            FallbackMode::TARGET => "target",
            FallbackMode::BIGGER => "bigger",
            FallbackMode::NEWER => "newer",
            FallbackMode::DIFFERENT => "different",
            FallbackMode::INTERVENTION => "intervention",
            FallbackMode::SKIP => "skip",
        }
    }
}

impl Display for FallbackMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.label())
    }
}
