#[cfg(feature = "debug")]
use log::info;

use rl_ast::{Ast, statements::Statement};
use rl_lexer::tokentypes::Token;
use rl_parser::parser_logic::Parser;
use rl_utils::source::SourceFile;

/// Parses `tokens` into an AST statement list, or prints the error and exits.
pub fn parse(source: SourceFile, tokens: Vec<Token>) -> (Ast, Vec<Statement>) {
    #[cfg(feature = "debug")]
    info!("parsing the tokens into ast tree...");
    match Parser::parse(tokens, source.clone()) {
        Ok(s) => s,
        Err(e) => {
            e.report_to_stderr();
            std::process::exit(1);
        }
    }
}
