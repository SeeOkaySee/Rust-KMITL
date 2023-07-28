use assert_cmd::Command;

#[test]
fn run4() {
    let mut cmd= Command::cargo_bin("triangle").unwrap();
    cmd.arg("4");
    cmd.assert().success().stdout("*\n**\n***\n****\n");
}

#[test]
fn run3() {
    let mut cmd= Command::cargo_bin("triangle").unwrap();
    cmd.arg("3");
    cmd.assert().success().stdout("*\n**\n***\n");
}

#[test]
fn run2() {
    let mut cmd= Command::cargo_bin("triangle").unwrap();
    cmd.arg("2");
    cmd.assert().success().stdout("*\n**\n");
}

#[test]
fn run5() {
    let mut cmd= Command::cargo_bin("triangle").unwrap();
    cmd.arg("5");
    cmd.assert().success().stdout("*\n**\n***\n****\n*****\n");
}

#[test]
fn run7() {
    let mut cmd= Command::cargo_bin("triangle").unwrap();
    cmd.arg("7");
    cmd.assert().success().stdout("*\n**\n***\n****\n*****\n******\n*******\n");
}