use crate::{
    ast::{nodes::Expression, statements::Statement},
    lexer::tokentypes::TokenType,
    parser::parser::Parser,
};

impl Parser {
    pub fn parse_variable_declartion(&mut self) -> Statement {
        // println!("{:?}", self.peek());
        // println!("parsing type");
        let var_type = match self.peek() {
            TokenType::Int
            | TokenType::Float
            | TokenType::Bool
            | TokenType::String
            | TokenType::Char => {
                let t = self.peek();
                self.advance();
                t
            }
            _ => {
                crate::utils::errors::Error::init(
                    "expected type after dec".to_string(),
                    None,
                    Some(crate::utils::errors::ErrorReason::init(
                        crate::utils::errors::Reason::Parse,
                        None,
                    )),
                )
                .print_error();
                unreachable!()
            }
        };

        let name = match self.peek() {
            TokenType::Identifier(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => {
                crate::utils::errors::Error::init(
                    "expected name after type".to_string(),
                    None,
                    Some(crate::utils::errors::ErrorReason::init(
                        crate::utils::errors::Reason::Parse,
                        None,
                    )),
                )
                .print_error();
                unreachable!()
            }
        };

        if !self.match_type(&[TokenType::Assign]) {
            crate::utils::errors::Error::init(
                "expected '=' after name".to_string(),
                None,
                Some(crate::utils::errors::ErrorReason::init(
                    crate::utils::errors::Reason::Parse,
                    None,
                )),
            )
            .print_error();
        }

        let value = self.parse_expression();

        crate::ast::statements::Statement::VariableDeclaration {
            name,
            type_annotation: var_type,
            value,
        }
    }
}
