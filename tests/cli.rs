use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_hostid"))
}

fn temporary_config(contents: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "myhostid-test-{}-{unique}.conf",
        std::process::id()
    ));
    fs::write(&path, contents).unwrap();
    path
}

fn run_with_config(config: &PathBuf) -> Output {
    Command::new(binary())
        .env("MYHOSTID_CONFIG", config)
        .output()
        .unwrap()
}

#[test]
fn prints_the_configured_value_like_hostid() {
    let config = temporary_config("BED0745A0764\n");
    let output = run_with_config(&config);
    fs::remove_file(config).unwrap();

    assert!(output.status.success());
    assert_eq!(output.stdout, b"BED0745A0764\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn reports_a_missing_configuration() {
    let path = std::env::temp_dir().join(format!(
        "myhostid-missing-{}-{}.conf",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let output = run_with_config(&path);

    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("cannot read"));
}

#[test]
fn supports_help_without_reading_configuration() {
    let output = Command::new(binary()).arg("--help").output().unwrap();

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).starts_with("Usage: hostid [OPTION]"));
    assert!(output.stderr.is_empty());
}

#[test]
fn rejects_an_extra_operand() {
    let output = Command::new(binary()).arg("unexpected").output().unwrap();

    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("extra operand 'unexpected'"));
}
