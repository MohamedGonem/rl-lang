//! Unit annotation parser.
//!
//! Parses the unit expression that appears after `:` in numeric declarations.
//!
//! ```text
//! dec float distance: m = 10.0
//! dec float speed: m/s = 12.5
//! dec float force: kg*m/s = 20.0
//! ```
//!
//! Unit annotations are represented as [`UnitAnnotation`] syntax trees. Their
//! mathematical normalization and consistency checking are performed later by
//! the type checker, then the annotation is discarded before execution.

use rl_ast::statements::{TypeAnnotation, UnitAnnotation};
use rl_lexer::tokentypes::TokenType;
use rl_utils::errors::Error;

use crate::parser_logic::Parser;

impl Parser {
    /// Returns `true` if `ty` is a numeric type that can carry a unit
    /// annotation (any `int`/`uint`/`float`/`byte` variant, mutable or const).
    pub fn is_numeric_type(ty: &TypeAnnotation) -> bool {
        matches!(
            ty,
            TypeAnnotation::Int
                | TypeAnnotation::UInt
                | TypeAnnotation::SInt
                | TypeAnnotation::SUInt
                | TypeAnnotation::Float
                | TypeAnnotation::SFloat
                | TypeAnnotation::Byte
                | TypeAnnotation::SByte
                | TypeAnnotation::BByte
                | TypeAnnotation::BSByte
                | TypeAnnotation::CInt
                | TypeAnnotation::CUInt
                | TypeAnnotation::CSInt
                | TypeAnnotation::CSUInt
                | TypeAnnotation::CFloat
                | TypeAnnotation::CSFloat
                | TypeAnnotation::CByte
                | TypeAnnotation::CSByte
                | TypeAnnotation::CBByte
                | TypeAnnotation::CBSByte
        )
    }

    /// Parses a complete unit annotation expression.
    ///
    /// The `:` token must already have been consumed by the declaration
    /// parser. Parsing begins at the first unit symbol and stops before a token
    /// that is not part of a unit expression - normally the `=`.
    ///
    /// # Syntax
    ///
    /// ```text
    /// unit        := symbol { ("*" | "/") symbol }
    /// symbol      := identifier
    /// ```
    ///
    /// Multiplication and division share the same precedence and are parsed
    /// from left to right, e.g. `kg*m/s` becomes
    /// `Divide(Multiply(Symbol("kg"), Symbol("m")), Symbol("s"))`.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - the annotation does not begin with a unit symbol;
    /// - `*` or `/` is not followed by another unit symbol.
    pub fn parse_unit_annotation(&mut self) -> Result<UnitAnnotation, Error> {
        while self.match_type(&[TokenType::Newline]) {}

        let mut unit = self.parse_unit_symbol()?;

        loop {
            while self.match_type(&[TokenType::Newline]) {}

            let operator = match self.peek() {
                TokenType::Star | TokenType::Slash => self.peek().clone(),
                _ => break,
            };

            self.advance();

            while self.match_type(&[TokenType::Newline]) {}

            let right = self.parse_unit_symbol()?;

            unit = match operator {
                TokenType::Star => UnitAnnotation::Multiply(Box::new(unit), Box::new(right)),
                TokenType::Slash => UnitAnnotation::Divide(Box::new(unit), Box::new(right)),
                _ => unreachable!("unit annotations only accept `*` and `/`"),
            };
        }

        Ok(unit)
    }

    /// Parses a single unit symbol.
    ///
    /// Unit symbols are regular identifiers such as `m`, `s`, `kg`, or `N`.
    ///
    /// # Errors
    ///
    /// Returns an error if the current token is not an identifier.
    fn parse_unit_symbol(&mut self) -> Result<UnitAnnotation, Error> {
        match self.peek() {
            TokenType::Identifier(name) => {
                self.advance();
                Ok(UnitAnnotation::Symbol(name))
            }
            _ => Err(self.err("expected a unit symbol", self.peek_span())),
        }
    }
}
