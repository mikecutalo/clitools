#[cfg(test)]
extern crate assert_cmd;

use std::process::Command;
use assert_cmd::prelude::*;

#[test]
fn test_simple_count() {
    let mut cmd = Command::cargo_bin("frequency").unwrap();
    cmd
        .with_stdin()
        .buffer("meow\nmeow\nmeow\ncat\ncat")
        .assert()
        .stdout("3 meow\n2 cat\n");
}

#[test]
fn test_empty_lines() {
    let mut cmd = Command::cargo_bin("frequency").unwrap();
    cmd
        .with_stdin()
        .buffer("\nmeow\n\nmeow\ncat")
        .assert()
        .stdout("2 meow\n1 cat\n");
}
