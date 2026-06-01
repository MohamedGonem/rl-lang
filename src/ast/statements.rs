use crate::{ast::nodes::Expression, lexer::tokentypes::TokenType};

#[derive(Debug)]
pub enum Statement {
    VariableDeclaration {
        name: String,
        type_annotation: TokenType,
        value: Expression,
    },
}
