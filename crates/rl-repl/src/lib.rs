//! TUI REPL for rl, built on [`ratatui`] and [`crossterm`].
//!
//! # Layout
//!
//! ```text
//! |---------------------------------|
//! |  output area  (scrollable)      |
//! |  ❯ dec int x = 10               |
//! |  ❯ x + 1                        |
//! |  11                             |
//! |---------------------------------|
//! |  ❯ _  (input bar)               |
//! |---------------------------------|
//! ```
//!
//! # Key bindings
//!
//! | Key            | Action                        |
//! |----------------|-------------------------------|
//! | `Enter`        | Submit / continue multiline   |
//! | `Ctrl+C`       | Exit                          |
//! | `Esc`          | Cancel multiline input        |
//! | `↑` / `↓`     | History navigation            |
//! | `Shift+↑/↓`   | Scroll output                 |
//! | `Ctrl+←/→`    | Word jump                     |
//! | `Home` / `End` | Line start / end              |
//! | `Tab`          | Complete / cycle candidates   |

#[cfg(any(feature = "treewalker", feature = "vm"))]
mod backend;
#[cfg(any(feature = "treewalker", feature = "vm"))]
mod command_handler;
#[cfg(any(feature = "treewalker", feature = "vm"))]
mod completion;
#[cfg(any(feature = "treewalker", feature = "vm"))]
mod depth_checker;
#[cfg(any(feature = "treewalker", feature = "vm"))]
mod input_eval;
#[cfg(any(feature = "treewalker", feature = "vm"))]
mod lines_types;
#[cfg(any(feature = "treewalker", feature = "vm"))]
mod logic_loop;
#[cfg(any(feature = "treewalker", feature = "vm"))]
mod output_render;
#[cfg(any(feature = "treewalker", feature = "vm"))]
mod syntax_highlighting;
#[cfg(any(feature = "treewalker", feature = "vm"))]
mod theme;
#[cfg(any(feature = "treewalker", feature = "vm"))]
mod utils;

/// The treewalker-backed REPL: initializes the ratatui terminal, runs the
/// interactive loop against [`backend::TreewalkerBackend`], and restores the
/// terminal on exit. Prints any IO error to stderr.
#[cfg(feature = "treewalker")]
pub fn start_treewalker_repl() {
    let mut terminal = ratatui::init();
    let mut backend = backend::TreewalkerBackend::new();
    let result = logic_loop::run_repl(&mut terminal, &mut backend);
    ratatui::restore();

    if let Err(e) = result {
        eprintln!("repl error: {}", e);
    }
}

/// The bytecode VM-backed REPL: initializes the ratatui terminal, runs the
/// interactive loop against [`backend::VmBackend`] (persistent global state
/// across inputs), and restores the terminal on exit. Prints any IO error to
/// stderr.
#[cfg(feature = "vm")]
pub fn start_vm_repl() {
    let mut terminal = ratatui::init();
    let mut backend = backend::VmBackend::new();
    let result = logic_loop::run_repl(&mut terminal, &mut backend);
    ratatui::restore();

    if let Err(e) = result {
        eprintln!("repl error: {}", e);
    }
}

/// Initializes the ratatui terminal, runs the REPL loop, and restores the
/// terminal on exit. Prints any IO error to stderr.
///
/// Deprecated alias for [`start_treewalker_repl`] - kept so existing callers
/// keep working. Prefer the explicitly-named backend entry points.
#[cfg(feature = "treewalker")]
pub fn start_repl() {
    start_treewalker_repl();
}
