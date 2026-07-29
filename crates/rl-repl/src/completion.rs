//! Tab-completion candidate generation for the REPL input bar.
//!
//! Completion is prefix-based over a merged pool of candidates:
//! - `:`-commands (only when the word being completed starts with `:`)
//! - rl-lang reserved keywords, spelled exactly as the lexer expects them
//! - `std::<module>` and `std::<module>::<function>` paths, sourced from
//!   [`rl_docs::entries`] so this stays in sync with the real stdlib
//! - names currently bound in the evaluator: top-level functions, `record`
//!   types, and `tag` types
//!
//! [`logic_loop`](crate::logic_loop) owns the actual `Tab`-cycling behavior
//! (which candidate is showing, wrapping on repeated presses); this module
//! only answers "what could this word become".

use rl_docs::entries;
use rl_interpreter::evaluator::Evaluator;

/// All `:`-prefixed REPL meta-commands, kept in sync with [`crate::command_handler`].
pub const COMMANDS: &[&str] = &[
    ":help", ":stdlib", ":save", ":load", ":attach", ":detach", ":clear", ":reset", ":exit",
];

/// Reserved words of the language.
const KEYWORDS: &[&str] = &[
    "fn", "for", "while", "return", "continue", "break", "get", "from", "in", "or", "and", "null",
    "int", "CONST", "float", "bool", "string", "byte", "char", "true", "false", "dec", "if",
    "else", "arr", "as", "error", "result", "ok", "err", "match", "record", "impl", "tag", "map",
    "set", "loop", "uint", "big", "small", "sbyte",
];

/// Tracks an in-progress `Tab`-cycle so repeated presses walk through
/// `candidates` instead of recomputing them from scratch each time.
pub struct CompletionState {
    pub candidates: Vec<String>,
    pub index: usize,
    /// Char index into the input buffer where the completed word starts.
    pub word_start: usize,
    /// Char length of whichever candidate is currently inserted, so the
    /// next cycle knows exactly what span to replace.
    pub word_len: usize,
}

impl CompletionState {
    /// Advances to the next candidate, wrapping around at the end.
    pub fn next(&mut self) -> &str {
        self.index = (self.index + 1) % self.candidates.len();
        &self.candidates[self.index]
    }
}

/// Returns `(start_char_idx, word)` - the run of completable characters
/// immediately before `cursor_pos` (char-indexed, not byte-indexed).
///
/// A "word" is a run of alphanumerics, `_`, and `:` - the colon is included
/// so a partially-typed `std::io::` stays one unit instead of splitting on
/// each `::`.
pub fn word_at_cursor(input: &str, cursor_pos: usize) -> (usize, String) {
    let chars: Vec<char> = input.chars().collect();
    let end = cursor_pos.min(chars.len());
    let mut start = end;
    while start > 0 {
        let c = chars[start - 1];
        if c.is_alphanumeric() || c == '_' || c == ':' {
            start -= 1;
        } else {
            break;
        }
    }
    (start, chars[start..end].iter().collect())
}

/// Returns every candidate that starts with `word`, deduplicated and sorted.
///
/// `word == ""` matches everything in scope, which is intentional - pressing
/// `Tab` on an empty word starts a full cycle through every known name.
pub fn candidates(word: &str, evaluator: &Evaluator) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();

    if word.starts_with(':') {
        out.extend(
            COMMANDS
                .iter()
                .filter(|c| c.starts_with(word))
                .map(|c| c.to_string()),
        );
        out.sort();
        out.dedup();
        return out;
    }

    out.extend(
        KEYWORDS
            .iter()
            .filter(|k| k.starts_with(word))
            .map(|k| k.to_string()),
    );

    out.extend(
        evaluator
            .fn_names
            .keys()
            .filter(|n| n.starts_with(word))
            .cloned(),
    );
    out.extend(
        evaluator
            .records
            .keys()
            .filter(|n| n.starts_with(word))
            .cloned(),
    );
    out.extend(
        evaluator
            .tags
            .keys()
            .filter(|n| n.starts_with(word))
            .cloned(),
    );

    for module in entries::stdlib_entries() {
        let mod_path = format!("std::{}", module.name);
        if mod_path.starts_with(word) {
            out.push(mod_path.clone());
        }
        // Only expand into individual function paths once the module
        // segment is settled (either the user has typed past it, or the
        // module path itself is a prefix of what they typed) - otherwise
        // completing bare "std" would explode into every function in
        // every module at once.
        let module_settled =
            word.starts_with(&format!("{mod_path}::")) || mod_path.starts_with(word);
        if module_settled {
            for function in module.functions {
                // FnEntry has no standalone `name` field - the callable
                // name is the text before the signature's first `(`.
                let fn_name = function
                    .signature
                    .split('(')
                    .next()
                    .unwrap_or(function.signature)
                    .trim();
                let fn_path = format!("{mod_path}::{fn_name}");
                if fn_path.starts_with(word) {
                    out.push(fn_path);
                }
            }
        }
    }

    out.sort();
    out.dedup();
    out
}
