use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn bytes_megabytes() {
    let mut cmd = Command::cargo_bin("convertx").unwrap();
    cmd.args(&["bytes", "1048576", "--megabytes"]);
    cmd.assert().success().stdout(contains("1.00 MB"));
}
