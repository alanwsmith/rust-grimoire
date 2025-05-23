use assert_cmd::Command;

#[test]
fn test_tone() {
    let mut cmd = Command::cargo_bin("basic-test-setup").unwrap();
    cmd.assert().stdout("Hello, App!\n");
}
