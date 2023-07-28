use assert_cmd::Command;

#[test]
fn grade_S() {
    let mut cmd = Command::cargo_bin("grading").unwrap();
    cmd.arg("97");
    cmd.assert().success().stdout("Excellent with A+\n");
}

#[test]
fn grade_A() {
    let mut cmd = Command::cargo_bin("grading").unwrap();
    cmd.arg("85");
    cmd.assert().success().stdout("A\n");
}

#[test]
fn grade_B() {
    let mut cmd = Command::cargo_bin("grading").unwrap();
    cmd.arg("78");
    cmd.assert().success().stdout("B\n");
}

#[test]
fn grade_C() {
    let mut cmd = Command::cargo_bin("grading").unwrap();
    cmd.arg("66");
    cmd.assert().success().stdout("C\n");
}

#[test]
fn grade_D() {
    let mut cmd = Command::cargo_bin("grading").unwrap();
    cmd.arg("55");
    cmd.assert().success().stdout("D\n");
}

#[test]
fn grade_F() {
    let mut cmd = Command::cargo_bin("grading").unwrap();
    cmd.arg("6");
    cmd.assert().success().stdout("Failed with F\n");
}

#[test]
fn grade_err() {
    let mut cmd = Command::cargo_bin("grading").unwrap();
    cmd.arg("289");
    cmd.assert().success().stdout("Invalid score\n");
}