use crate::common;

#[test]
fn missing_closing_bracket_in_index() {
    let err = common::parse_assert_err("dec int x = myArr[0");
    assert!(
        !err.is_empty(),
        "expected a parse error for missing `]` in index"
    );
}

#[test]
fn missing_closing_bracket_in_chained_index() {
    let err = common::parse_assert_err("dec int x = myArr[0][1");
    assert!(
        !err.is_empty(),
        "expected a parse error for missing `]` in chained index"
    );
}

#[test]
fn missing_closing_bracket_in_array_literal() {
    let err = common::parse_assert_err("dec int x = [1, 2, 3");
    assert!(
        !err.is_empty(),
        "expected a parse error for missing `]` in array literal"
    );
}

#[test]
fn missing_closing_brace_in_map_literal() {
    let err = common::parse_assert_err("dec int x = {\"a\": 1, \"b\": 2");
    assert!(
        !err.is_empty(),
        "expected a parse error for missing `}}` in map literal"
    );
}

#[test]
fn missing_closing_brace_in_set_literal() {
    let err = common::parse_assert_err("dec int x = {1, 2, 3");
    assert!(
        !err.is_empty(),
        "expected a parse error for missing `}}` in set literal"
    );
}

#[test]
fn missing_closing_paren_in_grouped_expression() {
    let err = common::parse_assert_err("dec int x = (1 + 2");
    assert!(
        !err.is_empty(),
        "expected a parse error for missing `)` in grouped expression"
    );
}

#[test]
fn missing_closing_paren_in_function_params() {
    let err = common::parse_assert_err("fn foo(int x {return x}");
    assert!(
        err.contains("expected `)`")
            || err.contains("expected `,`")
            || err.contains("expected parameter name"),
        "unexpected error: {err}"
    );
}

#[test]
fn missing_opening_paren_in_function() {
    let err = common::parse_assert_err("fn foo int x) {return x}");
    assert!(
        err.contains("expected `(` after function name"),
        "unexpected error: {err}"
    );
}

#[test]
fn missing_opening_paren_in_lambda() {
    let err = common::parse_assert_err("dec fn f = fn int x) {return x}");
    assert!(
        err.contains("expected `(` after `fn`"),
        "unexpected error: {err}"
    );
}

#[test]
fn missing_closing_paren_in_lambda_params() {
    let err = common::parse_assert_err("dec fn f = fn(int x {return x}");
    assert!(
        err.contains("expected `)`")
            || err.contains("expected `,`")
            || err.contains("expected parameter name"),
        "unexpected error: {err}"
    );
}

#[test]
fn missing_closing_paren_in_method_params() {
    let err = common::parse_assert_err("record Point { int x, int y }\nimpl Point {\n    fn new(int x, int y Point {\n        return Point { x: x, y: y }\n    }\n}");
    assert!(
        err.contains("expected `)`")
            || err.contains("expected `,`")
            || err.contains("expected parameter name"),
        "unexpected error: {err}"
    );
}

#[test]
fn missing_closing_bracket_in_dec_array_literal() {
    let err = common::parse_assert_err("dec array[int] x = [1, 2, 3");
    assert!(
        !err.is_empty(),
        "expected a parse error for missing `]` in dec array literal"
    );
}

#[test]
fn missing_closing_brace_in_dec_map_literal() {
    let err = common::parse_assert_err("dec map[string,int] x = {\"a\": 1");
    assert!(
        !err.is_empty(),
        "expected a parse error for missing `}}` in dec map literal"
    );
}

#[test]
fn missing_closing_brace_in_dec_set_literal() {
    let err = common::parse_assert_err("dec set[int] x = {1, 2");
    assert!(
        !err.is_empty(),
        "expected a parse error for missing `}}` in dec set literal"
    );
}

#[test]
fn missing_closing_bracket_in_const_array_literal() {
    let err = common::parse_assert_err("CONST array[int] X = [1, 2, 3");
    assert!(
        !err.is_empty(),
        "expected a parse error for missing `]` in const array literal"
    );
}

