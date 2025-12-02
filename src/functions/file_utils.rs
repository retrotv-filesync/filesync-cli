use std::fs::{metadata, read_dir, create_dir_all};
use fs_extra::file::{copy as fse_copy, CopyOptions};
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};
use crate::Cli;
use crate::enums::entry_type::EntryType;
use crate::enums::entry_type::EntryType::{D, F};
use crate::enums::merge_mode::MergeMode::{SOURCE, TARGET, BIGGER};

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
                    SOURCE => {
                        source_copy(&cli, path, &target_path)?
                    }

                    TARGET => {
                        target_copy(&cli, &target_path, path)?
                    }

                    BIGGER => {
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
        // 대상 파일이 존재할 경우, 동일한 파일인지 확인
        if is_same_file(path, target_path)? {
            if cli.verbose {
                println!("[F]: {} 건너뛰기 (파일 동일)", path.display());
            }

            // 동일 파일이면 아무것도 하지 않고 성공 반환
            Ok(())
        } else {
            // 다른 파일이면 덮어쓰기
            if cli.verbose {
                println!("[F]: {} -> {} 덮어쓰기", path.display(), target_path.display());
            }

            overwrite_copy(cli, path, target_path)
        }
    } else {
        // 대상 파일이 존재하지 않으면 새로 복사
        if cli.verbose {
            println!("[F]: {} -> {} 복사", path.display(), target_path.display());
        }

        overwrite_copy(cli, path, target_path)
    }
}

fn target_copy(cli: &Cli, target_path: &Path, path: &Path) -> Result<()> {
    // TARGET 모드에서는 target -> source 방향으로 복사합니다.
    // 먼저, 대상 파일이 실제로 존재하는지 확인해야 합니다.
    if !target_path.exists() {
        if cli.verbose {
            println!("[F]: {} 건너뛰기 (대상 파일 없음)", target_path.display());
        }
        
        return Ok(());
    }

    // 원본 경로에 파일이 존재하는지 확인
    if path.exists() {
        // 원본과 대상 파일이 동일한지 확인
        if is_same_file(target_path, path)? {
            if cli.verbose {
                println!("[F]: {} 건너뛰기 (파일 동일)", path.display());
            }

            Ok(())
        } else {
            // 다른 파일이면 대상 파일로 원본 파일을 덮어쓰기
            if cli.verbose {
                println!("[F]: {} <- {} 덮어쓰기", path.display(), target_path.display());
            }

            overwrite_copy(cli, target_path, path)
        }
    } else {
        // 원본 파일이 존재하지 않으면 대상 파일을 원본 위치로 복사
        if cli.verbose {
            println!("[F]: {} <- {} 복사", path.display(), target_path.display());
        }

        overwrite_copy(cli, target_path, path)
    }
}

fn bigger_copy(cli: &Cli, path: &Path, target_path: &Path) -> Result<()> {
    if !target_path.exists() {
        if cli.verbose {
            println!("[F]: {} -> {} 복사", path.display(), target_path.display());
        }

        return overwrite_copy(cli, path, target_path);
    }

    let source_meta = metadata(path)?;
    let target_meta = metadata(target_path)?;

    if source_meta.len() > target_meta.len() {
        if cli.verbose {
            println!("[F]: {} -> {} 덮어쓰기 (원본이 더 큼)", path.display(), target_path.display());
        }

        overwrite_copy(cli, path, target_path)
    } else if target_meta.len() > source_meta.len() {
        if cli.verbose {
            println!("[F]: {} <- {} 덮어쓰기 (대상이 더 큼)", path.display(), target_path.display());
        }

        // BIGGER 모드에서는 대상이 더 크면 원본을 덮어써야 합니다.
        overwrite_copy(cli, target_path, path)
    } else {
        if cli.verbose {
            println!("[F]: {} 건너뛰기 (파일 동일)", path.display());
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

fn is_same_file(path1: &Path, path2: &Path) -> Result<bool> {
    let meta1 = metadata(path1)?;
    let meta2 = metadata(path2)?;

    Ok(meta1.len() == meta2.len() && meta1.modified()? == meta2.modified()?)
}
