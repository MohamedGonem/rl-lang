#[cfg(feature = "debug")]
use log::info;

use rl_lexer::{tokenizer::Tokenizer, tokentypes::Token};
use rl_utils::source::SourceFile;

/// Lexes `source` into a token stream, or prints the error and exits.
pub fn lex(source: SourceFile) -> Vec<Token> {
    #[cfg(feature = "debug")]
    info!("lexing the source file...");
    match Tokenizer::lex(source.clone()) {
        Ok(t) => t,
        Err(e) => {
            e.report_to_stderr();
            std::process::exit(1);
        }
    }
}
