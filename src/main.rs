use clap::Parser;
use std::path::PathBuf;
use enums::sync_mode::SyncMode;

mod enums;
mod functions;
use functions::file_utils::{list_entries, copy_entries};
use crate::enums::fallback_mode::FallbackMode;
use crate::enums::merge_mode::MergeMode;
use crate::enums::sync_mode::SyncMode::MIRRORING;
use crate::enums::merge_mode::MergeMode::{SOURCE};
use crate::functions::logging::{entry_logging, input_logging};

#[derive(Parser)]
#[command(author, version, about = "filesync 예제 CLI")]
struct Cli {
    // short -> 짧은 명령어 사용
    // long -> 긴 명령어 사용
    // value_name -> 값 이름 지정

    // --source=<SOURCE_PATH> or --source <SOURCE_PATH>
    /// 원본 디렉토리
    #[arg(long, value_name = "SOURCE_PATH", required = true)]
    source: PathBuf,

    // --target=<TARGET_PATH> or --target <TARGET_PATH>
    /// 대상 디렉토리
    #[arg(long, value_name = "TARGET_PATH", required = true)]
    target: PathBuf,

    // --sync-mode=<SYNC_MODE> or --sync-mode <SYNC_MODE>
    /// 동기화 모드
    #[arg(long, value_enum, value_name = "SYNC_MODE", default_value = "mirroring")]
    sync_mode: SyncMode,

    // --merge-mode=<MERGE_MODE> or --merge-mode <MERGE_MODE>
    /// 병합 모드
    #[arg(long, value_enum, value_name = "MERGE_MODE", default_value = "source")]
    merge_mode: MergeMode,

    // --fallback=<FALLBACK_MODE> or --fallback <FALLBACK_MODE>
    /// 병합 모드 적용 불가 시 대체 모드
    #[arg(long, value_enum, value_name = "FALLBACK_MODE", default_value = "skip")]
    fallback: FallbackMode,

    // --dry-run or -d
    /// 동기화 시뮬레이션 실행
    #[arg(short, long)]
    dry_run: bool,

    // --verbose or -v
    /// 상세 출력 활성화
    #[arg(short, long)]
    verbose: bool
}

fn main() {
    let mut cli = Cli::parse();

    // source와 target이 같은지 확인 (같으면 에러 메시지 출력 후 종료)
    if cli.source == cli.target {
        eprintln!("ERROR: `--source`와 `--target`이 같은 경로일 수 없습니다. 동기화가 수행되지 않습니다.");
        std::process::exit(1);
    }

    // 미러링 모드일 때 merge_mode가 SOURCE인지 확인 (아니면 경고 메시지 출력 후 자동으로 SOURCE로 설정)
    if cli.sync_mode == MIRRORING {
        if cli.merge_mode != SOURCE {
            println!("WARN: 미러링 모드에서는 `--merge-mode`를 'source'로만 설정할 수 있습니다. 자동으로 'source'로 설정됩니다.");
            cli.merge_mode = SOURCE;
        }
    }

    if cli.dry_run {
        cli.verbose = true;
        println!("INFO: 동기화 시뮬레이션 모드가 활성화 되었습니다. verbose 옵션이 활성화 됩니다.")
    }

    // 입력 값 로깅
    input_logging(&cli);

    // 원본 경로의 목록 불러오기
    let fl = list_entries(&cli.source, 0).unwrap_or_else(|err| {
        eprintln!("[ERROR]: 목록을 불러오는 중 오류 발생");
        eprintln!("[ERROR]: {}", err);
        std::process::exit(1);
    });

    // 불러온 목록 로깅
    entry_logging(&cli, &fl);

    // 파일 및 디렉토리 복사
    copy_entries(&fl, &cli).unwrap_or_else(|err| {
        eprintln!("[ERROR]: 파일 복사 중 오류 발생");
        eprintln!("[ERROR]: {}", err);
        std::process::exit(1);
    });

    if cli.dry_run {
        logging!(cli.verbose, "시뮬레이션 완료 (실제로 복사되지 않음)");
    } else {
        print!("파일 동기화 완료");
    }
}
