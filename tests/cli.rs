use std::fs;
use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echo_rs")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error: The following required arguments were not provided:"))
        .stderr(predicate::str::contains("<TEXT>"));

    Ok(())
}

#[test]
fn single_word_output() -> TestResult {
    let mut cmd = Command::cargo_bin("echo_rs")?;
    cmd.arg("hello")
        .assert()
        .success()
        .stdout("hello\n");

    Ok(())
}

#[test]
fn multiple_word_output() -> TestResult {
    let mut cmd = Command::cargo_bin("echo_rs")?;
    cmd.args(&["hello", "world"])
        .assert()
        .success()
        .stdout("hello world\n");

    Ok(())
}

#[test]
fn omit_newline() -> TestResult {
    let mut cmd = Command::cargo_bin("echo_rs")?;
    cmd.args(&["-n", "hello"])
        .assert()
        .success()
        .stdout("hello");

    Ok(())
}

#[test]
fn matches_echos_output1() -> TestResult {
    let outfile = "hello1.txt";

    run(&["Hello there"], outfile)
}

#[test]
fn matches_echos_output2() -> TestResult {
    let outfile = "hello2.txt";

    run(&["Hello", "there"], outfile)
}

#[test]
fn matches_echos_output1_no_newline() -> TestResult {
    let outfile = "hello1.n.txt";

    run(&["Hello", "there", "-n"], outfile)
}

#[test]
fn matches_echos_output2_no_newline() -> TestResult {
    let outfile = "hello2.n.txt";

    run(&["Hello", "there", "-n"], outfile)
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let outfile = format!("tests/expected/{}", expected_file);
    let expected = fs::read_to_string(outfile)?;
    let mut cmd = Command::cargo_bin("echo_rs")?;

    cmd.args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}