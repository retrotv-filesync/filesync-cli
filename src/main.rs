use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
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

    if cli.source == cli.target {
        eprintln!("ERROR: `--source`와 `--target`이 같은 경로일 수 없습니다.");
        std::process::exit(1);
    }

    if cli.verbose {
        input_logging(cli);
    }


}

fn input_logging(cli: Cli) {
    println!("Source: {}", cli.source.display());
    println!("Target: {}", cli.target.display());
    println!("Is dry run: {}", cli.dry_run);
}
