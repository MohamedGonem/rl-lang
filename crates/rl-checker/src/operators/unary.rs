//! Unary operator type checking.
//!
//! | Operator | Operand         | Result |
//! |----------|-----------------|--------|
//! | `!`      | bool            | bool   |
//! | `-`      | int             | int    |
//! | `-`      | float           | float  |
//!
//! `uint` deliberately has no `-` rule - negating an unsigned value doesn't
//! produce another valid `uint`, so it's rejected here as a type error
//! rather than silently wrapping or falling back to `int`.
//!
//! A unit-carrying operand keeps its unit through `-` (negation doesn't
//! change dimensionality); `!` always produces a dimensionless `bool`.

use crate::structs::{CheckedExpr, CheckType, TypeChecker};
use rl_ast::statements::TypeAnnotation;
use rl_lexer::tokentypes::TokenType;
use rl_utils::span::Span;

impl TypeChecker {
    pub fn check_unary_operator(
        &mut self,
        operand: CheckedExpr,
        _operand_span: Span,
        op: &TokenType,
        span: Span,
    ) -> CheckedExpr {
        if operand.ty.is_unknown() {
            return CheckedExpr::new(CheckType::Unknown, None);
        }
        match op {
            // is it correct bang unary?
            TokenType::Bang => match &operand.ty {
                CheckType::Known(TypeAnnotation::Bool | TypeAnnotation::CBool) => {
                    CheckedExpr::new(CheckType::Known(TypeAnnotation::Bool), None)
                }
                _ => {
                    self.error(
                        format!("type mismatch on !: got {}", operand.ty.info()),
                        span,
                    );
                    CheckedExpr::new(CheckType::Unknown, None)
                }
            },
            // is it correect minus unary?
            TokenType::Minus => match &operand.ty {
                CheckType::Known(TypeAnnotation::Int | TypeAnnotation::CInt) => {
                    // negation keeps the operand's unit
                    CheckedExpr::new(CheckType::Known(TypeAnnotation::Int), operand.unit)
                }
                CheckType::Known(TypeAnnotation::Float | TypeAnnotation::CFloat) => {
                    CheckedExpr::new(CheckType::Known(TypeAnnotation::Float), operand.unit)
                }
                CheckType::Known(TypeAnnotation::UInt | TypeAnnotation::CUInt) => {
                    self.error(
                        "cannot negate a uint value - uint has no negative range".to_string(),
                        span,
                    );
                    CheckedExpr::new(CheckType::Unknown, None)
                }
                _ => {
                    self.error(
                        format!("type mismatch on unary -: got {}", operand.ty.info()),
                        span,
                    );
                    CheckedExpr::new(CheckType::Unknown, None)
                }
            },
            // undefined unary
            _ => {
                self.error(format!("unknown unary operator {:?}", op), span);
                CheckedExpr::new(CheckType::Unknown, None)
            }
        }
    }
}