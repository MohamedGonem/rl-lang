use crate::common::checker_messages;

#[test]
fn unimported_bare_call_errors() {
    let msgs = checker_messages(r#"println("hello")"#);
    assert!(
        msgs.iter().any(|m| m.contains("import")),
        "expected an 'import it before use' error, got: {msgs:?}"
    );
}

#[test]
fn imported_bare_call_passes() {
    let msgs = checker_messages("get println from std::io\nprintln(\"hello\")");
    assert!(msgs.is_empty(), "expected no errors, got: {msgs:?}");
}

#[test]
fn fully_qualified_path_passes_without_import() {
    let msgs = checker_messages(r#"std::io::println("hello")"#);
    assert!(msgs.is_empty(), "expected no errors, got: {msgs:?}");
}

#[test]
fn import_unknown_module_errors() {
    let msgs = checker_messages("get x from std::nope");
    assert!(
        msgs.iter().any(|m| m.contains("unknown module")),
        "expected an 'unknown module' error, got: {msgs:?}"
    );
}

#[test]
fn import_unknown_name_errors() {
    let msgs = checker_messages("get nope from std::io");
    assert!(
        msgs.iter().any(|m| m.contains("not defined")),
        "expected a 'not defined' error, got: {msgs:?}"
    );
}

#[test]
fn user_function_shadows_unimported_stdlib_bare_call() {
    let msgs = checker_messages("fn println(string s) {\n}\nprintln(\"hello\")");
    assert!(msgs.is_empty(), "expected no errors, got: {msgs:?}");
}
