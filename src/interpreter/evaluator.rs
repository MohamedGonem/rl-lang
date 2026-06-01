use crate::{
    ast::nodes::Expression, interpreter::values::Value, lexer::tokentypes::TokenType,
    utils::errors::Error,
};

pub struct Evaluator {}

impl Evaluator {
    pub fn evaluate(&mut self, expression: &Expression) -> Value {
        match expression {
            Expression::Integer(i) => Value::Integer(i),
            Expression::String(s) => Value::String(s),
            Expression::Bool(b) => Value::Bool(b),
            Expression::Float(f) => Value::Float(f),
            Expression::Character(c) => Value::Char(c),

            Expression::Grouping(inner) => self.evaluate(inner),
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left);
                let right = self.evaluate(right);
                self.match_binary_operator(left, right, operator)
            }
            Expression::Unary { operator, operand } => {
                let operand = self.evaluate(operand);
                self.match_unary_operator(operand, operator)
            }
        }
    }

    pub fn match_unary_operator(&mut self, operand: Value, operator: &TokenType) -> Value {
        match operator {
            TokenType::Bang => match operand {
                Value::Bool(b) => Value::Bool(!b),
                _ => {
                    Error::init("type mismatch on !".to_string(), None, None).print_error();
                    unreachable!();
                }
            },
            TokenType::Minus => match operand {
                Value::Integer(i) => Value::Integer(-i),
                Value::Float(f) => Value::Float(-f),
                _ => {
                    Error::init("type mismatch on -".to_string(), None, None).print_error();
                    unreachable!();
                }
            },
            _ => {
                Error::init("some error".to_string(), None, None).print_error();
                unreachable!();
            }
        }
    }

    pub fn match_binary_operator(
        &mut self,
        left: Value,
        right: Value,
        operator: &TokenType,
    ) -> Value {
        match operator {
            TokenType::Plus => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a + b),
                (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
                _ => {
                    Error::init("type mismatch on +".to_string(), None, None).print_error();
                    unreachable!();
                }
            },
            TokenType::Minus => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a - b),
                (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
                _ => {
                    Error::init("type mismatch on -".to_string(), None, None).print_error();
                    unreachable!();
                }
            },
            TokenType::Star => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a * b),
                (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
                _ => {
                    Error::init("type mismatch on *".to_string(), None, None).print_error();
                    unreachable!();
                }
            },
            TokenType::Slash => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a / b),
                (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
                _ => {
                    Error::init("type mismatch on /".to_string(), None, None).print_error();
                    unreachable!();
                }
            },
            _ => {
                Error::init("some error".to_string(), None, None).print_error();
                unreachable!();
            }
        }
    }
}
