#[cfg(test)]
extern crate assert_cmd;

use assert_cmd::Command;

#[test]
fn test_column_parsing() {
    let mut cmd = Command::cargo_bin("cslice").unwrap();
    cmd
        .arg("0")
        .write_stdin("meow cat dog")
        .assert()
        .stdout("meow\n");
}

#[test]
fn test_column_parsing_plus_skip() {
    let mut cmd = Command::cargo_bin("cslice").unwrap();
    cmd
        .args(&["1", "1"])
        .write_stdin("meow cat dog\nchoice not drp")
        .assert()
        .stdout("not\n");
}

#[test]
fn test_empty_lines() {
    let mut cmd = Command::cargo_bin("cslice").unwrap();
    cmd
        .arg("0")
        .write_stdin("\n\n\nmeow cat dog")
        .assert()
        .stdout("meow\n");
}

#[test]
fn test_limit_output_option() {
    let mut cmd = Command::cargo_bin("cslice").unwrap();
    cmd
        .args(&["0", "--limit", "1"])
        .write_stdin("meow cat dog\nchoice not blah")
        .assert()
        .stdout("meow\n");
}
