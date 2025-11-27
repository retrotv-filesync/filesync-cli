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

    /// 짧은 ID 문자열("D" 또는 "F")을 반환합니다.
    pub fn id(&self) -> &'static str {
        match self {
            EntryType::D => "D",
            EntryType::F => "F",
        }
    }
}

impl Display for EntryType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.label())
    }
}
