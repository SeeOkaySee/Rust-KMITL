use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("circle_calc").unwrap();
    cmd.arg("4");
    cmd.assert().success().stdout("circumference is 50.2656\n");
}