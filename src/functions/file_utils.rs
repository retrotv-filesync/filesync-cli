use std::fs::{metadata, read_dir};
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use crate::enums::entry_type::EntryType;
use crate::enums::entry_type::EntryType::{D, F};

/// 입력 경로가 디렉터리인지 확인합니다.
pub fn is_directory<P: AsRef<Path>>(path: P) -> std::io::Result<bool> {
    metadata(path).map(|md| md.is_dir())
}

/// 입력 경로가 파일인지 확인합니다.
pub fn is_file<P: AsRef<Path>>(path: P) -> std::io::Result<bool> {
    metadata(path).map(|md| md.is_file())
}

/// 입력 경로가 파일이면 그 파일만, 디렉터리이면 디렉터리 내 항목들을 반환합니다.
/// 에러는 호출자에게 전파됩니다.
pub fn list_entries<P: AsRef<Path>>(path: P, depth: i32) -> std::io::Result<Vec<(PathBuf, EntryType, i32)>> {
    let p = path.as_ref();

    if is_file(p)? {
        return Ok(vec![(p.to_path_buf(), F, depth)]);
    }

    if is_directory(p)? {
        let mut items = Vec::new();
        for entry_res in read_dir(p)? {
            let entry = entry_res?;
            if is_file(entry.path())? {

                // 파일인 경우, 항목에 추가
                items.push((entry.path(), F, depth));
            } else if is_directory(entry.path())? {
                items.push((entry.path(), D, depth));

                // 디렉터리인 경우, 재귀적으로 항목들을 추가
                let sub_items = list_entries(entry.path(), depth + 1)?;
                items.extend(sub_items);
            }
        }

        return Ok(items);
    }

    Err(
        Error::new(
            ErrorKind::NotFound, "path is neither file nor directory"
        )
    )
}
