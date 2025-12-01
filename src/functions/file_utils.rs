use std::fs::{metadata, read_dir, create_dir_all, copy};
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};
use crate::enums::entry_type::EntryType;
use crate::enums::entry_type::EntryType::{D, F};

/// 입력 경로가 디렉터리인지 확인합니다.
pub fn is_directory<P: AsRef<Path>>(path: P) -> Result<bool> {
    metadata(path).map(|md| md.is_dir())
}

/// 입력 경로가 파일인지 확인합니다.
pub fn is_file<P: AsRef<Path>>(path: P) -> Result<bool> {
    metadata(path).map(|md| md.is_file())
}

/// 입력 경로가 파일이면 그 파일만, 디렉터리이면 디렉터리 내 항목들을 반환합니다.
/// 에러는 호출자에게 전파됩니다.
pub fn list_entries<P: AsRef<Path>>(path: P, depth: i32) -> Result<Vec<(PathBuf, EntryType, i32)>> {
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

/// 원본 경로의 파일/디렉토리를 대상 경로로 복사합니다.
/// entries: list_entries로 얻은 항목 목록
/// source_base: 원본 기준 경로
/// target_base: 대상 기준 경로
/// dry_run: true면 실제 복사하지 않고 시뮬레이션만 수행
/// verbose: true면 복사 과정을 출력
pub fn copy_entries(
    entries: &[(PathBuf, EntryType, i32)],
    source_base: &Path,
    target_base: &Path,
    dry_run: bool,
    verbose: bool
) -> Result<()> {
    for (path, entry_type, _depth) in entries {

        // 원본 경로에서 기준 경로를 제거하여 상대 경로 계산
        let relative_path = path.strip_prefix(source_base)
            .map_err(|e| Error::new(ErrorKind::InvalidInput, format!("경로 처리 실패: {}", e)))?;

        // 대상 경로 생성
        let target_path = target_base.join(relative_path);

        match entry_type {
            D => {
                // 디렉토리 생성
                if verbose {
                    let base_with_sep = format!("{}/", target_base.display());
                    let tp = target_path.display().to_string().replace(&base_with_sep, "");
                    println!("[D]: {}", tp);
                }

                if !dry_run {
                    create_dir_all(&target_path)?;
                }
            },

            F => {
                // 파일 복사 전 부모 디렉토리 생성
                if let Some(parent) = target_path.parent() {
                    if !dry_run {
                        create_dir_all(parent)?;
                    }
                }

                if verbose {
                    let source_with_sep = format!("{}/", source_base.display());
                    let target_with_sep = format!("{}/", target_base.display());
                    let sp = path.display().to_string().replace(&source_with_sep, "");
                    let tp = target_path.display().to_string().replace(&target_with_sep, "");
                    println!("[F]: {} -> {}", sp, tp);
                }

                if !dry_run {
                    copy(path, &target_path)?;
                }
            }
        }
    }

    Ok(())
}

