use crate::parser_logic::Parser;
use rl_ast::{ExprId, nodes::ExpressionKind};
use rl_lexer::tokentypes::TokenType;
use rl_utils::errors::Error;

impl Parser {
    /// Parses unary prefix expressions: `!` (logical not) and `-` (negation).
    ///
    /// Right-associative by recursion: `--x` parses as `-(-(x))`.
    /// Falls through to [`parse_primary`] when no prefix operator is present.
    ///
    /// # Negative integer literals
    /// `NumberLiteral` is unsigned (`u64`) - a bare `-` in front of one is
    /// special-cased here and folded directly into a signed
    /// [`ExpressionKind::Integer`], rather than the general `Unary { Minus, .. }`
    /// node `parse_primary` would otherwise produce. This is what allows
    /// `i64::MIN` (`-9223372036854775808`) to parse at all: its magnitude
    /// (`9223372036854775808`, i.e. `i64::MAX + 1`) is one past what
    /// `parse_primary`'s standalone bounds check allows, since that check has
    /// no way to know a `-` is coming. Folding sign and magnitude together
    /// before bounds-checking sidesteps that.
    ///
    /// This fold only applies to a bare literal with no `as` cast following
    /// it. `-5 as uint` is deliberately left as a normal `Unary` node so the
    /// cast is parsed as casting `5` to `uint` first - which is then rejected
    /// as its own error (negating an unsigned value doesn't make sense), not
    /// silently folded into a negative literal.
    ///
    /// [`parse_primary`]: Parser::parse_primary
    pub fn parse_unary(&mut self) -> Result<ExprId, Error> {
        let start = self.peek_span();

        if self.match_type(&[TokenType::Minus]) {
            if let TokenType::NumberLiteral(n) = self.peek()
                && self.peek_next() != TokenType::As
            {
                self.advance(); // consume the NumberLiteral
                let span = start.join(self.previous_span());
                let Some(negated) = negate_u64(n) else {
                    return Err(self.err(
                        format!(
                            "value -{} is out of range for int ({}..={})",
                            n,
                            i64::MIN,
                            i64::MAX
                        ),
                        span,
                    ));
                };
                let expr = self
                    .ast_arena
                    .alloc_expr(ExpressionKind::Integer(negated), span);
                return self.parse_postfix(expr, start);
            }

            let operator = self.previous();
            let operand = self.parse_unary()?;
            let operand_id = self.ast_arena.exprs.get(operand);
            let span = start.join(operand_id.span);
            return Ok(self
                .ast_arena
                .alloc_expr(ExpressionKind::Unary { operator, operand }, span));
        }

        if self.match_type(&[TokenType::Bang]) {
            let operator = self.previous();
            let operand = self.parse_unary()?;
            let operand_id = self.ast_arena.exprs.get(operand);
            let span = start.join(operand_id.span);
            return Ok(self
                .ast_arena
                .alloc_expr(ExpressionKind::Unary { operator, operand }, span));
        }

        self.parse_primary()
    }
}

/// Negates an unsigned literal magnitude into a signed `i64`, allowing the
/// one extra value (`i64::MAX + 1`) that only exists as `i64::MIN`.
///
/// Returns `None` if the magnitude is too large to be represented as any
/// signed `i64`, even negated.
fn negate_u64(n: u64) -> Option<i64> {
    if n <= i64::MAX as u64 {
        Some(-(n as i64))
    } else if n == i64::MAX as u64 + 1 {
        Some(i64::MIN)
    } else {
        None
    }
}
