use rl_ast::nodes::ExpressionKind;

use crate::common::{self, span_of, span_whole};
use crate::assert_while;

#[test]
fn while_loop() {
    let source = "while (true) {0}";
    assert_while!(
        source,
        condition: ExpressionKind::Bool(true), span_of(source, "true"), grouped: span_of(source, "(true)"),
        body_expr: ExpressionKind::Integer(0), span_of(source, "0"), span_of(source, "0"),
        span: span_whole(source),
    );
}
