use crate::{
    ast::nodes::Expression,
    interpreter::evaluator::{self, Evaluator},
    lexer::tokentypes::{Token, TokenType},
};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    pub fn parse(tokens: Vec<Token>) {
        let mut parser = Parser { tokens, current: 0 };
        let mut evaluator = Evaluator::new();

        while !parser.is_at_end() {
            parser.parse_statement(&mut evaluator);
        }
    }
}
