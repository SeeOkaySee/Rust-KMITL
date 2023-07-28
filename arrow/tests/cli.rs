use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn test_run4() {
    let mut cmd = Command::cargo_bin("arrow").unwrap();
    let expected_output = "*\n**\n***\n****\n***\n**\n*\n";
    cmd.arg("4").assert().success().stdout(contains(expected_output));
}

#[test]
fn test_run3() {
    let mut cmd = Command::cargo_bin("arrow").unwrap();
    let expected_output = "*\n**\n***\n**\n*\n";
    cmd.arg("3").assert().success().stdout(contains(expected_output));
}

#[test]
fn test_run2() {
    let mut cmd = Command::cargo_bin("arrow").unwrap();
    let expected_output = "*\n**\n*\n";
    cmd.arg("2").assert().success().stdout(contains(expected_output));
}

#[test]
fn test_run5() {
    let mut cmd = Command::cargo_bin("arrow").unwrap();
    let expected_output = "*\n**\n***\n****\n*****\n****\n***\n**\n*\n";
    cmd.arg("5").assert().success().stdout(contains(expected_output));
}

#[test]
fn test_reverse4() {
    let mut cmd = Command::cargo_bin("reverse").unwrap();
    let expected_output = "   *\n  **\n ***\n****\n ***\n  **\n   *\n";
    cmd.arg("4").assert().success().stdout(contains(expected_output));
}

#[test]
fn test_reverse3() {
    let mut cmd = Command::cargo_bin("reverse").unwrap();
    let expected_output = "  *\n **\n***\n **\n  *\n";
    cmd.arg("3").assert().success().stdout(contains(expected_output));
}

#[test]
fn test_reverse2() {
    let mut cmd = Command::cargo_bin("reverse").unwrap();
    let expected_output = " *\n**\n *\n";
    cmd.arg("2").assert().success().stdout(contains(expected_output));
}

#[test]
fn test_reverse5() {
    let mut cmd = Command::cargo_bin("reverse").unwrap();
    let expected_output = "    *\n   **\n  ***\n ****\n*****\n ****\n  ***\n   **\n    *\n";
    cmd.arg("5").assert().success().stdout(contains(expected_output));
}
