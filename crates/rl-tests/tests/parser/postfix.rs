use rl_ast::{
    nodes::ExpressionKind,
    statements::{StatementKind, TypeAnnotation},
};

use crate::common::{self, span_of, span_whole};

#[test]
fn index_simple() {
    let source = "arx[0]";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    let expr_id = match &statements[0].kind {
        StatementKind::Expression(id) => *id,
        other => panic!("expected Expression statement, got {:?}", other),
    };
    let expr = ast.exprs.get(expr_id);
    assert_eq!(expr.span, span_whole(source));
    match &expr.kind {
        ExpressionKind::Index { target, index } => {
            common::assert_expr(
                &ast,
                *target,
                ExpressionKind::Identifier("arx".to_string()),
                span_of(source, "arx"),
            );
            common::assert_expr(
                &ast,
                *index,
                ExpressionKind::Integer(0),
                span_of(source, "0"),
            );
        }
        other => panic!("expected Index, got {:?}", other),
    }
    assert_eq!(statements[0].span, span_whole(source));
}

#[test]
fn index_chained() {
    let source = "arx[0][1]";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    let expr_id = match &statements[0].kind {
        StatementKind::Expression(id) => *id,
        other => panic!("expected Expression statement, got {:?}", other),
    };
    let expr = ast.exprs.get(expr_id);
    assert_eq!(expr.span, span_whole(source));
    match &expr.kind {
        ExpressionKind::Index { target, index } => {
            common::assert_expr(
                &ast,
                *index,
                ExpressionKind::Integer(1),
                span_of(source, "1"),
            );
            let inner = ast.exprs.get(*target);
            assert_eq!(inner.span, span_of(source, "arx[0]"));
            match &inner.kind {
                ExpressionKind::Index { target, index } => {
                    common::assert_expr(
                        &ast,
                        *target,
                        ExpressionKind::Identifier("arx".to_string()),
                        span_of(source, "arx"),
                    );
                    common::assert_expr(
                        &ast,
                        *index,
                        ExpressionKind::Integer(0),
                        span_of(source, "0"),
                    );
                }
                other => panic!("expected inner Index, got {:?}", other),
            }
        }
        other => panic!("expected Index, got {:?}", other),
    }
    assert_eq!(statements[0].span, span_whole(source));
}

#[test]
fn index_assign() {
    let source = "arx[0] = 1";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    let expr_id = match &statements[0].kind {
        StatementKind::Expression(id) => *id,
        other => panic!("expected Expression statement, got {:?}", other),
    };
    let expr = ast.exprs.get(expr_id);
    assert_eq!(expr.span, span_whole(source));
    match &expr.kind {
        ExpressionKind::IndexAssign {
            target,
            index,
            value,
        } => {
            common::assert_expr(
                &ast,
                *target,
                ExpressionKind::Identifier("arx".to_string()),
                span_of(source, "arx"),
            );
            common::assert_expr(
                &ast,
                *index,
                ExpressionKind::Integer(0),
                span_of(source, "0"),
            );
            common::assert_expr(
                &ast,
                *value,
                ExpressionKind::Integer(1),
                span_of(source, "1"),
            );
        }
        other => panic!("expected IndexAssign, got {:?}", other),
    }
    assert_eq!(statements[0].span, span_whole(source));
}

#[test]
fn method_call_simple() {
    let source = "x.foo(1)";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    let expr_id = match &statements[0].kind {
        StatementKind::Expression(id) => *id,
        other => panic!("expected Expression statement, got {:?}", other),
    };
    let expr = ast.exprs.get(expr_id);
    assert_eq!(expr.span, span_whole(source));
    match &expr.kind {
        ExpressionKind::MethodCall {
            caller,
            method,
            args,
        } => {
            common::assert_expr(
                &ast,
                *caller,
                ExpressionKind::Identifier("x".to_string()),
                span_of(source, "x"),
            );
            assert_eq!(method, &vec!["foo".to_string()]);
            assert_eq!(args.len(), 1);
            common::assert_expr(
                &ast,
                args[0],
                ExpressionKind::Integer(1),
                span_of(source, "1"),
            );
        }
        other => panic!("expected MethodCall, got {:?}", other),
    }
    assert_eq!(statements[0].span, span_whole(source));
}

#[test]
fn cast_postfix() {
    let source = "x as int";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    let expr_id = match &statements[0].kind {
        StatementKind::Expression(id) => *id,
        other => panic!("expected Expression statement, got {:?}", other),
    };
    let expr = ast.exprs.get(expr_id);
    assert_eq!(expr.span, span_whole(source));
    match &expr.kind {
        ExpressionKind::Cast { value, target_type } => {
            assert_eq!(*target_type, TypeAnnotation::Int);
            common::assert_expr(
                &ast,
                *value,
                ExpressionKind::Identifier("x".to_string()),
                span_of(source, "x"),
            );
        }
        other => panic!("expected Cast, got {:?}", other),
    }
    assert_eq!(statements[0].span, span_whole(source));
}

#[test]
fn propagate_operator() {
    let source = "x?";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    let expr_id = match &statements[0].kind {
        StatementKind::Expression(id) => *id,
        other => panic!("expected Expression statement, got {:?}", other),
    };
    let expr = ast.exprs.get(expr_id);
    assert_eq!(expr.span, span_whole(source));
    match &expr.kind {
        ExpressionKind::Propagate(inner) => {
            common::assert_expr(
                &ast,
                *inner,
                ExpressionKind::Identifier("x".to_string()),
                span_of(source, "x"),
            );
        }
        other => panic!("expected Propagate, got {:?}", other),
    }
    assert_eq!(statements[0].span, span_whole(source));
}
