use rl_ast::{
    nodes::ExpressionKind,
    statements::{FunctionAttribute, Param, StatementKind, TypeAnnotation},
};

use crate::common::{self, span_of, span_of_last, span_whole};

#[test]
fn fn_simple() {
    let source = "fn x (int x) {return x}";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    match &statements[0].kind {
        StatementKind::FunctionDeclaration {
            name,
            params,
            return_type,
            attribute,
            body,
        } => {
            assert_eq!(name, "x");
            assert_eq!(
                params,
                &vec![Param {
                    param_name: "x".to_string(),
                    param_type: TypeAnnotation::Int,
                }]
            );
            assert_eq!(*return_type, TypeAnnotation::Null);
            assert_eq!(*attribute, None);
            assert_eq!(body.len(), 1, "expected exactly one body statement");
            common::assert_return(
                &body[0],
                &ast,
                Some((
                    ExpressionKind::Identifier("x".to_string()),
                    span_of_last(source, "x"),
                )),
                span_of(source, "return x"),
            );
        }
        other => panic!("expected FunctionDeclaration, got {:?}", other),
    }
    assert_eq!(statements[0].span, span_whole(source));
}

#[test]
fn fn_fn_param() {
    let source = "fn x (fn x, int y) {return x(y)}";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    match &statements[0].kind {
        StatementKind::FunctionDeclaration {
            name,
            params,
            return_type,
            attribute,
            body,
        } => {
            assert_eq!(name, "x");
            assert_eq!(
                params,
                &vec![
                    Param {
                        param_name: "x".to_string(),
                        param_type: TypeAnnotation::Fn
                    },
                    Param {
                        param_name: "y".to_string(),
                        param_type: TypeAnnotation::Int
                    },
                ]
            );
            assert_eq!(*return_type, TypeAnnotation::Null);
            assert_eq!(*attribute, None);
            assert_eq!(body.len(), 1, "expected exactly one body statement");

            assert_eq!(body[0].span, span_of(source, "return x(y)"));
            match &body[0].kind {
                StatementKind::Return(Some(id)) => {
                    let expr = ast.exprs.get(*id);
                    assert_eq!(expr.span, span_of(source, "x(y)"));
                    match &expr.kind {
                        ExpressionKind::Call { path, args } => {
                            assert_eq!(path, &vec!["x".to_string()]);
                            assert_eq!(args.len(), 1);
                            common::assert_expr(
                                &ast,
                                args[0],
                                ExpressionKind::Identifier("y".to_string()),
                                span_of_last(source, "y"),
                            );
                        }
                        other => panic!("expected Call, got {:?}", other),
                    }
                }
                other => panic!("expected Return(Some(_)), got {:?}", other),
            }
        }
        other => panic!("expected FunctionDeclaration, got {:?}", other),
    }
    assert_eq!(statements[0].span, span_whole(source));
}

#[test]
fn dec_fn_lambda() {
    let source = "dec fn x = fn(int x) {return x}";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1, "expected exactly one statement");
    match &statements[0].kind {
        StatementKind::VariableDeclaration {
            name,
            type_annotation,
            value,
        } => {
            assert_eq!(name, "x");
            assert_eq!(*type_annotation, TypeAnnotation::Fn);

            let expr = ast.exprs.get(*value);
            assert_eq!(expr.span, span_of(source, "fn(int x) {return x}"));
            match &expr.kind {
                ExpressionKind::Lambda {
                    params,
                    return_type,
                    body,
                } => {
                    assert_eq!(
                        params,
                        &vec![Param {
                            param_name: "x".to_string(),
                            param_type: TypeAnnotation::Int
                        }]
                    );
                    assert_eq!(*return_type, None);
                    assert_eq!(body.len(), 1, "expected exactly one body statement");
                    common::assert_return(
                        &body[0],
                        &ast,
                        Some((
                            ExpressionKind::Identifier("x".to_string()),
                            span_of_last(source, "x"),
                        )),
                        span_of(source, "return x"),
                    );
                }
                other => panic!("expected Lambda, got {:?}", other),
            }
        }
        other => panic!("expected VariableDeclaration, got {:?}", other),
    }
    assert_eq!(statements[0].span, span_whole(source));
}

#[test]
fn entry_attribute_marks_function() {
    let (_ast, statements) = common::parse("!#[entry]\nfn start () {return 1}");
    match &statements[0].kind {
        StatementKind::FunctionDeclaration {
            name, attribute, ..
        } => {
            assert_eq!(name, "start");
            assert_eq!(*attribute, Some(FunctionAttribute::Entry));
        }
        other => panic!("expected function declaration, got {:?}", other),
    }
}
