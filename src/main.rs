use clap::Parser;
use std::path::PathBuf;
use enums::mode::Mode;

mod enums;
mod functions;
use functions::file_utils::list_entries;

#[derive(Parser)]
#[command(author, version, about = "filesync 예제 CLI")]
struct Cli {
    // short -> 짧은 명령어 사용
    // long -> 긴 명령어 사용
    // value_name -> 값 이름 지정

    // --source=<path> or --source <path>
    /// 원본 디렉토리
    #[arg(long, value_name = "SOURCE_PATH", required = true)]
    source: PathBuf,

    // --target=<path> or --target <path>
    /// 대상 디렉토리
    #[arg(long, value_name = "TARGET_PATH", required = true)]
    target: PathBuf,

    // --mode=<MODE> or --mode <MODE>
    /// 동기화 모드
    #[arg(long, value_enum, value_name = "MODE", default_value = "mirroring")]
    mode: Mode,

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
    let cli = Cli::parse();

    // source와 target이 같은지 확인 (같으면 에러 메시지 출력 후 종료)
    if cli.source == cli.target {
        eprintln!("ERROR: `--source`와 `--target`이 같은 경로일 수 없습니다.");
        std::process::exit(1);
    }

    // 입력 값 로깅
    if cli.verbose {
        input_logging(&cli);
    }

    let fl = list_entries(cli.source).expect("목록을 불러오는 중 오류 발생");
    for entry in fl {
        println!("{} {}", entry.1.id(), entry.0.display());
    }
}

fn input_logging(cli: &Cli) {
    println!("Source: {}", cli.source.display());
    println!("Target: {}", cli.target.display());
    println!("Is dry run: {}", cli.dry_run);
    println!("Mode: {}", cli.mode);
}
