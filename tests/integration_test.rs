use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("vsh").unwrap();
    cmd.arg("--help");
    cmd.assert().success();
}

#[test]
fn test_copy_command_terse() {
    let dir = tempdir().unwrap();
    let source = dir.path().join("source.txt");
    let dest = dir.path().join("dest.txt");

    // Create source file
    fs::write(&source, "test content").unwrap();

    let mut cmd = Command::cargo_bin("vsh").unwrap();
    cmd.arg("-c")
        .arg(format!("cp {} {}", source.display(), dest.display()));

    cmd.assert().success();
    assert!(dest.exists());
}

#[test]
fn test_copy_command_verbose() {
    let dir = tempdir().unwrap();
    let source = dir.path().join("source.txt");
    let dest = dir.path().join("dest.txt");

    fs::write(&source, "test content").unwrap();

    let mut cmd = Command::cargo_bin("vsh").unwrap();
    cmd.arg("-c")
        .arg(format!("copy {} to {}", source.display(), dest.display()));

    cmd.assert().success();
    assert!(dest.exists());
}

#[test]
fn test_copy_command_named() {
    let dir = tempdir().unwrap();
    let source = dir.path().join("source.txt");
    let dest = dir.path().join("dest.txt");

    fs::write(&source, "test content").unwrap();

    let mut cmd = Command::cargo_bin("vsh").unwrap();
    cmd.arg("-c").arg(format!(
        "copy source={} destination={}",
        source.display(),
        dest.display()
    ));

    cmd.assert().success();
    assert!(dest.exists());
}

#[test]
fn test_copy_nonexistent_file() {
    let mut cmd = Command::cargo_bin("vsh").unwrap();
    cmd.arg("-c").arg("copy nonexistent.txt to dest.txt");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("File not found"));
}

#[test]
fn test_list_command() {
    let dir = tempdir().unwrap();

    let mut cmd = Command::cargo_bin("vsh").unwrap();
    cmd.arg("-c")
        .arg(format!("list {}", dir.path().display()));

    cmd.assert().success();
}
