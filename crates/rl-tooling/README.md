# rl-tooling

Project scaffolding, packaging, formatting, documentation generation, and CLI helper utilities for the rl programming language.

## Commands

| Command | Description |
|---------|-------------|
| `rl new <name>` | Scaffold a new rl project directory with `rl.toml` and `src/main.rl` |
| `rl new --script <name>` | Create a standalone executable `.rl` script with shebang header |
| `rl new <name> --no-git` | Skip `git init` when scaffolding |
| `rl package <file>` | Bundle a `.rl` file into a self-contained binary |
| `rl workflows` | Generate GitHub Actions workflow YAML |
| `rl docs --generate` | Build project documentation from `///` doc comments |
| `rl format <file>` | Reformat a `.rl` source file in place |
| `rl print <file>` | Print token/parser/AST tree with box-drawing characters |

## Script mode

`rl new --script hello` creates `hello.rl` with:
- A shebang header pointing to the `rl` binary (auto-detected via `current_exe` / PATH scan, falls back to `rlc`)
- A hello world program
- Executable permissions (`0755`)

The shebang is stripped by the lexer, so `rl run hello.rl` and `./hello.rl` both work.

## Packaging

`rl package script.rl` bundles the source into a self-contained binary that embeds the rl runtime. Use `--vm` to compile to `.rlc` bytecode first, skipping lex/parse at startup.

## Files

- `src/new.rs` - `create_project()`, `create_script()`, `find_rl_binary()`
- `src/package.rs` - `package()`, `find_embedded()` for self-contained binaries
- `src/format.rs` - `format_tokens()` for source formatting
- `src/generate_docs.rs` - documentation site generation
- `src/workflows.rs` - GitHub Actions YAML generation
- `src/tree_print.rs` - box-drawing tree printer for tokens and statements
- `src/dev.rs` - `read_rl_toml()` for project config
