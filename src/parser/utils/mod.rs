use crate::{
    lexer::tokentypes::{self, TokenType},
    parser::parser::Parser,
};

impl Parser {
    pub fn is_at_end(&self) -> bool {
        matches!(self.peek(), TokenType::Eof)
    }

    pub fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
    }

    pub fn peek(&self) -> TokenType {
        self.tokens[self.current].token.clone()
    }

    pub fn previous(&self) -> TokenType {
        self.tokens[self.current - 1].token.clone()
    }

    pub fn check(&self, token_type: &TokenType) -> bool {
        let current = self.peek();
        match (token_type, &current) {
            (TokenType::NumberLiteral(_), TokenType::NumberLiteral(_)) => true,
            (TokenType::StringLiteral(_), TokenType::StringLiteral(_)) => true,
            (TokenType::FloatLiteral(_), TokenType::FloatLiteral(_)) => true,
            (TokenType::BoolLiteral(_), TokenType::BoolLiteral(_)) => true,
            _ => token_type == &current,
        }
    }

    pub fn match_type(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }
}
