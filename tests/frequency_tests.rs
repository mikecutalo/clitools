#[cfg(test)]
extern crate assert_cmd;

use assert_cmd::Command;

#[test]
fn test_simple_count() {
    let mut cmd = Command::cargo_bin("frequency").unwrap();
    cmd
        .write_stdin("meow\nmeow\nmeow\ncat\ncat")
        .assert()
        .stdout("3 meow\n2 cat\n");
}

#[test]
fn test_empty_lines() {
    let mut cmd = Command::cargo_bin("frequency").unwrap();
    cmd
        .write_stdin("\nmeow\n\nmeow\ncat")
        .assert()
        .stdout("2 meow\n1 cat\n");
}
