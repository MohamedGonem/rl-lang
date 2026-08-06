use rl_tooling::format::format_tokens;

use crate::common;

#[test]
fn identifier_before_minus_is_binary_operator() {
    let tokens = common::lex("x-10");

    assert_eq!(format_tokens(&tokens), "x - 10");
}

#[test]
fn literal_before_minus_is_binary_operator() {
    let tokens = common::lex("10-5");

    assert_eq!(format_tokens(&tokens), "10 - 5");
}

#[test]
fn operator_before_minus_is_unary_operator() {
    let tokens = common::lex("10+-5");

    assert_eq!(format_tokens(&tokens), "10 + -5");
}
