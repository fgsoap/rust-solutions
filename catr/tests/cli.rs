use assert_cmd::Command;
#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("catr").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}
