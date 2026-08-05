use rl_tooling::workflows::try_generate;

#[test]
fn generate_check_workflow() {
    let temp = tempfile::tempdir().unwrap();

    try_generate(temp.path(), true, false).unwrap();

    let file = temp.path().join("check.yml");

    assert!(file.exists());

    let content = std::fs::read_to_string(file).unwrap();

    assert!(content.contains("name: RL Check"));
    assert!(content.contains("rl-check"));
}

#[test]
fn generate_package_workflow() {
    let temp = tempfile::tempdir().unwrap();

    try_generate(temp.path(), false, true).unwrap();

    let file = temp.path().join("release.yml");

    assert!(file.exists());

    let content = std::fs::read_to_string(file).unwrap();

    assert!(content.contains("name: Release"));
    assert!(content.contains("rl-package"));
}

#[test]
fn generate_all_workflows() {
    let temp = tempfile::tempdir().unwrap();

    try_generate(temp.path(), true, true).unwrap();

    assert!(temp.path().join("check.yml").exists());
    assert!(temp.path().join("release.yml").exists());
}

#[test]
fn does_not_override_existing_workflow() {
    let temp = tempfile::tempdir().unwrap();

    std::fs::create_dir_all(temp.path()).unwrap();

    std::fs::write(
        temp.path().join("check.yml"),
        "existing" 
    )
    .unwrap();

    let result = try_generate(temp.path(), true, false);

    assert!(result.is_err());

    let content = std::fs::read_to_string(temp.path().join("check.yml")).unwrap();

    assert_eq!(content, "existing");
}