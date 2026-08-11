use std::fs;

use rl_tooling::generate_docs::{DocItem, extract_doc_items, write_doc_site, write_doc_site_html};

use crate::common;

#[test]
fn extracts_function_documentation() {
    let tokens = common::lex(
        r#"/// prints hello
fn hello() {
}"#,
    );

    let items = extract_doc_items(&tokens, "main.rl");

    assert_eq!(items.len(), 1);

    let item = &items[0];

    assert_eq!(item.kind, "fn");
    assert_eq!(item.name, "hello");
    assert_eq!(item.doc, "prints hello");
    assert_eq!(item.file, "main.rl");
}

#[test]
fn ignores_regular_comments() {
    let tokens = common::lex(
        r#"// not docs
fn hello() {
}
"#,
    );

    let items = extract_doc_items(&tokens, "main.rl");
    assert!(items.is_empty())
}

#[test]
fn extracts_multiple_documented_items() {
    let tokens = common::lex(
        r#"/// first
fn one() {
}

/// second
record User {
}
        "#,
    );

    let items = extract_doc_items(&tokens, "main.rl");

    assert_eq!(items.len(), 2);

    assert_eq!(items[0].name, "one");
    assert_eq!(items[1].name, "User");
}

#[test]
fn write_markdown_docs() {
    let temp = tempfile::tempdir().unwrap();

    let items = vec![DocItem {
        kind: "fn",
        name: "hello".into(),
        signature: "fn hello()".into(),
        doc: "prints hello".into(),
        file: "main.rl".into(),
        line: 1,
    }];

    write_doc_site(&items, temp.path(), "my-project").unwrap();

    assert!(temp.path().join("index.md").exists());
    assert!(temp.path().join("main.md").exists());

    let content = std::fs::read_to_string(temp.path().join("main.md")).unwrap();

    assert!(content.contains("hello"));
    assert!(content.contains("prints hello"));
}

#[test]
fn groups_docs_by_file() {
    let temp = tempfile::tempdir().unwrap();

    let items = vec![
        DocItem {
            kind: "fn",
            name: "foo".into(),
            signature: "fn foo()".into(),
            doc: "prints foo".into(),
            file: "foo.rl".into(),
            line: 1,
        },
        DocItem {
            kind: "fn",
            name: "bar".into(),
            signature: "fn bar()".into(),
            doc: "prints bar".into(),
            file: "bar.rl".into(),
            line: 1,
        },
    ];

    write_doc_site(&items, temp.path(), "test").unwrap();

    assert!(temp.path().join("foo.md").exists());
    assert!(temp.path().join("bar.md").exists());
}

#[test]
fn writes_html_docs() {
    let temp = tempfile::tempdir().unwrap();

    let items = vec![DocItem {
        kind: "fn",
        name: "foo".into(),
        signature: "fn foo()".into(),
        doc: "prints foo".into(),
        file: "main.rl".into(),
        line: 1,
    }];

    write_doc_site_html(&items, temp.path(), "test", None, true).unwrap();

    assert!(temp.path().join("index.html").exists());
    assert!(temp.path().join("main.html").exists());
}

#[test]
fn html_output_escapes_special_chars() {
    let temp = tempfile::tempdir().unwrap();

    let items = vec![DocItem {
        kind: "fn",
        name: "test".into(),
        signature: "<bad>".into(),
        doc: "a < b".into(),
        file: "main.rl".into(),
        line: 1,
    }];

    write_doc_site_html(&items, temp.path(), "test", None, true).unwrap();

    let html = fs::read_to_string(temp.path().join("main.html")).unwrap();

    assert!(html.contains("&lt;bad&gt;"));
    assert!(html.contains("a &lt; b"));
}

#[test]
fn html_without_highlight_does_not_write_script() {
    let temp = tempfile::tempdir().unwrap();

    write_doc_site_html(&[], temp.path(), "test", None, true).unwrap();

    assert!(!temp.path().join("rl-highlight.js").exists())
}
