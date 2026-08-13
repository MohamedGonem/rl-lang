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

#[cfg(feature = "vm")]
mod backend;
#[cfg(feature = "vm")]
mod command_handler;
#[cfg(feature = "vm")]
mod completion;
#[cfg(feature = "vm")]
mod depth_checker;
#[cfg(feature = "vm")]
mod input_eval;
#[cfg(feature = "vm")]
mod lines_types;
#[cfg(feature = "vm")]
mod logic_loop;
#[cfg(feature = "vm")]
mod output_render;
#[cfg(feature = "vm")]
mod syntax_highlighting;
#[cfg(feature = "vm")]
mod theme;
#[cfg(feature = "vm")]
mod utils;

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
