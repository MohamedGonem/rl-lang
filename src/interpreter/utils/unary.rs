use crate::{
    interpreter::evaluator::Evaluator, interpreter::values::Value, lexer::tokentypes::TokenType,
    utils::errors::Error,
};

impl Evaluator {
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
}
