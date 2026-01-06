use std::io::Error;
use std::path::{Path, PathBuf};
use crate::enums::entry_type::EntryType;
use crate::enums::entry_type::EntryType::{D, F};

pub struct DirEntry {
    pub depth: i32,
    pub path: PathBuf,
    pub file_name: String,
    pub entry_type: EntryType,
    pub child_entries: Vec<DirEntry>,
}

impl DirEntry {
    pub fn new(path: &Path) -> Result<Self, Error> {
        let mut root = DirEntry {
            depth: 0,
            path: path.to_path_buf(),
            file_name: path
                .file_name()
                .map(|os_str| os_str.to_string_lossy().to_string())
                .unwrap_or_else(|| String::from("")),
            entry_type: if path.is_file() { F } else { D },
            child_entries: Vec::new(),
        };

        // 디렉토리인 경우 자식 항목들을 재귀적으로 추가
        if path.is_dir() {
            let entries = std::fs::read_dir(path)?;
            for entry_res in entries {
                let entry = entry_res?;
                let entry_path = entry.path();
                let mut child_entry = DirEntry::new(&entry_path)?;
                child_entry.depth = root.depth + 1;

                root.child_entries.push(child_entry);
            }
        }

        Ok(root)
    }

    pub fn is_file(&self) -> bool {
        matches!(self.entry_type, F)
    }

    pub fn is_dir(&self) -> bool {
        matches!(self.entry_type, D)
    }
}
