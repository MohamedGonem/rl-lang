//! Central color palette for the REPL.
//!
//! A dark, desaturated palette (Tokyo-Night-adjacent) so every widget -
//! borders, titles, prompts, syntax highlighting, output lines - pulls from
//! one consistent set of colors instead of ad-hoc [`ratatui::style::Color`]
//! picks scattered across files. Changing the look of the REPL should mean
//! editing constants here, not hunting through five files for `Color::Cyan`.

use ratatui::style::Color;

// -- base --
/// Very dark background tone - used as the foreground for text sitting on
/// top of a solid accent-colored block (e.g. the cursor).
pub const BG_DARK: Color = Color::Rgb(0x1a, 0x1b, 0x26);

// -- chrome (borders, titles) --
/// Default (unfocused) border color for both panes.
pub const BORDER: Color = Color::Rgb(0x3b, 0x40, 0x54);
/// Border color for the pane currently receiving input.
pub const BORDER_FOCUS: Color = Color::Rgb(0x7a, 0xa2, 0xf7);
/// Block titles (" rl ", command headers).
pub const TITLE: Color = Color::Rgb(0x7a, 0xa2, 0xf7);

// -- text --
/// Primary readable text (identifiers, plain output).
pub const TEXT: Color = Color::Rgb(0xc0, 0xca, 0xf5);
/// Secondary text - hints, separators, punctuation.
pub const TEXT_DIM: Color = Color::Rgb(0x56, 0x5f, 0x89);
/// Lowest-emphasis text - the empty-input cursor bar, faint punctuation.
pub const TEXT_MUTED: Color = Color::Rgb(0x41, 0x48, 0x68);

// -- semantic accents --
/// Primary prompt / accent color (the `❯` prompt, headers).
pub const ACCENT: Color = Color::Rgb(0x7a, 0xa2, 0xf7);
/// Secondary accent - the `·` continuation prompt, argument placeholders.
pub const ACCENT2: Color = Color::Rgb(0xbb, 0x9a, 0xf7);
pub const SUCCESS: Color = Color::Rgb(0x9e, 0xce, 0x6a);
pub const WARNING: Color = Color::Rgb(0xe0, 0xaf, 0x68);
pub const ERROR: Color = Color::Rgb(0xf7, 0x76, 0x8e);
pub const INFO: Color = Color::Rgb(0x73, 0x7a, 0xa2);

// -- syntax highlighting --
pub const SYN_KEYWORD: Color = Color::Rgb(0xbb, 0x9a, 0xf7); // control flow
pub const SYN_DECL: Color = Color::Rgb(0x7a, 0xa2, 0xf7); // dec/const/fn
pub const SYN_IMPORT: Color = Color::Rgb(0x56, 0x5f, 0x89); // get/from
pub const SYN_TYPE: Color = Color::Rgb(0x2a, 0xc3, 0xde); // type keywords
pub const SYN_LOGIC: Color = Color::Rgb(0xe0, 0xaf, 0x68); // and/or
pub const SYN_NUMBER: Color = Color::Rgb(0xff, 0x9e, 0x64);
pub const SYN_STRING: Color = Color::Rgb(0x9e, 0xce, 0x6a);
pub const SYN_CHAR: Color = Color::Rgb(0xc9, 0xd6, 0x7a);
pub const SYN_BOOL: Color = Color::Rgb(0xff, 0x9e, 0x64);
pub const SYN_NULL: Color = Color::Rgb(0x56, 0x5f, 0x89);
pub const SYN_OPERATOR: Color = Color::Rgb(0xc0, 0xca, 0xf5);
pub const SYN_COMPARE: Color = Color::Rgb(0x2a, 0xc3, 0xde);
pub const SYN_PUNCT: Color = Color::Rgb(0x56, 0x5f, 0x89);
