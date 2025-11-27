use clap::Parser;
use std::path::{PathBuf};

mod enums;
mod functions;
use functions::file_utils::list_entries;

#[derive(Parser)]
#[command(author, version, about = "filesync 예제 CLI")]
struct Cli {
    // short -> 짧은 명령어 사용
    // long -> 긴 명령어 사용
    // value_name -> 값 이름 지정

    // --source=<path>
    /// 원본 디렉토리
    #[arg(long, value_name = "SOURCE")]
    source: PathBuf,

    // --target=<path>
    /// 대상 디렉토리
    #[arg(long, value_name = "TARGET")]
    target: PathBuf,

    // --dry-run or -d
    /// 실제로 동작하지 않고 시뮬레이션만 수행
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
        println!("{} {}", entry.0.display(), entry.1.label());
    }
}

fn input_logging(cli: &Cli) {
    println!("Source: {}", cli.source.display());
    println!("Target: {}", cli.target.display());
    println!("Is dry run: {}", cli.dry_run);
}
