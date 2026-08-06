use rl_tooling::format::format_tokens;

use crate::common;


#[test]
fn no_space_after_opening_tokens() {
    let tokens = common::lex("( x )");

    assert_eq!(format_tokens(&tokens), "(x)");
}

#[test]
fn no_space_before_closing_tokens() {
    let tokens = common::lex("foo( x )");

    assert_eq!(format_tokens(&tokens), "foo(x)");
}

#[test]
fn no_space_before_comma() {
    let tokens = common::lex("foo(a , b)");
    
    assert_eq!(format_tokens(&tokens), "foo(a, b)");
}

#[test]
fn index_access_has_no_space() {
    let tokens = common::lex("arr [0]");

    assert_eq!(format_tokens(&tokens), "arr[0]");
}