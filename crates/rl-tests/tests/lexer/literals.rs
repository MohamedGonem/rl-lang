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

// --- \x hex escape tests ---

/// `\x41` in a string resolves to 'A'
#[test]
fn string_hex_escape_one_digit() {
    let tokens = common::lex(r#""\x41""#);
    assert_eq!(
        tokens[0].token,
        TokenType::StringLiteral("A".to_string())
    );
}

/// `\xff` in a string resolves to '\u{FF}'
#[test]
fn string_hex_escape_two_digits() {
    let tokens = common::lex(r#""\xff""#);
    assert_eq!(
        tokens[0].token,
        TokenType::StringLiteral("\u{FF}".to_string())
    );
}

/// `\x4` in a string resolves to '\u{04}' (single hex digit)
#[test]
fn string_hex_escape_single_digit() {
    let tokens = common::lex(r#""\x4""#);
    assert_eq!(
        tokens[0].token,
        TokenType::StringLiteral("\u{04}".to_string())
    );
}

/// `'\xff'` as a character literal resolves to '\u{FF}'
#[test]
fn character_hex_escape() {
    let tokens = common::lex(r"'\xff'");
    assert_eq!(tokens[0].token, TokenType::CharacterLiteral('\u{FF}'));
}

/// `\x` with no hex digits in a string produces an error
#[test]
fn string_hex_escape_no_digits() {
    let result = rl_lexer::tokenizer::Tokenizer::lex(
        rl_utils::source::SourceFile::new("test", r#""\x""#.to_string()),
    );
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert!(err.message().contains("expected hex digits"));
}

/// `\xG` (non-hex) in a string produces an error
#[test]
fn string_hex_escape_non_hex() {
    let result = rl_lexer::tokenizer::Tokenizer::lex(
        rl_utils::source::SourceFile::new("test", r#""\xG""#.to_string()),
    );
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert!(err.message().contains("expected hex digits"));
}

// --- \u unicode escape tests ---

/// `\u0041` (fixed 4-digit) in a string resolves to 'A'
#[test]
fn string_unicode_escape_fixed() {
    let tokens = common::lex(r#""\u0041""#);
    assert_eq!(
        tokens[0].token,
        TokenType::StringLiteral("A".to_string())
    );
}

/// `\u{1F600}` (braced) in a string resolves to the grinning face emoji
#[test]
fn string_unicode_escape_braced() {
    let tokens = common::lex(r#""\u{1F600}""#);
    assert_eq!(
        tokens[0].token,
        TokenType::StringLiteral("\u{1F600}".to_string())
    );
}

/// `\u{41}` (braced, short) in a string resolves to 'A'
#[test]
fn string_unicode_escape_braced_short() {
    let tokens = common::lex(r#""\u{41}""#);
    assert_eq!(
        tokens[0].token,
        TokenType::StringLiteral("A".to_string())
    );
}

/// `\u0041` as a character literal resolves to 'A'
#[test]
fn character_unicode_escape_fixed() {
    let tokens = common::lex(r"'\u0041'");
    assert_eq!(tokens[0].token, TokenType::CharacterLiteral('A'));
}

/// `\u{1F600}` as a character literal resolves to the grinning face emoji
#[test]
fn character_unicode_escape_braced() {
    let tokens = common::lex(r"'\u{1F600}'");
    assert_eq!(tokens[0].token, TokenType::CharacterLiteral('\u{1F600}'));
}

/// `\u{}` (empty) in a string produces an error
#[test]
fn string_unicode_escape_empty() {
    let result = rl_lexer::tokenizer::Tokenizer::lex(
        rl_utils::source::SourceFile::new("test", r#""\u{}""#.to_string()),
    );
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert!(err.message().contains("expected hex digits"));
}

/// `\u` with fewer than 4 hex digits (non-braced) in a string produces an error
#[test]
fn string_unicode_escape_too_few_digits() {
    let result = rl_lexer::tokenizer::Tokenizer::lex(
        rl_utils::source::SourceFile::new("test", r#""\u004""#.to_string()),
    );
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert!(err.message().contains("expected 4 hex digits"));
}

/// `\u{110000}` (out of range) in a string produces an error
#[test]
fn string_unicode_escape_out_of_range() {
    let result = rl_lexer::tokenizer::Tokenizer::lex(
        rl_utils::source::SourceFile::new("test", r#""\u{110000}""#.to_string()),
    );
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert!(err.message().contains("invalid unicode codepoint"));
}

/// `\u{ZZZZ}` (invalid char) in a string produces an error
#[test]
fn string_unicode_escape_invalid_char() {
    let result = rl_lexer::tokenizer::Tokenizer::lex(
        rl_utils::source::SourceFile::new("test", r#""\u{ZZZZ}""#.to_string()),
    );
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert!(err.message().contains("invalid character in unicode escape"));
}

/// `\u{` (unterminated) in a string produces an error
#[test]
fn string_unicode_escape_unterminated() {
    let result = rl_lexer::tokenizer::Tokenizer::lex(
        rl_utils::source::SourceFile::new("test", r#""\u{41""#.to_string()),
    );
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert!(
        err.message().contains("unterminated unicode escape")
            || err.message().contains("invalid character in unicode escape")
    );
}
