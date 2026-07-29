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

## Modules

| Module | Contents |
|---|---|
| `logic_loop` | Main event loop driving the REPL |
| `command_handler` | Handles REPL meta-commands |
| `completion` | Tab-completion candidate generation (`:`-commands, keywords, stdlib paths, bound names) |
| `depth_checker` | Detects unterminated input to trigger multiline continuation |
| `input_eval` | Feeds submitted input through the lex/parse/resolve/evaluate pipeline |
| `lines_types` | Types backing the scrollable output/history buffers |
| `output_render` | Renders evaluation results and errors to the output area |
| `syntax_highlighting` | Live syntax highlighting of the input bar |
| `theme` | Centralized color palette used by every widget |
| `utils` | Shared REPL helpers |

## Dependencies

Builds on `rl-ast`, `rl-docs`, `rl-interpreter`, `rl-parser`, `rl-lexer`, `rl-utils`, `crossterm`, and `ratatui`.

## Usage

```toml
[dependencies]
rl-repl = { workspace = true }
```

```rust
use rl_repl::start_repl;

start_repl();
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option - see [LICENSE.md](../../LICENSE.md) for the full text and why both are offered.
