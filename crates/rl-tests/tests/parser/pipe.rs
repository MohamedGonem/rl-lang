use rl_ast::nodes::ExpressionKind;
use rl_ast::statements::StatementKind;

use crate::common::{self, span_of, span_whole};

#[test]
fn pipe_simple_function_call() {
    let source = "x |> foo()";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
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
            assert_eq!(args.len(), 0);
        }
        other => panic!("expected MethodCall, got {:?}", other),
    }
}

#[test]
fn pipe_function_call_with_args() {
    let source = "x |> foo(1, 2)";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
    let expr_id = match &statements[0].kind {
        StatementKind::Expression(id) => *id,
        other => panic!("expected Expression statement, got {:?}", other),
    };
    let expr = ast.exprs.get(expr_id);
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
            assert_eq!(args.len(), 2);
            common::assert_expr(&ast, args[0], ExpressionKind::Integer(1), span_of(source, "1"));
            common::assert_expr(&ast, args[1], ExpressionKind::Integer(2), span_of(source, "2"));
        }
        other => panic!("expected MethodCall, got {:?}", other),
    }
}

#[test]
fn pipe_on_string_literal() {
    let source = "\"hello\" |> foo()";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
    let expr_id = match &statements[0].kind {
        StatementKind::Expression(id) => *id,
        other => panic!("expected Expression statement, got {:?}", other),
    };
    let expr = ast.exprs.get(expr_id);
    match &expr.kind {
        ExpressionKind::MethodCall {
            caller,
            method,
            args,
        } => {
            common::assert_expr(
                &ast,
                *caller,
                ExpressionKind::String("hello".to_string()),
                span_of(source, "\"hello\""),
            );
            assert_eq!(method, &vec!["foo".to_string()]);
            assert_eq!(args.len(), 0);
        }
        other => panic!("expected MethodCall, got {:?}", other),
    }
}

#[test]
fn pipe_chained() {
    let source = "x |> foo() |> bar()";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
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
            assert_eq!(method, &vec!["bar".to_string()]);
            assert_eq!(args.len(), 0);
            // The caller should be the result of x |> foo()
            let inner = ast.exprs.get(*caller);
            match &inner.kind {
                ExpressionKind::MethodCall {
                    caller: inner_caller,
                    method: inner_method,
                    args: inner_args,
                } => {
                    common::assert_expr(
                        &ast,
                        *inner_caller,
                        ExpressionKind::Identifier("x".to_string()),
                        span_of(source, "x"),
                    );
                    assert_eq!(inner_method, &vec!["foo".to_string()]);
                    assert_eq!(inner_args.len(), 0);
                }
                other => panic!("expected inner MethodCall, got {:?}", other),
            }
        }
        other => panic!("expected MethodCall, got {:?}", other),
    }
}

#[test]
fn pipe_preserves_rhs_call_args() {
    let source = "x |> add(1)";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
    let expr_id = match &statements[0].kind {
        StatementKind::Expression(id) => *id,
        other => panic!("expected Expression statement, got {:?}", other),
    };
    let expr = ast.exprs.get(expr_id);
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
            assert_eq!(method, &vec!["add".to_string()]);
            assert_eq!(args.len(), 1);
            common::assert_expr(&ast, args[0], ExpressionKind::Integer(1), span_of(source, "1"));
        }
        other => panic!("expected MethodCall, got {:?}", other),
    }
}

#[test]
fn pipe_rhs_not_call_is_error() {
    let source = "x |> y";
    let err = common::parse_assert_err(source);
    assert!(
        err.contains("function or method call"),
        "expected error about function or method call, got: {}",
        err
    );
}

#[test]
fn pipe_rhs_method_call() {
    let source = "x |> y.foo()";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
    let expr_id = match &statements[0].kind {
        StatementKind::Expression(id) => *id,
        other => panic!("expected Expression statement, got {:?}", other),
    };
    let expr = ast.exprs.get(expr_id);
    match &expr.kind {
        ExpressionKind::MethodCall {
            caller,
            method,
            args,
        } => {
            // y.foo() becomes y.foo(x)
            assert_eq!(method, &vec!["foo".to_string()]);
            assert_eq!(args.len(), 1);
            common::assert_expr(
                &ast,
                args[0],
                ExpressionKind::Identifier("x".to_string()),
                span_of(source, "x"),
            );
            // caller is y
            common::assert_expr(
                &ast,
                *caller,
                ExpressionKind::Identifier("y".to_string()),
                span_of(source, "y"),
            );
        }
        other => panic!("expected MethodCall, got {:?}", other),
    }
}

#[test]
fn pipe_in_assignment() {
    let source = "dec int val = x |> foo()";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
    match &statements[0].kind {
        StatementKind::VariableDeclaration { value, .. } => {
            let expr = ast.exprs.get(*value);
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
                    assert_eq!(args.len(), 0);
                }
                other => panic!("expected MethodCall, got {:?}", other),
            }
        }
        other => panic!("expected VariableDeclaration, got {:?}", other),
    }
}
