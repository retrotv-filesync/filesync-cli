use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryType {
    D, // Directory
    F, // File
}

impl EntryType {
    pub fn label(&self) -> &'static str {
        match self {
            EntryType::D => "Directory",
            EntryType::F => "File",
        }
    }
}

impl Display for EntryType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.label())
    }
}

