//! Token-based syntax highlighting for the REPL input bar and output.
//!
//! Lexes the input with the real rl lexer and maps each [`TokenType`] to a
//! ratatui [`Style`]. Gaps between token spans are emitted as unstyled raw spans
//! to preserve whitespace exactly. On lex failure the entire input is returned
//! as a single error-colored span.
//!
//! # Color scheme
//!
//! Colors are pulled from [`crate::theme`] so the input bar's highlighting
//! stays visually consistent with borders, prompts, and output coloring.
//!
//! | Token group         | Color / modifier            |
//! |---------------------|------------------------------|
//! | Control flow        | `SYN_KEYWORD` bold           |
//! | Declarations        | `SYN_DECL` italic            |
//! | Import keywords     | `SYN_IMPORT` dim             |
//! | Type keywords       | `SYN_TYPE` italic            |
//! | Logical operators   | `SYN_LOGIC` bold             |
//! | Number literals     | `SYN_NUMBER`                 |
//! | String literals     | `SYN_STRING`                 |
//! | Char literals       | `SYN_CHAR`                   |
//! | Bool literals       | `SYN_BOOL` italic            |
//! | `null`              | `SYN_NULL` italic            |
//! | Comparison ops      | `SYN_COMPARE`                |
//! | Punctuation/braces  | `SYN_PUNCT`                  |
//! | Identifiers         | `theme::TEXT`                |

use crate::theme;
use ratatui::{
    style::{Modifier, Style},
    text::Span,
};
use rl_lexer::{tokenizer::Tokenizer, tokentypes::TokenType};
use rl_utils::source::SourceFile;

/// Returns the ratatui [`Style`] for a given [`TokenType`].
fn token_color(tt: &TokenType) -> Style {
    match tt {
        // control flow
        TokenType::If
        | TokenType::Else
        | TokenType::While
        | TokenType::For
        | TokenType::Break
        | TokenType::Continue
        | TokenType::Return => Style::default()
            .fg(theme::SYN_KEYWORD)
            .add_modifier(Modifier::BOLD),

        // declarations
        TokenType::Dec | TokenType::Const | TokenType::Fn | TokenType::Array => Style::default()
            .fg(theme::SYN_DECL)
            .add_modifier(Modifier::ITALIC),

        // import keywords
        TokenType::Get | TokenType::From => Style::default()
            .fg(theme::SYN_IMPORT)
            .add_modifier(Modifier::DIM),

        // types
        TokenType::Int
        | TokenType::UInt
        | TokenType::Float
        | TokenType::Bool
        | TokenType::String
        | TokenType::Char
        | TokenType::Byte
        | TokenType::Error => Style::default()
            .fg(theme::SYN_TYPE)
            .add_modifier(Modifier::ITALIC),

        // logic operators
        TokenType::Or | TokenType::And => Style::default()
            .fg(theme::SYN_LOGIC)
            .add_modifier(Modifier::BOLD),

        // number literals
        TokenType::NumberLiteral(_) | TokenType::FloatLiteral(_) => {
            Style::default().fg(theme::SYN_NUMBER)
        }

        // string literals
        TokenType::StringLiteral(_) => Style::default().fg(theme::SYN_STRING),

        // char literals
        TokenType::CharacterLiteral(_) => Style::default().fg(theme::SYN_CHAR),

        // bool literals
        TokenType::BoolLiteral(_) => Style::default()
            .fg(theme::SYN_BOOL)
            .add_modifier(Modifier::ITALIC),

        // null
        TokenType::Null => Style::default()
            .fg(theme::SYN_NULL)
            .add_modifier(Modifier::ITALIC),

        // arrow, dimmed
        TokenType::Arrow | TokenType::Pipe => Style::default().fg(theme::TEXT).add_modifier(Modifier::DIM),

        // operators
        TokenType::Plus
        | TokenType::Minus
        | TokenType::Star
        | TokenType::Slash
        | TokenType::Bang
        | TokenType::PlusEqual
        | TokenType::MinusEqual
        | TokenType::StarEqual
        | TokenType::SlashEqual => Style::default().fg(theme::SYN_OPERATOR),

        // comparison operators
        TokenType::Compare
        | TokenType::BangEqual
        | TokenType::Greater
        | TokenType::GreaterEqual
        | TokenType::Less
        | TokenType::LessEqual => Style::default().fg(theme::SYN_COMPARE),

        // assignment
        TokenType::Assign => Style::default().fg(theme::SYN_OPERATOR),

        // punctuation
        TokenType::Comma
        | TokenType::Semicolon
        | TokenType::Colon
        | TokenType::ColonColon
        | TokenType::Dot
        | TokenType::DotDot
        | TokenType::Hash
        | TokenType::BangHash => Style::default().fg(theme::SYN_PUNCT),

        // braces/parens/brackets
        TokenType::LeftBrace
        | TokenType::RightBrace
        | TokenType::LeftParen
        | TokenType::RightParen
        | TokenType::LeftBracket
        | TokenType::RightBracket => Style::default().fg(theme::SYN_PUNCT),

        // identifiers
        TokenType::Identifier(_) => Style::default().fg(theme::TEXT),

        _ => Style::default().fg(theme::TEXT),
    }
}

/// Lexes `input` and returns a vec of syntax-highlighted [`Span`]s.
///
/// On lex error returns a single error-colored span containing the raw input.
pub fn highlight(input: &str) -> Vec<Span<'static>> {
    let source = SourceFile::new("<hl>", input.to_string());
    let tokens = match Tokenizer::lex(source) {
        Ok(t) => t,
        Err(_) => {
            return vec![Span::styled(
                input.to_string(),
                Style::default().fg(theme::ERROR),
            )];
        }
    };

    let mut spans = Vec::new();
    let mut last = 0usize;
    let chars: Vec<char> = input.chars().collect();

    for tok in &tokens {
        let start = tok.span.start;
        let end = tok.span.end;

        if start > last {
            let gap: String = chars[last..start].iter().collect();
            spans.push(Span::raw(gap));
        }

        let text: String = chars[start..end].iter().collect();
        let style = token_color(&tok.token);
        spans.push(Span::styled(text, style));
        last = end;
    }

    if last < chars.len() {
        let tail: String = chars[last..].iter().collect();
        spans.push(Span::raw(tail));
    }

    spans
}
