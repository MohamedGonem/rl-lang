//! Binary operator evaluation for all supported operand type combinations.
//!
//! All type mismatches emit a labeled error pointing at both operands.

use crate::{evaluator::Evaluator, values::Value};
use rl_lexer::tokentypes::TokenType;
use rl_utils::{errors::Error, span::Span};

impl Evaluator {
    fn type_mismatch_binary(
        &self,
        op: &str,
        left: &Value,
        left_span: Span,
        right: &Value,
        right_span: Span,
        span: Span,
    ) -> Error {
        self.err(format!("type mismatch on {}", op), span)
            .with_label(left_span, format!("this is {}", left.type_name()))
            .with_label(right_span, format!("this is {}", right.type_name()))
    }

    fn error_binary(
        &self,
        op: &str,
        left: &Value,
        left_span: Span,
        right: &Value,
        right_span: Span,
        span: Span,
        result: &str,
    ) -> Error {
        self.err(format!("cannot apply operator: {}", op), span)
            .with_label(left_span, format!("this is {}", left.type_name()))
            .with_label(right_span, format!("this is {}", right.type_name()))
            .with_label(span, format!("the result would be {}", result))
    }

    pub fn match_binary_operator(
        &mut self,
        left: Value,
        left_span: Span,
        right: Value,
        right_span: Span,
        operator: &TokenType,
        span: Span,
    ) -> Result<Value, Error> {
        // Helper macro to inject types.
        macro_rules! with_numeric_types {
                ($mac:ident, $($args:tt)*) => {
                    $mac! {
                        $($args)*,
                        ints: [Integer, UInteger, SInteger, SUInteger, BByte, BSByte, Byte, SByte],
                        floats: [Float, SFloat]
                    }
                };
            }

        // Macro for standard math (+, -, *)
        macro_rules! math_op {
                (
                    $op_str:expr, $op_token:tt, $check:ident, $wrap:ident,
                    ints: [ $($int_ty:ident),* ],
                    floats: [ $($float_ty:ident),* ]
                ) => {
                    match (&left, &right) {
                        $(
                            (Value::$int_ty(a), Value::$int_ty(b)) => {
                                Value::$int_ty(a.$check(*b).ok_or_else(|| {
                                    self.error_binary(
                                        $op_str,
                                        &left,
                                        left_span,
                                        &right,
                                        right_span,
                                        span,
                                        &a.$wrap(*b).to_string(),
                                    )
                                })?)
                            }
                        )*
                        $(
                            (Value::$float_ty(a), Value::$float_ty(b)) => {
                                Value::$float_ty(*a $op_token *b)
                            }
                        )*
                        _ => return Err(self.type_mismatch_binary($op_str, &left, left_span, &right, right_span, span)),
                    }
                };
            }

        // Macro specifically for division (/) to handle division by zero checks safely
        macro_rules! div_op {
                (
                    $op_str:expr, $op_token:tt,
                    ints: [ $($int_ty:ident),* ],
                    floats: [ $($float_ty:ident),* ]
                ) => {
                    match (&left, &right) {
                        $(
                            (Value::$int_ty(a), Value::$int_ty(b)) => {
                                if *b == 0 {
                                    return Err(self.err("division by zero", span));
                                }
                                Value::$int_ty(a.checked_div(*b).ok_or_else(|| {
                                    self.error_binary(
                                        $op_str,
                                        &left,
                                        left_span,
                                        &right,
                                        right_span,
                                        span,
                                        &a.wrapping_div(*b).to_string(),
                                    )
                                })?)
                            }
                        )*
                        $(
                            (Value::$float_ty(a), Value::$float_ty(b)) => {
                                Value::$float_ty(*a $op_token *b)
                            }
                        )*
                        _ => return Err(self.type_mismatch_binary($op_str, &left, left_span, &right, right_span, span)),
                    }
                };
            }

        // Macro for relative comparisons (<, >, <=, >=)
        macro_rules! cmp_op {
                (
                    $op_str:expr, $op_token:tt,
                    ints: [ $($int_ty:ident),* ],
                    floats: [ $($float_ty:ident),* ]
                ) => {
                    match (&left, &right) {
                        $( (Value::$int_ty(a), Value::$int_ty(b)) => Value::Bool(*a $op_token *b), )*
                        $( (Value::$float_ty(a), Value::$float_ty(b)) => Value::Bool(*a $op_token *b), )*
                        _ => return Err(self.type_mismatch_binary($op_str, &left, left_span, &right, right_span, span)),
                    }
                };
            }

        // Macro for equality (==, !=) including Strings, Chars, Bools, and Enums
        macro_rules! eq_op {
                (
                    $op_str:expr, $op_token:tt,
                    ints: [ $($int_ty:ident),* ],
                    floats: [ $($float_ty:ident),* ]
                ) => {
                    match (&left, &right) {
                        $( (Value::$int_ty(a), Value::$int_ty(b)) => Value::Bool(*a $op_token *b), )*
                        $( (Value::$float_ty(a), Value::$float_ty(b)) => Value::Bool(*a $op_token *b), )*
                        (Value::String(a), Value::String(b)) => Value::Bool(a $op_token b),
                        (Value::Char(a), Value::Char(b)) => Value::Bool(a $op_token b),
                        (Value::Bool(a), Value::Bool(b)) => Value::Bool(a $op_token b),
                        (
                            Value::Enum { name: a_name, variant: a_var },
                            Value::Enum { name: b_name, variant: b_var }
                        ) => {
                            Value::Bool((a_name, a_var) $op_token (b_name, b_var))
                        },
                        _ => return Err(self.type_mismatch_binary($op_str, &left, left_span, &right, right_span, span)),
                    }
                };
            }

        let v = match operator {
            TokenType::Plus => with_numeric_types!(math_op, "+", +, checked_add, wrapping_add),
            TokenType::Minus => with_numeric_types!(math_op, "-", -, checked_sub, wrapping_sub),
            TokenType::Star => with_numeric_types!(math_op, "*", *, checked_mul, wrapping_mul),
            TokenType::Slash => with_numeric_types!(div_op, "/", /),

            TokenType::Less => with_numeric_types!(cmp_op, "<", <),
            TokenType::Greater => with_numeric_types!(cmp_op, ">", >),
            TokenType::LessEqual => with_numeric_types!(cmp_op, "<=", <=),
            TokenType::GreaterEqual => with_numeric_types!(cmp_op, ">=", >=),

            TokenType::BangEqual => with_numeric_types!(eq_op, "!=", !=),
            TokenType::Compare => with_numeric_types!(eq_op, "==", ==),

            _ => return Err(self.err("unknown binary operator", span)),
        };

        Ok(v)
    }
}
