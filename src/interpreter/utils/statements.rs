use crate::{ast::statements::Statement, interpreter::evaluator::Evaluator, utils::errors::Error};

impl Evaluator {
    pub fn evaluate_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::VariableDeclaration {
                name,
                type_annotation,
                value,
            } => {
                let val = self.evaluate(value);
                // should add type check here but for now assume the user input correctly
                self.insert_value(name.clone(), val);
            }
            _ => {
                Error::init("some error".to_string(), None, None).print_error();
                unreachable!();
            }
        }
    }
}
