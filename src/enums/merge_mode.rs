use std::fmt::{Display, Formatter, Result};
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum MergeMode {
    SOURCE,
    TARGET,
    BIGGER,
    NEWER,
    DIFFERENT,
    INTERVENTION,
    SKIP
}

impl MergeMode {
    pub fn label(&self) -> &'static str {
        match self {
            MergeMode::SOURCE => "source",
            MergeMode::TARGET => "target",
            MergeMode::BIGGER => "bigger",
            MergeMode::NEWER => "newer",
            MergeMode::DIFFERENT => "different",
            MergeMode::INTERVENTION => "intervention",
            MergeMode::SKIP => "skip",
        }
    }
}

impl Display for MergeMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.label())
    }
}
