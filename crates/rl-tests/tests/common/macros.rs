#[macro_export]
macro_rules! assert_decl {
    (
        $source:expr,
        $variant:path,
        name: $name:expr,
        type_annotation: $ty:expr,
        value: $expr_kind:expr, $expr_span:expr,
        span: $stmt_span:expr $(,)?
    ) => {{
        let (ast, statements) = common::parse($source);
        assert_eq!(statements.len(), 1, "expected exactly one statement");
        match &statements[0].kind {
            $variant {
                name,
                type_annotation,
                value,
            } => {
                assert_eq!(name, $name);
                assert_eq!(*type_annotation, $ty);
                assert_eq!(ast.exprs.get(*value).kind, $expr_kind);
                assert_eq!(ast.exprs.get(*value).span, $expr_span);
            }
            other => panic!("expected {}, got {:?}", stringify!($variant), other),
        }
        assert_eq!(statements[0].span, $stmt_span);
    }};
    (
        $source:expr,
        $variant:path,
        name: $name:expr,
        type_annotation: $ty:expr,
        value: $expr_kind:expr $(,)?
    ) => {{
        let (ast, statements) = common::parse($source);
        assert_eq!(statements.len(), 1, "expected exactly one statement");
        match &statements[0].kind {
            $variant {
                name,
                type_annotation,
                value,
            } => {
                assert_eq!(name, $name);
                assert_eq!(*type_annotation, $ty);
                assert_eq!(ast.exprs.get(*value).kind, $expr_kind);
            }
            other => panic!("expected {}, got {:?}", stringify!($variant), other),
        }
    }};
}

#[macro_export]
macro_rules! assert_stmt {
    ($source:expr, $expected_kind:expr, $stmt_span:expr $(,)?) => {{
        let (_ast, statements) = common::parse($source);
        assert_eq!(statements.len(), 1, "expected exactly one statement");
        assert_eq!(statements[0].kind, $expected_kind);
        assert_eq!(statements[0].span, $stmt_span);
    }};
    ($source:expr, $expected_kind:expr $(,)?) => {{
        let (_ast, statements) = common::parse($source);
        assert_eq!(statements.len(), 1, "expected exactly one statement");
        assert_eq!(statements[0].kind, $expected_kind);
    }};
}

#[macro_export]
macro_rules! assert_while {
    (
        $source:expr,
        condition: $cond_kind:expr, $cond_span:expr, grouped: $group_span:expr,
        body_expr: $body_kind:expr, $body_expr_span:expr, $body_stmt_span:expr,
        span: $stmt_span:expr $(,)?
    ) => {{
        let (ast, statements) = common::parse($source);
        assert_eq!(statements.len(), 1, "expected exactly one statement");
        match &statements[0].kind {
            rl_ast::statements::StatementKind::While { condition, body } => {
                common::assert_grouping(&ast, *condition, $cond_kind, $cond_span, $group_span);
                assert_eq!(body.len(), 1, "expected exactly one body statement");
                common::assert_single_expr_stmt(
                    &body[0],
                    &ast,
                    $body_kind,
                    $body_expr_span,
                    $body_stmt_span,
                );
            }
            other => panic!("expected While, got {:?}", other),
        }
        assert_eq!(statements[0].span, $stmt_span);
    }};
    (
        $source:expr,
        condition: $cond_kind:expr,
        body_expr: $body_kind:expr $(,)?
    ) => {{
        let (ast, statements) = common::parse($source);
        assert_eq!(statements.len(), 1, "expected exactly one statement");
        match &statements[0].kind {
            rl_ast::statements::StatementKind::While { condition, body } => {
                common::assert_grouping_no_span(&ast, *condition, $cond_kind);
                assert_eq!(body.len(), 1, "expected exactly one body statement");
                common::assert_single_expr_stmt_no_span(&body[0], &ast, $body_kind);
            }
            other => panic!("expected While, got {:?}", other),
        }
    }};
}

#[macro_export]
macro_rules! assert_for_range {
    (
        $source:expr,
        variable: $var:expr,
        range: $range_vals:expr, $range_span:expr,
        body_expr: $body_kind:expr, $body_expr_span:expr, $body_stmt_span:expr,
        span: $stmt_span:expr $(,)?
    ) => {{
        let (ast, statements) = common::parse($source);
        assert_eq!(statements.len(), 1, "expected exactly one statement");
        match &statements[0].kind {
            rl_ast::statements::StatementKind::ForRange {
                variable,
                range,
                body,
            } => {
                assert_eq!(variable, $var);
                match &range.kind {
                    rl_ast::statements::StatementKind::Range(vals) => {
                        assert_eq!(*vals, $range_vals);
                    }
                    other => panic!("expected Range, got {:?}", other),
                }
                assert_eq!(range.span, $range_span);
                assert_eq!(body.len(), 1, "expected exactly one body statement");
                common::assert_single_expr_stmt(
                    &body[0],
                    &ast,
                    $body_kind,
                    $body_expr_span,
                    $body_stmt_span,
                );
            }
            other => panic!("expected ForRange, got {:?}", other),
        }
        assert_eq!(statements[0].span, $stmt_span);
    }};
}

#[macro_export]
macro_rules! assert_array_decl {
    (
        $source:expr,
        $variant:path,
        name: $name:expr,
        type_annotation: $ty:expr,
        item: $item_kind:expr, $item_span:expr,
        span: $stmt_span:expr $(,)?
    ) => {{
        let (ast, statements) = common::parse($source);
        assert_eq!(statements.len(), 1, "expected exactly one statement");
        match &statements[0].kind {
            $variant {
                name,
                type_annotation,
                value,
            } => {
                assert_eq!(name, $name);
                assert_eq!(*type_annotation, $ty);
                assert_eq!(value.len(), 1, "expected exactly one array item");
                common::assert_expr(&ast, value[0], $item_kind, $item_span);
            }
            other => panic!("expected {}, got {:?}", stringify!($variant), other),
        }
        assert_eq!(statements[0].span, $stmt_span);
    }};
}
