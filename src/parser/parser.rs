use crate::{ast::statements::Statement, lexer::tokentypes::Token};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Vec<Statement> {
        let mut parser = Parser { tokens, current: 0 };
        let mut statements = Vec::new();

        while !parser.is_at_end() {
            statements.push(parser.parse_statement_to_ast());
        }

        statements
    }
}
