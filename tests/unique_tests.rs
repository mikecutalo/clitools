#[cfg(test)]
extern crate assert_cmd;

use std::process::Command;
use assert_cmd::prelude::*;

#[test]
fn test_simple_intergration() {
    let mut cmd = Command::cargo_bin("unique").unwrap();
    cmd
        .with_stdin()
        .buffer("meow\nmeow\ncat\nchoice")
        .assert()
        .code(0);
}

#[test]
fn test_empty_lines() {
    let mut cmd = Command::cargo_bin("unique").unwrap();
    cmd
        .with_stdin()
        .buffer("\n\n\nmeow\n\n\nmeow\ncat\nchoice")
        .assert()
        .code(0);
}
