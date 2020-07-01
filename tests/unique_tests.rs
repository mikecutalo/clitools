#[cfg(test)]
extern crate assert_cmd;

use assert_cmd::Command;

#[test]
fn test_simple_intergration() {
    let mut cmd = Command::cargo_bin("unique").unwrap();
    cmd
        .write_stdin("meow\nmeow\ncat\nchoice")
        .assert()
        .code(0);
}

#[test]
fn test_empty_lines() {
    let mut cmd = Command::cargo_bin("unique").unwrap();
    cmd
        .write_stdin("\n\n\nmeow\n\n\nmeow\ncat\nchoice")
        .assert()
        .code(0);
}
