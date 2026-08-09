# rl-repl

> Interactive REPL for the rl-lang programming language

Part of the [rl-lang](https://github.com/rl-lang/rl-lang) workspace. A TUI REPL built on `ratatui` and `crossterm`, wired up behind the `repl` feature of `rl-cli`.

## Layout

```text
|-- ✦ rl ------------------------ ● live --|
|  ❯ dec int x = 10                        |
|  ❯ x + 1                                 |
|  11                                       |
|--------------------------------------------|
|-- input ---- Tab complete · Shift+↑↓ ... --|
|  ❯ _                                      |
|--------------------------------------------|
```

A dark, Tokyo-Night-adjacent palette is centralized in `theme.rs` - see that module for the full color list.

## Key bindings

| Key | Action |
|---|---|
| `Enter` | Submit / continue multiline |
| `Ctrl+C` | Exit |
| `Esc` | Cancel multiline input |
| `↑` / `↓` | History navigation |
| `Shift+↑/↓` | Scroll output |
| `Ctrl+←/→` | Word jump |
| `Home` / `End` | Line start / end |
| `Tab` | Complete word at cursor / cycle candidates |

## Backends

The REPL UI never touches an execution engine directly - it drives a
[`ReplBackend`](src/backend.rs), so `rl` can run the same interactive loop on
either engine:

| Backend | Execution engine | Feature |
|---|---|---|
| `TreewalkerBackend` | tree-walking interpreter (`rl-interpreter`) | `treewalker` |
| `VmBackend` | bytecode VM (`rl-vm`) with persistent global state across inputs | `vm` |

`VmBackend` keeps a single `Vm` + `Resolver` alive for the whole session, so
declarations, functions, and `get x from std::io` imports survive between
submitted inputs. Each input is resolved against the persistent resolver,
compiled with a `Compiler` seeded at the current global slot count, and run
via `run_and_return` so a trailing expression's value is rendered.

## Modules

| Module | Contents |
|---|---|
| `backend` | [`ReplBackend`] trait plus the `TreewalkerBackend` / `VmBackend` implementations |
| `logic_loop` | Main event loop driving the REPL |
| `command_handler` | Handles REPL meta-commands |
| `completion` | Tab-completion candidate generation (`:`-commands, keywords, stdlib paths, bound names) |
| `depth_checker` | Detects unterminated input to trigger multiline continuation |
| `input_eval` | Lexes/parses submitted input, then delegates evaluation to the backend |
| `lines_types` | Types backing the scrollable output/history buffers |
| `output_render` | Renders evaluation results and errors to the output area |
| `syntax_highlighting` | Live syntax highlighting of the input bar |
| `theme` | Centralized color palette used by every widget |
| `utils` | Shared REPL helpers |

## Features

- `treewalker` (default) - tree-walking interpreter backend (`rl-interpreter`)
- `vm` - bytecode VM backend (`rl-vm` + `rl-resolver`)

Both backends produce identical REPL behavior; enabling `vm` makes the VM the
preferred engine (as wired up by `rl-cli`). The crate requires at least one
backend feature.

## Dependencies

Builds on `rl-ast`, `rl-docs`, `rl-parser`, `rl-lexer`, `rl-utils`, `crossterm`,
and `ratatui`, plus `rl-interpreter` (`treewalker`), `rl-vm` (`vm`), and
`rl-resolver` (`vm`).

## Usage

```toml
[dependencies]
rl-repl = { workspace = true }
```

```rust
use rl_repl::start_treewalker_repl;

start_treewalker_repl();
```

`start_vm_repl()` starts the bytecode VM-backed REPL instead. `start_repl()` is
kept as a deprecated alias for `start_treewalker_repl()`.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option - see [LICENSE.md](../../LICENSE.md) for the full text and why both are offered.
