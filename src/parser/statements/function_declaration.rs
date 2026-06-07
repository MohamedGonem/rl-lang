use crate::{
    ast::statements::{Statement, StatementKind},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::{errors::Error, span::Span},
};

impl Parser {
    // fn name(param1, param2) {
    //     body
    // }
    //
    // fn name(param1, param2) -> int {   ← return type ignored for now
    //     body
    // }

    pub fn parse_function(&mut self, start: Span) -> Result<Statement, Error> {
        // function name
        let name = match self.peek() {
            TokenType::Identifier(n) => {
                self.advance();
                n
            }
            _ => return Err(self.err("expected function name", self.peek_span())),
        };

        // opening paren
        self.match_type(&[TokenType::LeftParen]);

        // parameters
        let mut params = Vec::new();
        while !self.match_type(&[TokenType::RightParen]) {
            match self.peek() {
                TokenType::Identifier(p) => {
                    self.advance();
                    params.push(p);
                }
                _ => return Err(self.err("expected parameter name", self.peek_span())),
            }
            if !self.match_type(&[TokenType::Comma]) {
                break;
            }
        }
        self.match_type(&[TokenType::RightParen]);

        // optional return type annotation — skip it for now
        if self.match_type(&[TokenType::Arrow]) {
            // consume the type token (int, float, str, etc.)
            self.advance();
        }

        // body

        let body = self.parse_block()?;

        let span = start.join(self.previous_span());
        Ok(Statement::new(
            StatementKind::FunctionDeclaration { name, params, body },
            span,
        ))
    }
}
