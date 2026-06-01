mod variable_declaration;
use crate::{lexer::tokentypes::TokenType, parser::parser::Parser};

impl Parser {
    pub fn parse_statement(&mut self) {
        match self.peek() {
            TokenType::Newline => self.advance(),
            TokenType::Dec => {
                self.advance();
                let stmt = self.parse_variable_declartion();
                println!("{:?}", stmt);
            }
            _ => {
                let expr = self.parse_expression();
                println!("{:?}", expr);
                let mut x = crate::interpreter::evaluator::Evaluator {};
                println!("{:?}", x.evaluate(&expr));
            }
        }
    }
}
