use std::error::Error;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn prints_paths_for_different_dirs() -> Result<(), Box<dyn Error>> {
    let tmp = assert_fs::TempDir::new()?;
    let src = tmp.child("src");
    src.create_dir_all()?;
    let tgt = tmp.child("tgt");
    tgt.create_dir_all()?;

    // verbose 플래그를 추가하여 상세 출력이 나오도록 함
    assert_cmd::cargo::cargo_bin_cmd!("filesync")
        .arg("--source")
        .arg(src.path())
        .arg("--target")
        .arg(tgt.path())
        .arg("--verbose")
        .assert()
        .success()
        .stdout(predicate::str::contains("Source:"))
        .stdout(predicate::str::contains("Target:"))
        .stdout(predicate::str::contains("Is dry run"));

    tmp.close()?;
    Ok(())
}

#[test]
fn fails_when_source_and_target_are_same() -> Result<(), Box<dyn Error>> {
    let tmp = assert_fs::TempDir::new()?;
    let same = tmp.child("same");
    same.create_dir_all()?;

    assert_cmd::cargo::cargo_bin_cmd!("filesync")
        .arg("--source")
        .arg(same.path())
        .arg("--target")
        .arg(same.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("--source`와 `--target`이 같은 경로일 수 없습니다"));

    tmp.close()?;
    Ok(())
}
