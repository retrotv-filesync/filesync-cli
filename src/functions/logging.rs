use std::path::PathBuf;
use crate::Cli;
use crate::enums::entry_type::EntryType;
use crate::enums::entry_type::EntryType::D;

// 입력 값 로깅
pub fn input_logging(cli: &Cli) {
    println!("원본 경로: {}", cli.source.display());
    println!("대상 경로: {}", cli.target.display());
    println!("동기화 모드: {}", cli.sync_mode);
    println!("병합 모드: {}", cli.merge_mode);
    println!("동기화 시뮬레이션 여부: {}", cli.dry_run);
}

// Directory 구조로 항목 로깅
pub fn entry_logging(source_path: &PathBuf, entries: &Vec<(PathBuf, EntryType, i32)>) {
    let mut prefix_paths: Vec<String> = Vec::new();

    for i in 0..entries.len() {
        let entry = &entries[i];
        let indent = "  ".repeat(entry.2 as usize);
        let prefix = format!("{}/", source_path.display());
        let mut display_path = entry.0.display().to_string().replace(&prefix, "");

        // entry의 타입이 Directory인 경우, prefix_paths에 경로를 추가하거나 업데이트
        if entry.1 == D {
            if entry.2 == 0 {
                if prefix_paths.is_empty() {
                    prefix_paths.push(display_path.clone());
                } else if prefix_paths[0] != display_path {
                    prefix_paths[0] = display_path.clone();
                }
            } else {
                if (entry.2 + 1) as usize > prefix_paths.len() {
                    prefix_paths.push(display_path.clone());
                } else if prefix_paths[entry.2 as usize] != display_path {
                    prefix_paths[entry.2 as usize] = display_path.clone();
                }
            }
        }

        let dir_path = if entry.2 == 0 {
            String::new()
        } else {
            prefix_paths[(entry.2 - 1) as usize].clone() + "/"
        };
        display_path = display_path.replace(&dir_path, "");

        println!("{}[{}] {}", indent, entry.1.id(), display_path);
    }
}