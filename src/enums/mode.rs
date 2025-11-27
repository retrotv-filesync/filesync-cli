use std::fmt::{Display, Formatter, Result};
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Mode {
    MIRRORING, // Directory
    SYNC, // File
}

impl Mode {
    pub fn label(&self) -> &'static str {
        match self {
            Mode::MIRRORING => "mirroring",
            Mode::SYNC => "sync",
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.label())
    }
}