#[test]
fn missing_closing_brace_in_const_map_literal() {
    let err = common::parse_assert_err("CONST map[string,int] X = {\"a\": 1");
    assert!(
        !err.is_empty(),
        "expected a parse error for missing `}}` in const map literal"
    );
}

#[test]
fn missing_closing_brace_in_const_set_literal() {
    let err = common::parse_assert_err("CONST set[int] X = {1, 2");
    assert!(
        !err.is_empty(),
        "expected a parse error for missing `}}` in const set literal"
    );
}

#[test]
fn missing_closing_bracket_in_for_header() {
    let err = common::parse_assert_err("for [int i = 0, i < 10, i += 1 {return i}");
    assert!(
        err.contains("expected `]` in for header"),
        "unexpected error: {err}"
    );
}

#[test]
fn missing_dotdot_in_for_range() {
    let err = common::parse_assert_err("for i in 0 10 {return i}");
    assert!(
        err.contains("expected `..` in range"),
        "unexpected error: {err}"
    );
}

#[test]
fn missing_closing_bracket_in_for_inline_array() {
    let err = common::parse_assert_err("for i in [1, 2, 3 {return i}");
    assert!(
        !err.is_empty(),
        "expected a parse error for missing `]` in for inline array"
    );
}

#[test]
fn missing_bracket_in_arr_type() {
    let err = common::parse_assert_err("dec arr int] x = [1]");
    assert!(
        err.contains("expected `[` after `array`")
            || err.contains("expected `=` after name")
            || err.contains("expected a type"),
        "unexpected error: {err}"
    );
}

#[test]
fn missing_closing_bracket_in_arr_type() {
    let err = common::parse_assert_err("dec arr[int x = [1]");
    assert!(
        err.contains("expected `]` after array type")
            || err.contains("expected `,` or `)` in tuple type")
            || err.contains("expected a type"),
        "unexpected error: {err}"
    );
}

#[test]
fn missing_bracket_in_map_type() {
    let err = common::parse_assert_err("dec map string,int] x = {}");
    assert!(
        err.contains("expected `[` after `map`")
            || err.contains("expected `=` after name")
            || err.contains("expected a type"),
        "unexpected error: {err}"
    );
}

#[test]
fn missing_closing_bracket_in_map_type() {
    let err = common::parse_assert_err("dec map[string,int x = {}");
    assert!(
        err.contains("expected `]` after map type")
            || err.contains("expected `,` or `)` in tuple type")
            || err.contains("expected a type"),
        "unexpected error: {err}"
    );
}

#[test]
fn missing_bracket_in_set_type() {
    let err = common::parse_assert_err("dec set int] x = {}");
    assert!(
        err.contains("expected `[` after `set`")
            || err.contains("expected `=` after name")
            || err.contains("expected a type"),
        "unexpected error: {err}"
    );
}

#[test]
fn missing_closing_bracket_in_set_type() {
    let err = common::parse_assert_err("dec set[int x = {}");
    assert!(
        err.contains("expected `]` after set type")
            || err.contains("expected `,` or `)` in tuple type")
            || err.contains("expected a type"),
        "unexpected error: {err}"
    );
}

#[test]
fn empty_array_missing_bracket() {
    let err = common::parse_assert_err("dec int x = [");
    assert!(
        !err.is_empty(),
        "expected a parse error for unclosed array literal"
    );
}

#[test]
fn empty_map_missing_brace() {
    let err = common::parse_assert_err("dec int x = {");
    assert!(
        !err.is_empty(),
        "expected a parse error for unclosed map literal"
    );
}

#[test]
fn empty_parens_missing_close() {
    let err = common::parse_assert_err("dec int x = (");
    assert!(
        !err.is_empty(),
        "expected a parse error for unclosed grouped expression"
    );
}
