use clap::Parser;
use std::fmt::{Display, Formatter, Result as FmtResult};
use io::Result;
use std::fs::metadata;
use std::fs::read_dir;
use std::io;
use std::path::{Path, PathBuf};
use crate::EntryType::{D, F};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EntryType {
    D, // Directory
    F, // File
}

impl EntryType {
    fn label(&self) -> &'static str {
        match self {
            D => "Directory",
            F => "File",
        }
    }
}

impl Display for EntryType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.label())
    }
}

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
        println!("{} {:?}", entry.0.display(), entry.1.label());
    }
}

fn input_logging(cli: &Cli) {
    println!("Source: {}", cli.source.display());
    println!("Target: {}", cli.target.display());
    println!("Is dry run: {}", cli.dry_run);
}

/// 입력 경로가 디렉터리인지 확인합니다.
fn is_directory<P: AsRef<Path>>(path: P) -> Result<bool> {
    metadata(path).map(|md| md.is_dir())
}

/// 입력 경로가 파일인지 확인합니다.
fn is_file<P: AsRef<Path>>(path: P) -> Result<bool> {
    metadata(path).map(|md| md.is_file())
}

/// 입력 경로가 파일이면 그 파일만, 디렉터리이면 디렉터리 내 항목들을 반환합니다.
/// 에러는 호출자에게 전파됩니다.
fn list_entries<P: AsRef<Path>>(path: P) -> Result<Vec<(PathBuf, EntryType)>> {
    let p = path.as_ref();

    if is_file(p)? {
        return Ok(vec![(p.to_path_buf(), F)]);
    }

    if is_directory(p)? {
        let mut items = Vec::new();
        for entry_res in read_dir(p)? {
            let entry = entry_res?;
            if is_file(entry.path())? {

                // 파일인 경우, 항목에 추가
                items.push((entry.path(), F));
            } else if is_directory(entry.path())? {
                items.push((entry.path(), D));

                // 디렉터리인 경우, 재귀적으로 항목들을 추가
                let sub_items = list_entries(entry.path())?;
                items.extend(sub_items);
            }
        }

        return Ok(items);
    }

    Err(
        io::Error::new(
            io::ErrorKind::NotFound, "path is neither file nor directory"
        )
    )
}
