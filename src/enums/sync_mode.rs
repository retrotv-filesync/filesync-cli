use std::fmt::{Display, Formatter, Result};
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum SyncMode {
    MIRRORING, // Directory
    SYNC, // File
}

impl SyncMode {
    pub fn label(&self) -> &'static str {
        match self {
            SyncMode::MIRRORING => "mirroring",
            SyncMode::SYNC => "sync",
        }
    }
}

impl Display for SyncMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.label())
    }
}
