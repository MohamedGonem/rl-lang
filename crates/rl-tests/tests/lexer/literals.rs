//! Tests for lexer token recognition
//! Covers literals, arithmetic operators compound assignment operators and punctuation

use super::super::common;
use rl_lexer::tokentypes::TokenType;

/// Unsigned literal produces `NumberLiteral` with correct value and lexeme
#[test]
fn unsigned_literal() {
    let tokens = common::lex("420");
    assert_eq!(tokens[0].token, TokenType::NumberLiteral(420));
    assert_eq!(tokens[0].lexeme, "420");
}

/// Character literal produces `CharacterLiteral` with correct value and lexeme
#[test]
fn character_literal() {
    let tokens = common::lex("'x'");
    assert_eq!(tokens[0].token, TokenType::CharacterLiteral('x'));
    assert_eq!(tokens[0].lexeme, "'x'");
}

/// Float literal produces `FloatLiteral` with correct value and lexeme
#[test]
fn float_literal() {
    let tokens = common::lex("3.84");
    assert_eq!(tokens[0].token, TokenType::FloatLiteral(3.84));
    assert_eq!(tokens[0].lexeme, "3.84");
}

/// String literal strips quotes and produces `StringLiteral` with inner content
#[test]
fn string_literal() {
    let tokens = common::lex("\"hello\"");
    assert_eq!(
        tokens[0].token,
        TokenType::StringLiteral("hello".to_string())
    );
    assert_eq!(tokens[0].lexeme, "\"hello\"");
}

/// `true` keyword produces `BoolLiteral(true)`
#[test]
fn bool_literal_true() {
    let tokens = common::lex("true");
    assert_eq!(tokens[0].token, TokenType::BoolLiteral(true));
    assert_eq!(tokens[0].lexeme, "true");
}

/// `false` keyword produces `BoolLiteral(false)`
#[test]
fn bool_literal_false() {
    let tokens = common::lex("false");
    assert_eq!(tokens[0].token, TokenType::BoolLiteral(false));
    assert_eq!(tokens[0].lexeme, "false");
}

/// Identifier produces `Identifier` with correct name as lexeme
#[test]
fn identifier_literal() {
    let tokens = common::lex("my_var");
    assert_eq!(tokens[0].token, TokenType::Identifier("my_var".to_string()));
    assert_eq!(tokens[0].lexeme, "my_var");
}

/// `Eof` is always the last token produced
#[test]
fn eof_is_last_token() {
    let tokens = common::lex("42");
    assert_eq!(tokens.last().unwrap().token, TokenType::Eof);
}
