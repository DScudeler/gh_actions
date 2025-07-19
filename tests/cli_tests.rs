use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::NamedTempFile;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("gh_actions").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("A sample Rust application for testing GitHub Actions"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("gh_actions").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn test_fibonacci_command() {
    let mut cmd = Command::cargo_bin("gh_actions").unwrap();
    cmd.args(&["fib", "10"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Fibonacci of 10 is: 55"));
}

#[test]
fn test_fibonacci_invalid_input() {
    let mut cmd = Command::cargo_bin("gh_actions").unwrap();
    cmd.args(&["fib", "abc"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid number"));
}

#[test]
fn test_fibonacci_too_large() {
    let mut cmd = Command::cargo_bin("gh_actions").unwrap();
    cmd.args(&["fib", "100"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("too large"));
}

#[test]
fn test_user_add_command() {
    let mut cmd = Command::cargo_bin("gh_actions").unwrap();
    cmd.args(&["user", "add", "1", "John Doe", "john@example.com"])
        .assert()
        .success()
        .stdout(predicate::str::contains("User added successfully!"));
}

#[test]
fn test_user_add_invalid_email() {
    let mut cmd = Command::cargo_bin("gh_actions").unwrap();
    cmd.args(&["user", "add", "1", "John Doe", "invalid-email"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid email format"));
}

#[test]
fn test_user_add_invalid_id() {
    let mut cmd = Command::cargo_bin("gh_actions").unwrap();
    cmd.args(&["user", "add", "abc", "John Doe", "john@example.com"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid user ID"));
}

#[test]
fn test_user_list_empty() {
    let mut cmd = Command::cargo_bin("gh_actions").unwrap();
    cmd.args(&["user", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Users:"));
}

#[test]
fn test_user_help() {
    let mut cmd = Command::cargo_bin("gh_actions").unwrap();
    cmd.args(&["user", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("User management operations"));
}

#[test]
fn test_no_args() {
    let mut cmd = Command::cargo_bin("gh_actions").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Welcome to gh_actions!"))
        .stdout(predicate::str::contains("Use --help to see available commands."));
}

#[test]
fn test_invalid_command() {
    let mut cmd = Command::cargo_bin("gh_actions").unwrap();
    cmd.arg("invalid-command")
        .assert()
        .failure();
}