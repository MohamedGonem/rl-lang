use rl_tooling::dev::try_read_rl_toml;

#[test]
fn reads_valid_rl_toml() {
    let temp = tempfile::tempdir().unwrap();

    let file = temp.path().join("rl.toml");

    std::fs::write(
        &file, 
        r#"
[project]
name = "hello"
version = "0.0.1"
entry = "src/main.rl"
        "#
    )
    .unwrap();

    let manifest = try_read_rl_toml(&file).unwrap();

    assert_eq!(manifest.project.name, "hello");
    assert_eq!(manifest.project.version, "0.0.1");
    assert_eq!(manifest.project.entry, "src/main.rl");
}

#[test]
fn fails_when_rl_toml_does_not_exist() {
    let temp = tempfile::tempdir().unwrap();
    let file = temp.path().join("missing.toml");

    let result = try_read_rl_toml(&file);

    assert!(result.is_err());
}

#[test]
fn fails_when_rl_toml_is_invalid() {
    let temp = tempfile::tempdir().unwrap();
    let file = temp.path().join("rl.toml");

    std::fs::write(
        &file, 
        r#"
[project]
name = "hello"
        "#
    )
    .unwrap();

    let result = try_read_rl_toml(&file);

    assert!(result.is_err());
}

#[test]
fn fails_without_project_section() {
    let temp = tempfile::tempdir().unwrap();

    let file = temp.path().join("rl.toml");

    std::fs::write(
        &file, 
        r#"
name = "hello"
version = "0.0.1"
entry = "src/main.rl"
        "#
    )
    .unwrap();

    let result = try_read_rl_toml(&file);

    assert!(result.is_err());
}

#[test]
fn fails_when_project_entry_is_missing() {
    let temp = tempfile::tempdir().unwrap();

    let file = temp.path().join("rl.toml");

    std::fs::write(
        &file, 
        r#"
[project]
name = "hello"
version = "0.0.1"
        "#
    )
    .unwrap();

    let result = try_read_rl_toml(&file);
    
    assert!(result.is_err());
}