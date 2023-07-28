use assert_cmd::Command;
use float_cmp::approx_eq;

#[test]
fn runs_increasing() {
    let mut cmd = Command::cargo_bin("temptable").unwrap();
    cmd.arg("0").arg("50").arg("10");
    cmd.assert().success().stdout("Fahr\tCelcius\n   0\t-17.8\n  10\t-12.2\n  20\t-6.7\n  30\t-1.1\n  40\t 4.4\n  50\t10.0\n");
}

#[test]
fn runs_decreasing() {
    let mut cmd = Command::cargo_bin("temptable").unwrap();
    cmd.arg("100").arg("0").arg("20");
    cmd.assert().success().stdout("Fahr\tCelcius\n 100\t37.8\n  80\t26.7\n  60\t15.6\n  40\t 4.4\n  20\t-6.7\n   0\t-17.8\n");
}

#[test]
fn runs_approxeq() {
    assert!(approx_eq!(f32, temptable::conv(80), 26.7 ,epsilon = 0.1));
}

#[test]
fn runs_approxeq2() {
    assert!(approx_eq!(f32, temptable::conv(100), 37.7 ,epsilon = 0.1));
}