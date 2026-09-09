//! Integer and float literal scanner.
//!
//! Consumes a run of digits, checks for a `.` to decide between
//! [`TokenType::NumberLiteral`] and [`TokenType::FloatLiteral`], and handles
//! byte literals (`0b` prefix -> [`TokenType::ByteLiteral`]).
//!
//! Also handles numeric suffix sugar: `10_u8`, `3.14_f32`, `100_i32`, etc.
use crate::{tokenizer::Tokenizer, tokentypes::TokenType};

impl Tokenizer {
    /// Scans an integer or float literal starting at the current position.
    ///
    /// A `.` followed by a digit switches to float parsing.
    /// Integers in the range `0..=255` are emitted as [`TokenType::ByteLiteral`],
    /// larger integers as [`TokenType::NumberLiteral`], and decimals as [`TokenType::FloatLiteral`].
    ///
    /// If a `_suffix` follows the number, the appropriate suffixed token type is emitted:
    ///
    /// | Input      | Emitted token                |
    /// |------------|------------------------------|
    /// | `1`        | `ByteLiteral(1)`             |
    /// | `1000`     | `NumberLiteral(1000)`        |
    /// | `3.14`     | `FloatLiteral(3.14)`         |
    /// | `10_u8`    | `ByteLiteral(10)`            |
    /// | `10_i8`    | `SignedByteLiteral(10)`      |
    /// | `10_u16`   | `BigByteLiteral(10)`         |
    /// | `10_i16`   | `BigSignedByteLiteral(10)`   |
    /// | `10_i32`   | `SmallIntLiteral(10)`        |
    /// | `10_u32`   | `SmallUIntLiteral(10)`       |
    /// | `3.14_f32` | `SmallFloatLiteral(3.14)`    |
    /// | `10_i64`   | `SignedLiteral(10)`          |
    /// | `10_u64`   | `NumberLiteral(10)`          |
    /// | `3.14_f64` | `FloatLiteral(3.14)`         |
    pub fn number_literal(&mut self) {
        let mut is_float = false;
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            is_float = true;
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        // Check for numeric suffix: `_suffix`
        if self.peek() == '_' && self.peek_next().is_ascii_alphanumeric() {
            self.advance(); // consume '_'
            let suffix_start = self.current;
            while self.peek().is_ascii_alphanumeric() {
                self.advance();
            }
            let suffix: String = self.source[suffix_start..self.current].iter().collect();
            let number_str: String = self.source[self.start..suffix_start - 1].iter().collect();

            match suffix.as_str() {
                "u8" => {
                    let v: u8 = number_str.parse().unwrap();
                    self.add_token(TokenType::ByteLiteral(v));
                }
                "i8" => {
                    let v: i8 = number_str.parse().unwrap();
                    self.add_token(TokenType::SignedByteLiteral(v));
                }
                "u16" => {
                    let v: u16 = number_str.parse().unwrap();
                    self.add_token(TokenType::BigByteLiteral(v));
                }
                "i16" => {
                    let v: i16 = number_str.parse().unwrap();
                    self.add_token(TokenType::BigSignedByteLiteral(v));
                }
                "i32" => {
                    let v: i32 = number_str.parse().unwrap();
                    self.add_token(TokenType::SmallIntLiteral(v));
                }
                "u32" => {
                    let v: u32 = number_str.parse().unwrap();
                    self.add_token(TokenType::SmallUIntLiteral(v));
                }
                "f32" => {
                    let v: f32 = number_str.parse().unwrap();
                    self.add_token(TokenType::SmallFloatLiteral(v));
                }
                "i64" => {
                    let v: i64 = number_str.parse().unwrap();
                    self.add_token(TokenType::SignedLiteral(v));
                }
                "u64" => {
                    let v: u64 = number_str.parse().unwrap();
                    self.add_token(TokenType::UIntLiteral(v));
                }
                "f64" => {
                    let v: f64 = number_str.parse().unwrap();
                    self.add_token(TokenType::FloatLiteral(v));
                }
                other => {
                    panic!("unknown numeric suffix: _{}", other);
                }
            }
            return;
        }

        let value: String = self.source[self.start..self.current].iter().collect();

        if is_float {
            let parsed_value: f64 = value.parse().unwrap();
            self.add_token(TokenType::FloatLiteral(parsed_value));
        } else {
            let parsed_value: u64 = value.parse().unwrap();
            self.add_token(TokenType::NumberLiteral(parsed_value));
        }
    }
}
