//! TUI REPL for rl, built on [`ratatui`] and [`crossterm`].
//!
//! # Layout
//!
//! ```text
//! |---------------------------------|
//! |  output area  (scrollable)      |
//! |  >> dec int x = 10              |
//! |  >> x + 1                       |
//! |  11                             |
//! |---------------------------------|
//! |  >> _  (input bar)              |
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

mod command_handler;
mod completion;
mod depth_checker;
mod input_eval;
mod lines_types;
mod logic_loop;
mod output_render;
mod syntax_highlighting;
mod theme;
mod utils;

/// Initializes the ratatui terminal, runs the REPL loop, and restores the
/// terminal on exit. Prints any IO error to stderr.
pub fn start_repl() {
    let mut terminal = ratatui::init();
    let result = logic_loop::run_repl(&mut terminal);
    ratatui::restore();

    if let Err(e) = result {
        eprintln!("repl error: {}", e);
    }
}
