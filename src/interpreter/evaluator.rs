use std::collections::HashMap;

use crate::{ast::nodes::Expression, interpreter::values::Value, utils::errors::Error};

pub struct Evaluator {
    enviroment: HashMap<String, Value>,
}

impl Evaluator {
    pub fn evaluate(&mut self, expression: &Expression) -> Value {
        match expression {
            Expression::Integer(i) => Value::Integer(*i),
            Expression::String(s) => Value::String(s.clone()),
            Expression::Bool(b) => Value::Bool(*b),
            Expression::Float(f) => Value::Float(*f),
            Expression::Character(c) => Value::Char(*c),

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

            Expression::Identifier(name) => self.get_value(name.clone()),
            Expression::Assign { name, value } => {
                let val = self.evaluate(value);
                self.insert_value(name.clone(), val.clone());
                val
            }
        }
    }

    pub fn new() -> Self {
        Self {
            enviroment: HashMap::new(),
        }
    }

    pub fn get_value(&self, value_name: String) -> Value {
        match self.enviroment.get(&value_name) {
            Some(val) => val.clone(),
            _ => {
                Error::init(format!("undefined variable {}", &value_name), None, None)
                    .print_error();
                unreachable!();
            }
        }
    }

    pub fn insert_value(&mut self, value_name: String, value: Value) {
        self.enviroment.insert(value_name, value);
    }
}
