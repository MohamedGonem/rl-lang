use rl_tooling::format::format_tokens;

use crate::common;

#[test]
fn format_simple_expression() {
    let tokens = common::lex(" dec  int x =  1000");
    let out = format_tokens(&tokens);
    assert_eq!(out, "dec int x = 1000");
}

#[test]
fn format_function_call() {
    let tokens = common::lex("println    (\"hello\")");
    let out = format_tokens(&tokens);
    assert_eq!(out, "println(\"hello\")");
}

#[test]
fn format_indentation_with_braces() {
    let tokens = common::lex(r#"fn main(){
println("hello")
}"#
    );

    let out = format_tokens(&tokens);

    assert_eq!(
        out,
        r#"fn main() {
    println("hello")
}"#
    )
}

#[test]
fn format_nested_blocks() {
    let tokens = common::lex(r#"if x {
if y {
hello()
}
}"#
    );

    let out = format_tokens(&tokens);
    assert_eq!(
        out,
        r#"if x {
    if y {
        hello()
    }
}"#
    );
}

#[test]
fn format_line_comments() {
    let tokens = common::lex(r#"//hello
dec int x = 1"#
    );
    
    let out = format_tokens(&tokens);
    assert_eq!(
        out,
        r#"
// hello
dec int x = 1"#
    );
}

#[test]
fn format_doc_comments() {
    let tokens = common::lex(r#"/// docs
fn main()"#);

    let out = format_tokens(&tokens);
    assert_eq!(
        out,
        r#"
/// docs
fn main()"#
    );
}

#[test]
fn format_blank_lines() {
    let tokens = common::lex(r#"
    
    
    
dec int x = 1"#);

    let out = format_tokens(&tokens);

    assert_eq!(
        out,
        r#"

dec int x = 1"#
    );
}

#[test]
fn format_unary_minus() {
    let tokens = common::lex("x = - 10");
    
    let out = format_tokens(&tokens);

    assert_eq!(
        out,
        "x = -10"
    );
}

#[test]
fn format_binary_minus() {
    let tokens = common::lex("x = 10-5");
    
    let out = format_tokens(&tokens);

    assert_eq!(
        out,
        "x = 10 - 5"
    );
}

#[test]
fn format_array_indexing() {
    let tokens = common::lex("arr [0]");

    let out = format_tokens(&tokens);

    assert_eq!(
        out,
        "arr[0]"
    );
}

#[test]
fn format_generic_type_brackets() {
    let tokens = common::lex("arr [int]");

    let out = format_tokens(&tokens);

    assert_eq!(
        out,
        "arr[int]"
    );
}

#[test]
fn format_function_call_with_arguments() {
    let tokens = common::lex("foo(1,2)");

    let out = format_tokens(&tokens);

    assert_eq!(
        out,
        "foo(1, 2)"
    );
}