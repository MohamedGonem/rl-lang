use rl_ast::nodes::ExpressionKind;
use rl_ast::statements::MatchPattern;
use rl_ast::statements::Statement;
use rl_ast::statements::StatementKind;
use rl_ast::{Ast, ExprId};
use rl_lexer::tokentypes::TokenType;
use rl_utils::span::Span;

pub fn assert_expr(ast: &Ast, id: ExprId, kind: ExpressionKind, span: Span) {
    let expr = ast.exprs.get(id);
    assert_eq!(expr.kind, kind);
    assert_eq!(expr.span, span);
}

pub fn assert_expr_no_span(ast: &Ast, id: ExprId, kind: ExpressionKind) {
    let expr = ast.exprs.get(id);
    assert_eq!(expr.kind, kind);
}

/// Checks `id` is a `Grouping` wrapping `inner_kind`/`inner_span`, and that
/// the grouping expression itself spans `outer_span`.
pub fn assert_grouping(
    ast: &Ast,
    id: ExprId,
    inner_kind: ExpressionKind,
    inner_span: Span,
    outer_span: Span,
) {
    let expr = ast.exprs.get(id);
    assert_eq!(expr.span, outer_span);
    match &expr.kind {
        ExpressionKind::Grouping(inner_id) => assert_expr(ast, *inner_id, inner_kind, inner_span),
        other => panic!("expected Grouping, got {:?}", other),
    }
}

#[allow(dead_code)]
pub fn assert_grouping_no_span(ast: &Ast, id: ExprId, inner_kind: ExpressionKind) {
    let expr = ast.exprs.get(id);
    match &expr.kind {
        ExpressionKind::Grouping(inner_id) => assert_expr_no_span(ast, *inner_id, inner_kind),
        other => panic!("expected Grouping, got {:?}", other),
    }
}

/// Checks `stmt` is a single bare-expression statement wrapping `kind`/`expr_span`.
pub fn assert_single_expr_stmt(
    stmt: &Statement,
    ast: &Ast,
    kind: ExpressionKind,
    expr_span: Span,
    stmt_span: Span,
) {
    assert_eq!(stmt.span, stmt_span);
    match &stmt.kind {
        StatementKind::Expression(id) => assert_expr(ast, *id, kind, expr_span),
        other => panic!("expected Expression statement, got {:?}", other),
    }
}

#[allow(dead_code)]
pub fn assert_single_expr_stmt_no_span(stmt: &Statement, ast: &Ast, kind: ExpressionKind) {
    match &stmt.kind {
        StatementKind::Expression(id) => assert_expr_no_span(ast, *id, kind),
        other => panic!("expected Expression statement, got {:?}", other),
    }
}

pub fn assert_binary(
    ast: &Ast,
    id: ExprId,
    left: (ExpressionKind, Span),
    operator: TokenType,
    right_kind: ExpressionKind,
    right_span: Span,
    span: Span,
) {
    let (left_kind, left_span) = left;
    let expr = ast.exprs.get(id);
    assert_eq!(expr.span, span);
    match &expr.kind {
        ExpressionKind::Binary {
            left,
            operator: op,
            right,
        } => {
            assert_expr(ast, *left, left_kind, left_span);
            assert_eq!(*op, operator);
            assert_expr(ast, *right, right_kind, right_span);
        }
        other => panic!("expected Binary, got {:?}", other),
    }
}

#[allow(dead_code)]
pub fn assert_binary_no_span(
    ast: &Ast,
    id: ExprId,
    left_kind: ExpressionKind,
    operator: TokenType,
    right_kind: ExpressionKind,
) {
    let expr = ast.exprs.get(id);
    match &expr.kind {
        ExpressionKind::Binary {
            left,
            operator: op,
            right,
        } => {
            assert_expr_no_span(ast, *left, left_kind);
            assert_eq!(*op, operator);
            assert_expr_no_span(ast, *right, right_kind);
        }
        other => panic!("expected Binary, got {:?}", other),
    }
}

/// Checks an `Assign { name, value }` expression, delegating the value check
/// to a closure since the value is often itself a nested Binary/etc.
pub fn assert_assign(
    ast: &Ast,
    id: ExprId,
    name: &str,
    span: Span,
    check_value: impl FnOnce(&Ast, ExprId),
) {
    let expr = ast.exprs.get(id);
    assert_eq!(expr.span, span);
    match &expr.kind {
        ExpressionKind::Assign { name: n, value } => {
            assert_eq!(n, name);
            check_value(ast, *value);
        }
        other => panic!("expected Assign, got {:?}", other),
    }
}

/// Checks a `Return(expr)` statement. `expected` is `None` for bare `return`,
/// or `Some((kind, span))` for `return <expr>`.
pub fn assert_return(
    stmt: &Statement,
    ast: &Ast,
    expected: Option<(ExpressionKind, Span)>,
    stmt_span: Span,
) {
    assert_eq!(stmt.span, stmt_span);
    match &stmt.kind {
        StatementKind::Return(expr) => match (expr, expected) {
            (Some(id), Some((kind, span))) => assert_expr(ast, *id, kind, span),
            (None, None) => {}
            (got, want) => panic!(
                "return mismatch: got {:?}, expected present = {}",
                got,
                want.is_some()
            ),
        },
        other => panic!("expected Return, got {:?}", other),
    }
}

/// Checks a `ConditionalBranch { condition, body }` statement where the
/// condition (if present) is a parenthesised `Grouping`, and the body is
/// exactly one bare-expression statement.
pub fn assert_branch(
    stmt: &Statement,
    ast: &Ast,
    condition: Option<(ExpressionKind, Span, Span)>,
    body_expr: (ExpressionKind, Span, Span),
    branch_span: Span,
) {
    assert_eq!(stmt.span, branch_span);
    match &stmt.kind {
        StatementKind::ConditionalBranch {
            condition: cond,
            body,
            ..
        } => {
            match (cond, condition) {
                (Some(id), Some((inner_kind, inner_span, group_span))) => {
                    assert_grouping(ast, *id, inner_kind, inner_span, group_span);
                }
                (None, None) => {}
                (got, want) => panic!(
                    "condition mismatch: got present = {}, expected present = {}",
                    got.is_some(),
                    want.is_some()
                ),
            }
            assert_eq!(body.len(), 1, "expected exactly one body statement");
            let (kind, expr_span, stmt_span) = body_expr;
            assert_single_expr_stmt(&body[0], ast, kind, expr_span, stmt_span);
        }
        other => panic!("expected ConditionalBranch, got {:?}", other),
    }
}

/// Checks one `(MatchPattern, Vec<Statement>)` match arm. `pattern` is
/// `None` for a wildcard (`_`) arm, or `Some((kind, span))` for a literal
/// pattern. Body must be exactly one bare-expression statement.
pub fn assert_match_arm(
    arm: &(MatchPattern, Vec<Statement>),
    ast: &Ast,
    pattern: Option<(ExpressionKind, Span)>,
    body_expr: (ExpressionKind, Span, Span),
) {
    match (&arm.0, pattern) {
        (MatchPattern::Literal(id), Some((kind, span))) => assert_expr(ast, *id, kind, span),
        (MatchPattern::Wildcard, None) => {}
        (got, want) => panic!(
            "match pattern mismatch: got {:?}, expected literal-present = {}",
            got,
            want.is_some()
        ),
    }
    assert_eq!(arm.1.len(), 1, "expected exactly one body statement in arm");
    let (kind, expr_span, stmt_span) = body_expr;
    assert_single_expr_stmt(&arm.1[0], ast, kind, expr_span, stmt_span);
}
