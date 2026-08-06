use crate::tooling::common::create_test_project;

#[test]
fn create_project_creates_directory() {
    let (_temp, project) = create_test_project("my-project", true);
    assert!(project.exists());
}

#[test]
fn create_project_creates_rl_toml() {
    let (_temp, project) = create_test_project("my-project", true);
    assert!(project.join("rl.toml").exists());
}

#[test]
fn create_project_creates_main_file() {
    let (_temp, project) = create_test_project("test", true);
    assert!(project.join("src/main.rl").exists());
}

#[test]
fn create_project_generates_correct_manifest() {
    let (_temp, project) = create_test_project("test", true);
    let content = std::fs::read_to_string(project.join("rl.toml")).unwrap();

    assert!(content.contains(&format!("name = \"{}\"", project.to_str().unwrap())));
    assert!(content.contains("entry = \"src/main.rl\""));
}

#[test]
fn create_project_without_git_does_not_create_git_directory() {
    let (_temp, project) = create_test_project("hello", true);
    assert!(!project.join(".git").exists());
}

#[test]
fn create_project_with_git_creates_git_directory() {
    let (_temp, project) = create_test_project("hello", false);
    assert!(project.join(".git").exists());
}
