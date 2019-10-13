#[cfg(test)]
extern crate assert_cmd;

use std::process::Command;
use assert_cmd::prelude::*;

#[test]
fn test_column_parsing() {
    let mut cmd = Command::cargo_bin("cslice").unwrap();
    cmd.arg("0");
    cmd
        .with_stdin()
        .buffer("meow cat dog")
        .assert()
        .stdout("meow\n");
}

#[test]
fn test_column_parsing_plus_skip() {
    let mut cmd = Command::cargo_bin("cslice").unwrap();
    cmd.args(&["1", "1"]);
    cmd
        .with_stdin()
        .buffer("meow cat dog\nchoice not drp")
        .assert()
        .stdout("not\n");
}

#[test]
fn test_emptylines_input() {
    let mut cmd = Command::cargo_bin("cslice").unwrap();
    cmd.arg("0");
    cmd
        .with_stdin()
        .buffer("\n\n\nmeow cat dog")
        .assert()
        .stdout("meow\n");
}

#[test]
fn test_limit_output_option() {
    let mut cmd = Command::cargo_bin("cslice").unwrap();
    cmd.args(&["0", "--limit", "1"]);
    cmd
        .with_stdin()
        .buffer("meow cat dog\nchoice not blah")
        .assert()
        .stdout("meow\n");
}
