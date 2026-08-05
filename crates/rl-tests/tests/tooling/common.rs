use rl_tooling::new::try_create_project;
use std::path::PathBuf;

pub fn create_test_project(name: &str, no_git: bool) -> (tempfile::TempDir, PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let project = temp.path().join(name);

    try_create_project(project.to_str().unwrap(), no_git).unwrap();

    (temp, project)
}