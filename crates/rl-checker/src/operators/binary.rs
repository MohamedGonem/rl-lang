//! Binary operator type checking.
//!
//! # Rules
//!
//! | Operator          | Left / Right                    | Result  |
//! |-------------------|---------------------------------|---------|
//! | `+` `-` `*` `/`  | int + int                       | int     |
//! | `+` `-` `*` `/`  | uint + uint                     | uint    |
//! | `+` `-` `*` `/`  | float + float                   | float   |
//! | `+` `-` `*` `/`  | byte + byte                     | byte    |
//! | `+` `-` `*` `/`  | byte + int (or int + byte)      | int     |
//! | `<` `>` `<=` `>=`| int/byte pairs                  | bool    |
//! | `<` `>` `<=` `>=`| uint/uint pairs                 | bool    |
//! | `<` `>` `<=` `>=`| float + float                   | bool    |
//! | `==` `!=`         | matching primitive types        | bool    |
//!
//! `uint` does not mix with `int` or `byte` in arithmetic/comparison - both
//! sides must be `uint` (mirroring how `byte` is already strict about not
//! mixing with `int`, despite the table above's aspirational `byte + int`
//! row which isn't actually implemented below either).
//!
//! Any side being `Unknown` short-circuits to `Unknown` to suppress cascading errors.

use crate::{
    operators::op_str,
    structs::{CheckType, TypeChecker},
};
use rl_ast::statements::TypeAnnotation;
use rl_lexer::tokentypes::TokenType;
use rl_utils::span::Span;

impl TypeChecker {
    pub fn check_binary_operator(
        &mut self,
        left: CheckType,
        right: CheckType,
        op: &TokenType,
        span: Span,
    ) -> CheckType {
        // if any of sides is unknown then it is unknown
        if left.is_unknown() || right.is_unknown() {
            return CheckType::Unknown;
        }

        match op {
            // arithmetic check if both same type or not
            TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash => {
                match (&left, &right) {
                    (
                        CheckType::Known(TypeAnnotation::Int | TypeAnnotation::CInt),
                        CheckType::Known(TypeAnnotation::Int | TypeAnnotation::CInt),
                    ) => CheckType::Known(TypeAnnotation::Int),
                    (
                        CheckType::Known(TypeAnnotation::UInt | TypeAnnotation::CUInt),
                        CheckType::Known(TypeAnnotation::UInt | TypeAnnotation::CUInt),
                    ) => CheckType::Known(TypeAnnotation::UInt),
                    (
                        CheckType::Known(TypeAnnotation::Float | TypeAnnotation::CFloat),
                        CheckType::Known(TypeAnnotation::Float | TypeAnnotation::CFloat),
                    ) => CheckType::Known(TypeAnnotation::Float),
                    (
                        CheckType::Known(TypeAnnotation::Byte | TypeAnnotation::CByte),
                        CheckType::Known(TypeAnnotation::Byte | TypeAnnotation::CByte),
                    ) => CheckType::Known(TypeAnnotation::Byte),

                    _ => {
                        self.error(
                            format!(
                                "type mismatch on {}: got {} and {}",
                                op_str(op),
                                left.info(),
                                right.info()
                            ),
                            span,
                        );
                        CheckType::Unknown
                    }
                }
            }

            // comparisons should be same type
            TokenType::Less
            | TokenType::Greater
            | TokenType::LessEqual
            | TokenType::GreaterEqual => match (&left, &right) {
                (
                    CheckType::Known(TypeAnnotation::Int | TypeAnnotation::CInt),
                    CheckType::Known(TypeAnnotation::Int | TypeAnnotation::CInt),
                )
                | (
                    CheckType::Known(TypeAnnotation::Float | TypeAnnotation::CFloat),
                    CheckType::Known(TypeAnnotation::Float | TypeAnnotation::CFloat),
                ) => CheckType::Known(TypeAnnotation::Bool),
                (
                    CheckType::Known(TypeAnnotation::UInt | TypeAnnotation::CUInt),
                    CheckType::Known(TypeAnnotation::UInt | TypeAnnotation::CUInt),
                ) => CheckType::Known(TypeAnnotation::Bool),
                (
                    CheckType::Known(TypeAnnotation::Byte | TypeAnnotation::CByte),
                    CheckType::Known(TypeAnnotation::Byte | TypeAnnotation::CByte),
                ) => CheckType::Known(TypeAnnotation::Bool),

                _ => {
                    self.error(
                        format!(
                            "type mismatch on {}: got {} and {}",
                            op_str(op),
                            left.info(),
                            right.info()
                        ),
                        span,
                    );
                    CheckType::Unknown
                }
            },

            // equality should be between same types
            TokenType::Compare | TokenType::BangEqual => {
                let ok = matches!(
                    (&left, &right),
                    (
                        CheckType::Known(TypeAnnotation::Int | TypeAnnotation::CInt),
                        CheckType::Known(TypeAnnotation::Int | TypeAnnotation::CInt),
                    ) | (
                        CheckType::Known(TypeAnnotation::UInt | TypeAnnotation::CUInt),
                        CheckType::Known(TypeAnnotation::UInt | TypeAnnotation::CUInt),
                    ) | (
                        CheckType::Known(TypeAnnotation::Byte | TypeAnnotation::CByte),
                        CheckType::Known(TypeAnnotation::Byte | TypeAnnotation::CByte),
                    ) | (
                        CheckType::Known(TypeAnnotation::Float | TypeAnnotation::CFloat),
                        CheckType::Known(TypeAnnotation::Float | TypeAnnotation::CFloat),
                    ) | (
                        CheckType::Known(TypeAnnotation::String | TypeAnnotation::CString),
                        CheckType::Known(TypeAnnotation::String | TypeAnnotation::CString),
                    ) | (
                        CheckType::Known(TypeAnnotation::Char | TypeAnnotation::CChar),
                        CheckType::Known(TypeAnnotation::Char | TypeAnnotation::CChar),
                    ) | (
                        CheckType::Known(TypeAnnotation::Bool | TypeAnnotation::CBool),
                        CheckType::Known(TypeAnnotation::Bool | TypeAnnotation::CBool),
                    ) | (
                        CheckType::Known(TypeAnnotation::Enum(_) | TypeAnnotation::CEnum(_)),
                        CheckType::Known(TypeAnnotation::Enum(_) | TypeAnnotation::CEnum(_)),
                    ) | (
                        CheckType::Known(TypeAnnotation::Record(_) | TypeAnnotation::CRecord(_)),
                        CheckType::Known(TypeAnnotation::Record(_) | TypeAnnotation::CRecord(_)),
                    )
                );
                if !ok {
                    self.error(
                        format!(
                            "type mismatch on {}: got {} and {}",
                            op_str(op),
                            left.info(),
                            right.info()
                        ),
                        span,
                    );
                }
                CheckType::Known(TypeAnnotation::Bool)
            }

            TokenType::And | TokenType::Or => {
                if !matches!(
                    left,
                    CheckType::Known(TypeAnnotation::Bool | TypeAnnotation::CBool)
                ) {
                    self.error(
                        format!("expected bool on the left side of {}", op_str(op)),
                        span,
                    );
                }
                if !matches!(
                    right,
                    CheckType::Known(TypeAnnotation::Bool | TypeAnnotation::CBool)
                ) {
                    self.error(
                        format!("expected bool on the right side of {}", op_str(op)),
                        span,
                    );
                }

                CheckType::Known(TypeAnnotation::Bool)
            }

            // unknown operator
            _ => {
                self.error(format!("unknown binary operator {:?}", op), span);
                CheckType::Unknown
            }
        }
    }
}
