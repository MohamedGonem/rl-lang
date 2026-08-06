//! Evaluates a complete input string and appends results to the output buffer.
use rl_lexer::tokenizer::Tokenizer;
use rl_parser::parser_logic::Parser;
use rl_utils::source::SourceFile;

use crate::{backend::ReplBackend, lines_types::OutputLine};

/// Lexes and parses `input`, then delegates evaluation to `backend`.
///
/// Expression statements have their return value rendered directly with syntax
/// highlighting (unless the value is `null`). Non-expression statements
/// (declarations, loops, etc.) are evaluated for their side effects only.
/// Any `println` / `print` output captured by the backend is flushed into
/// `output` as [`OutputLine::Result`] lines after evaluation.
///
/// Returns `true` if all statements evaluated without error.
pub fn eval_input(
    input: &str,
    backend: &mut dyn ReplBackend,
    output: &mut Vec<OutputLine>,
) -> bool {
    let source = SourceFile::new("<repl>", input.to_string());

    let tokens = match Tokenizer::lex(source.clone()) {
        Ok(t) => t,
        Err(e) => {
            output.push(OutputLine::Error(format!("error: {}", e.message())));
            return false;
        }
    };

    let (file_ast, statements) = match Parser::parse(tokens, source.clone()) {
        Ok(s) => s,
        Err(e) => {
            output.push(OutputLine::Error(format!("error: {}", e.message())));
            return false;
        }
    };

    backend.eval_parsed(source, file_ast, statements, output)
}
