use rl_tooling::package::bundle;

#[test]
fn bundle_simple_source_file() {
    let temp = tempfile::tempdir().unwrap();

    let file = temp.path().join("main.rl");

    std::fs::write(&file, "fn main() {\n}\n").unwrap();

    let output = bundle(file.to_str().unwrap()).unwrap();

    assert!(output.contains("fn main()"));
}

#[test]
fn bundles_local_imports() {
    let temp = tempfile::tempdir().unwrap();

    std::fs::write(temp.path().join("utils.rl"), "fn helper() {}\n").unwrap();

    std::fs::write(temp.path().join("main.rl"), "get utils\nfn main() {}\n").unwrap();

    let output = bundle(temp.path().join("main.rl").to_str().unwrap()).unwrap();

    assert!(output.contains("fn helper()"));
    assert!(output.contains("fn main()"));
}

#[test]
fn does_not_bundle_std_imports() {
    let temp = tempfile::tempdir().unwrap();

    let main = temp.path().join("main.rl");

    std::fs::write(&main, "get println from std::io").unwrap();

    let output = bundle(main.to_str().unwrap()).unwrap();

    assert!(output.contains("get println from std::io"));
}

#[test]
fn does_not_duplicate_imports() {
    let temp = tempfile::tempdir().unwrap();

    std::fs::write(temp.path().join("utils.rl"), "fn helper() {}\n").unwrap();

    std::fs::write(temp.path().join("main.rl"), "get utils\nget utils\n").unwrap();

    let output = bundle(temp.path().join("main.rl").to_str().unwrap()).unwrap();

    assert_eq!(output.matches("fn helper").count(), 1);
}
