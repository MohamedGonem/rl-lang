use rl_ast::{
    nodes::ExpressionKind,
    statements::{StatementKind, TypeAnnotation},
};

use crate::common;
use crate::assert_decl;

#[test]
fn dec_int_with_semicolon() {
    let source = "dec int x = 10;";
    assert_decl!(
        source,
        StatementKind::VariableDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::Int,
        value: ExpressionKind::Integer(10),
    );
}

#[test]
fn dec_int_without_semicolon() {
    let source = "dec int x = 10";
    assert_decl!(
        source,
        StatementKind::VariableDeclaration,
        name: "x",
        type_annotation: TypeAnnotation::Int,
        value: ExpressionKind::Integer(10),
    );
}

#[test]
fn const_int_with_semicolon() {
    let source = "CONST int X = 42;";
    assert_decl!(
        source,
        StatementKind::ConstantDeclaration,
        name: "X",
        type_annotation: TypeAnnotation::CInt,
        value: ExpressionKind::Integer(42),
    );
}

#[test]
fn expression_statement_with_semicolon() {
    let source = "100";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
    match &statements[0].kind {
        StatementKind::Expression(expr_id) => {
            assert_eq!(ast.exprs.get(*expr_id).kind, ExpressionKind::Integer(100));
        }
        other => panic!("expected Expression, got {:?}", other),
    }
}

#[test]
fn return_with_semicolon() {
    let source = "fn main() {\nreturn 5;\n}";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
    match &statements[0].kind {
        StatementKind::FunctionDeclaration { body, .. } => {
            assert_eq!(body.len(), 1);
            match &body[0].kind {
                StatementKind::Return(Some(expr_id)) => {
                    assert_eq!(ast.exprs.get(*expr_id).kind, ExpressionKind::Integer(5));
                }
                other => panic!("expected Return(Some), got {:?}", other),
            }
        }
        other => panic!("expected FunctionDeclaration, got {:?}", other),
    }
}

#[test]
fn break_with_semicolon() {
    let source = "while (true) {\nbreak;\n}";
    let (_ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
    match &statements[0].kind {
        StatementKind::While { body, .. } => {
            assert_eq!(body.len(), 1);
            assert_eq!(body[0].kind, StatementKind::Break);
        }
        other => panic!("expected While, got {:?}", other),
    }
}

#[test]
fn continue_with_semicolon() {
    let source = "while (true) {\ncontinue;\n}";
    let (_ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
    match &statements[0].kind {
        StatementKind::While { body, .. } => {
            assert_eq!(body.len(), 1);
            assert_eq!(body[0].kind, StatementKind::Continue);
        }
        other => panic!("expected While, got {:?}", other),
    }
}

#[test]
fn if_block_with_semicolon() {
    let source = "if (true) {10};";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
    match &statements[0].kind {
        StatementKind::Conditional { if_branch, .. } => match &if_branch.kind {
            StatementKind::ConditionalBranch { body, .. } => {
                assert_eq!(body.len(), 1);
                match &body[0].kind {
                    StatementKind::Expression(expr_id) => {
                        assert_eq!(ast.exprs.get(*expr_id).kind, ExpressionKind::Integer(10));
                    }
                    other => panic!("expected Expression, got {:?}", other),
                }
            }
            other => panic!("expected ConditionalBranch, got {:?}", other),
        },
        other => panic!("expected Conditional, got {:?}", other),
    }
}

#[test]
fn while_block_with_semicolon() {
    let source = "while (true) {0};";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
    match &statements[0].kind {
        StatementKind::While { body, .. } => {
            assert_eq!(body.len(), 1);
            match &body[0].kind {
                StatementKind::Expression(expr_id) => {
                    assert_eq!(ast.exprs.get(*expr_id).kind, ExpressionKind::Integer(0));
                }
                other => panic!("expected Expression, got {:?}", other),
            }
        }
        other => panic!("expected While, got {:?}", other),
    }
}

#[test]
fn for_block_with_semicolon() {
    let source = "for [int i = 1, i < 10, i += 1] {0};";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
    match &statements[0].kind {
        StatementKind::For { body, .. } => {
            assert_eq!(body.len(), 1);
            match &body[0].kind {
                StatementKind::Expression(expr_id) => {
                    assert_eq!(ast.exprs.get(*expr_id).kind, ExpressionKind::Integer(0));
                }
                other => panic!("expected Expression, got {:?}", other),
            }
        }
        other => panic!("expected For, got {:?}", other),
    }
}

#[test]
fn fn_declaration_with_semicolon() {
    let source = "fn x () {0};";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
    match &statements[0].kind {
        StatementKind::FunctionDeclaration { body, .. } => {
            assert_eq!(body.len(), 1);
            match &body[0].kind {
                StatementKind::Expression(expr_id) => {
                    assert_eq!(ast.exprs.get(*expr_id).kind, ExpressionKind::Integer(0));
                }
                other => panic!("expected Expression, got {:?}", other),
            }
        }
        other => panic!("expected FunctionDeclaration, got {:?}", other),
    }
}

#[test]
fn multiple_statements_with_semicolons() {
    let source = "dec int a = 1;\ndec int b = 2;";
    let (_ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 2);
    match &statements[0].kind {
        StatementKind::VariableDeclaration { name, .. } => assert_eq!(name, "a"),
        other => panic!("expected VariableDeclaration, got {:?}", other),
    }
    match &statements[1].kind {
        StatementKind::VariableDeclaration { name, .. } => assert_eq!(name, "b"),
        other => panic!("expected VariableDeclaration, got {:?}", other),
    }
}

#[test]
fn mixed_semicolons_and_newlines() {
    let source = "dec int a = 1;\ndec int b = 2\ndec int c = 3;";
    let (_ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 3);
    match &statements[0].kind {
        StatementKind::VariableDeclaration { name, .. } => assert_eq!(name, "a"),
        other => panic!("expected VariableDeclaration, got {:?}", other),
    }
    match &statements[1].kind {
        StatementKind::VariableDeclaration { name, .. } => assert_eq!(name, "b"),
        other => panic!("expected VariableDeclaration, got {:?}", other),
    }
    match &statements[2].kind {
        StatementKind::VariableDeclaration { name, .. } => assert_eq!(name, "c"),
        other => panic!("expected VariableDeclaration, got {:?}", other),
    }
}

#[test]
fn semicolons_inside_block() {
    let source = "fn main() {\ndec int a = 1;\ndec int b = 2;\nreturn a;\n}";
    let (ast, statements) = common::parse(source);
    assert_eq!(statements.len(), 1);
    match &statements[0].kind {
        StatementKind::FunctionDeclaration { body, .. } => {
            assert_eq!(body.len(), 3);
            match &body[0].kind {
                StatementKind::VariableDeclaration { name, .. } => assert_eq!(name, "a"),
                other => panic!("expected VariableDeclaration, got {:?}", other),
            }
            match &body[1].kind {
                StatementKind::VariableDeclaration { name, .. } => assert_eq!(name, "b"),
                other => panic!("expected VariableDeclaration, got {:?}", other),
            }
            match &body[2].kind {
                StatementKind::Return(Some(expr_id)) => {
                    assert_eq!(
                        ast.exprs.get(*expr_id).kind,
                        ExpressionKind::Identifier("a".into())
                    );
                }
                other => panic!("expected Return(Some), got {:?}", other),
            }
        }
        other => panic!("expected FunctionDeclaration, got {:?}", other),
    }
}
