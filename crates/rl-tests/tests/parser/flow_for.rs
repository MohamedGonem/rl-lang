use {
    rl_ast::{
        nodes::ExpressionKind,
        statements::{StatementKind, TypeAnnotation},
    },
    rl_lexer::tokentypes::TokenType,
};

use crate::assert_for_range;
use crate::common::{self, span_of, span_of_last, span_of_nth, span_whole};

#[test]
fn for_c() {
    let source = "for [int i = 1, i < 10, i += 1] {0}";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    match &statements[0].kind {
        StatementKind::For {
            initializer,
            condition,
            increment,
            body,
        } => {
            match &initializer.kind {
                StatementKind::VariableDeclaration {
                    name,
                    type_annotation,
                    value,
                    ..
                } => {
                    assert_eq!(name, "i");
                    assert_eq!(*type_annotation, TypeAnnotation::Int);
                    common::assert_expr(
                        &ast,
                        *value,
                        ExpressionKind::Integer(1),
                        span_of(source, "1"),
                    );
                }
                other => panic!("expected VariableDeclaration, got {:?}", other),
            }
            assert_eq!(initializer.span, span_of(source, "int i = 1"));

            common::assert_binary(
                &ast,
                *condition,
                (
                    ExpressionKind::Identifier("i".to_string()),
                    span_of_nth(source, "i", 2),
                ),
                TokenType::Less,
                ExpressionKind::Integer(10),
                span_of(source, "10"),
                span_of(source, "i < 10"),
            );

            common::assert_assign(
                &ast,
                *increment,
                "i",
                span_of(source, "i += 1"),
                |ast, value_id| {
                    common::assert_binary(
                        ast,
                        value_id,
                        (
                            ExpressionKind::Identifier("i".to_string()),
                            span_of_nth(source, "i", 3),
                        ),
                        TokenType::Plus,
                        ExpressionKind::Integer(1),
                        span_of_last(source, "1"),
                        span_of(source, "i += 1"),
                    );
                },
            );

            assert_eq!(body.len(), 1, "expected exactly one body statement");
            common::assert_single_expr_stmt(
                &body[0],
                &ast,
                ExpressionKind::Integer(0),
                span_of_last(source, "0"),
                span_of_last(source, "0"),
            );
        }
        other => panic!("expected For, got {:?}", other),
    }
    assert_eq!(statements[0].span, span_whole(source));
}

#[test]
fn for_range() {
    let source = "for i in 1..10 {0}";
    assert_for_range!(
        source,
        variable: "i",
        range: vec![1, 2, 3, 4, 5, 6, 7, 8, 9], span_of(source, "for i in 1..10"),
        body_expr: ExpressionKind::Integer(0), span_of_last(source, "0"), span_of_last(source, "0"),
        span: span_whole(source),
    );
}

#[test]
fn for_iterable() {
    let source = "for i in [1,2,3,4,5,6,7,8,9] {0}";
    assert_for_range!(
        source,
        variable: "i",
        range: vec![1, 2, 3, 4, 5, 6, 7, 8, 9], span_of(source, "for i in [1,2,3,4,5,6,7,8,9]"),
        body_expr: ExpressionKind::Integer(0), span_of(source, "0"), span_of(source, "0"),
        span: span_whole(source),
    );
}
