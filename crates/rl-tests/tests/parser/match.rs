use {rl_ast::nodes::ExpressionKind, rl_ast::statements::StatementKind};

use crate::common::{self, span_of, span_of_last, span_whole};

#[test]
fn match_literal_and_wildcard() {
    // "match x { 1 => {0} _ => {1} }"
    let source = "match x { 1 => {0} _ => {1} }";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    match &statements[0].kind {
        StatementKind::Match { value, arms } => {
            common::assert_expr(
                &ast,
                *value,
                ExpressionKind::Identifier("x".to_string()),
                span_of(source, "x"),
            );
            assert_eq!(arms.len(), 2, "expected exactly two arms");
            common::assert_match_arm(
                &arms[0],
                &ast,
                Some((ExpressionKind::Integer(1), span_of(source, "1"))),
                (
                    ExpressionKind::Integer(0),
                    span_of(source, "0"),
                    span_of(source, "0"),
                ),
            );
            common::assert_match_arm(
                &arms[1],
                &ast,
                None,
                (
                    ExpressionKind::Integer(1),
                    span_of_last(source, "1"),
                    span_of_last(source, "1"),
                ),
            );
        }
        other => panic!("expected Match, got {:?}", other),
    }
    assert_eq!(statements[0].span, span_whole(source));
}

#[test]
fn match_multiple_literals() {
    // "match x { 1 => {0} 2 => {1} }"
    let source = "match x { 1 => {0} 2 => {1} }";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    match &statements[0].kind {
        StatementKind::Match { value, arms } => {
            common::assert_expr(
                &ast,
                *value,
                ExpressionKind::Identifier("x".to_string()),
                span_of(source, "x"),
            );
            assert_eq!(arms.len(), 2, "expected exactly two arms");
            common::assert_match_arm(
                &arms[0],
                &ast,
                Some((ExpressionKind::Integer(1), span_of(source, "1"))),
                (
                    ExpressionKind::Integer(0),
                    span_of(source, "0"),
                    span_of(source, "0"),
                ),
            );
            common::assert_match_arm(
                &arms[1],
                &ast,
                Some((ExpressionKind::Integer(2), span_of(source, "2"))),
                (
                    ExpressionKind::Integer(1),
                    span_of_last(source, "1"),
                    span_of_last(source, "1"),
                ),
            );
        }
        other => panic!("expected Match, got {:?}", other),
    }
    assert_eq!(statements[0].span, span_whole(source));
}
