use std::fs::{metadata, read_dir, create_dir_all};
use fs_extra::file::{copy as fse_copy, CopyOptions};
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};
use crate::Cli;
use crate::enums::entry_type::EntryType;
use crate::enums::entry_type::EntryType::{D, F};
use crate::enums::merge_mode::MergeMode;

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
    cli: &Cli,
) -> Result<()> {
    for (path, entry_type, _depth) in entries {

        // 원본 경로에서 기준 경로를 제거하여 상대 경로 계산
        let relative_path = path.strip_prefix(&cli.source)
            .map_err(|e| Error::new(ErrorKind::InvalidInput, format!("경로 처리 실패: {}", e)))?;

        // 대상 경로 생성
        let target_path = cli.target.join(relative_path);

        match entry_type {
            D => {
                // 디렉토리 생성
                if cli.verbose {
                    let base_with_sep = format!("{}/", cli.target.display());
                    let tp = target_path.display().to_string().replace(&base_with_sep, "");
                    println!("[D]: {}", tp);
                }

                if !cli.dry_run {
                    create_dir_all(&target_path)?;
                }
            },

            F => {
                // 파일 복사 전 부모 디렉토리 생성
                if let Some(parent) = target_path.parent() {
                    if !cli.dry_run {
                        create_dir_all(parent)?;
                    }
                }

                match cli.merge_mode {
                    MergeMode::SOURCE => {
                        source_copy(&cli, path, &target_path)?
                    }

                    MergeMode::TARGET => {
                        target_copy(&cli, path, &target_path)?
                    }

                    MergeMode::BIGGER => {
                        bigger_copy(&cli, path, &target_path)?
                    }

                    _ => {
                        todo!("이 병합 모드는 아직 구현되지 않았습니다: {:?}", cli.merge_mode);
                    }
                }
            }
        }
    }

    Ok(())
}

fn source_copy(cli: &Cli, path: &Path, target_path: &Path) -> Result<()> {
    if target_path.exists() {
        let source_meta = metadata(path)?;
        let target_meta = metadata(target_path)?;

        // 파일 크기와 수정 시간이 같으면 동일한 파일로 간주하고 건너뜁니다.
        if source_meta.len() == target_meta.len() && source_meta.modified()? == target_meta.modified()? {
            if cli.verbose {
                println!("[F]: {} 건너뛰기 (파일 동일)", path.display());
            }
            return Ok(());
        }
    }

    if cli.verbose {
        println!("[F]: {} -> {}", path.display(), target_path.display());
    }

    overwrite_copy(cli, path, target_path)
}

fn target_copy(cli: &Cli, path: &Path, target_path: &Path) -> Result<()> {
    if !target_path.exists() {
        if cli.verbose {
            println!("[F]: {} -> {} (대상 파일 없음)", path.display(), target_path.display());
        }
        overwrite_copy(cli, path, target_path)
    } else {
        if cli.verbose {
            println!("[F]: {} 건너뛰기 (대상 파일 유지)", path.display());
        }
        Ok(())
    }
}

fn bigger_copy(cli: &Cli, path: &Path, target_path: &Path) -> Result<()> {
    if !target_path.exists() {
        if cli.verbose {
            println!("[F]: {} -> {} (대상 파일 없음)", path.display(), target_path.display());
        }
        return overwrite_copy(cli, path, target_path);
    }

    let source_meta = metadata(path)?;
    let target_meta = metadata(target_path)?;

    if source_meta.len() > target_meta.len() {
        if cli.verbose {
            println!("[F]: {} -> {} (원본이 더 큼)", path.display(), target_path.display());
        }
        overwrite_copy(cli, path, target_path)
    } else if target_meta.len() > source_meta.len() {
        if cli.verbose {
            println!("[F]: {} <- {} (대상이 더 큼)", path.display(), target_path.display());
        }
        // BIGGER 모드에서는 대상이 더 크면 원본을 덮어써야 합니다.
        overwrite_copy(cli, target_path, path)
    } else {
        if cli.verbose {
            println!("[F]: {} 건너뛰기 (크기 동일)", path.display());
        }
        Ok(())
    }
}

fn overwrite_copy(cli: &Cli, path: &Path, target_path: &Path) -> Result<()> {
    if !cli.dry_run {
        let options = CopyOptions {
            overwrite: true,
            ..Default::default()
        };
        fse_copy(path, target_path, &options)
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
    }
    Ok(())
}
