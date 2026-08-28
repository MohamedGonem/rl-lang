# Changelog

All notable changes to the rl-lang toolchain are documented here. The format is based on [Keep a Changelog](https://keepachangelog.com/), and the project follows [Semantic Versioning](https://semver.org/) (see [VERSIONING.md](VERSIONING.md)). Full per-commit history is available on the [GitHub Releases](https://github.com/rl-lang/rl-lang/releases) page.

## [Unreleased]

### Added

- **C transpiler (`rl-cc`)** - transpiles rl to C99. Core language works end-to-end: types, arithmetic, booleans, comparisons, control flow, functions, casts, println/print. Compound types (tuples, arrays, records, enums, result) emit valid C but need runtime support to link. CLI: `rl transpile file.rl --compile`.
- **Escape sequences** - string and character literals now support the full set of escape sequences: `\n`, `\t`, `\r`, `\0`, `\\`, `\"`, `\'`, `\a`, `\b`, `\f`, `\v`, `\e`, plus `\xHH` hex byte escapes (1–2 hex digits, e.g. `\x41`, `\xff`) and `\uHHHH` / `\u{HHHH...}` unicode codepoint escapes (e.g. `\u0041`, `\u{1F600}`).

## [2.0.0] - 2026-08-13

The bytecode VM is now the sole execution backend; the tree-walking interpreter has been removed.

### Removed

- **Tree-walking interpreter** - the `rl-interpreter` crate and its `treewalker` feature are gone. `rl run` / `rl dev` execute through the bytecode VM only, and the `--treewalker` CLI flag no longer exists.
- **Interpreter test suite** - the `tests/interpreter` suite and the interpreter-vs-VM benchmark target have been removed.
- **Treewalker build variants** - the `rl_treewalker` / `rlp` release variants are no longer built or offered by the installers.

### Changed

- `rl run` / `rl dev` default to the bytecode VM backend.
- The VM now resolves programs directly via `rl-resolver` (previously it borrowed the interpreter's `Evaluator` as a resolver+stdlib holder).

[2.0.0]: https://github.com/rl-lang/rl-lang/releases/tag/v2.0.0

## [1.0.0] - 2026-08-06

First stable release. The workspace is now a set of 1.0.0 crates, and the bytecode VM is a default, drop-in execution backend alongside the tree-walking interpreter. Interactive installers are available for Linux and Windows.

### Added

- **Bytecode VM stdlib parity** - `rl-vm` now mirrors every standard library module of `rl-interpreter` (`array`, `audio`, `bitwise`, `c`, `collections`, `debug`, `fs`, `gui`, `http`, `io`, `math`, `net`, `path`, `process`, `random`, `result`, `rl`, `string`, `terminal`, `time`, `types`).
- **New `uint` type** - unsigned 8-byte integer (`UInt`), plus `sbyte` and the `big` / `small` modifiers, and `u8` / `i16` array element types.
- **`std::gui` module** - windowed UI toolkit with widgets (`gui_button`, `gui_checkbox`, `gui_dropdown`, `gui_image`, `gui_label`, `gui_number_input`, `gui_progress_bar`, `gui_radio_group`, `gui_separator`, `gui_slider`, `gui_textarea`, `gui_textbox`, `gui_window`), lifecycle/event control functions (`gui_run`, `gui_on_click`, `gui_on_change`, ...), and `z`-level ordering.
- **`std::audio` module** - audio playback (`play_file`, `play_file_async`, `beep`), transport controls (`sound_pause`, `sound_resume`, `sound_seek`, `sound_stop`, `sound_wait`, `sound_set_speed`), volume (`set_master_volume`, `sound_get/set_volume`), and output-device listing/selection.
- **`std::c` module** - call C library functions from rl via `libffi` (`call`, `compile`, `load`, `close`), including a cache for pre-built libraries and `CArg` typed signatures.
- **`impl` keyword** - methods on records, resolved and checked across `rl-lexer`, `rl-parser`, `rl-ast`, `rl-resolver`, `rl-checker`, `rl-vm`, and `rl-interpreter`.
- **`handle` type** - opaque handles (u64) for external resources.
- **`loop` keyword** - loop control flow.
- **Stdlib functions** - `map_keys`, `map_values`, `map_to_array`, `map_merge`, `map_remove`, `map_contains`, `map_clear`, `map_get`, `map_len`, `map_is_empty` in `std::collections`; `std::math::consts`; `with_*` exec helpers in `std::process`.
- **REPL improvements** - tab completion, `:clear` and `:reset` commands, centralized theme with live syntax highlighting, and a `VmBackend` that keeps persistent VM state across submitted inputs.
- **Installers** - interactive `install.sh` (Linux) and `install.ps1` (Windows) scripts that download prebuilt binaries from GitHub Releases, with variant selection and version pinning; nightly builds publish all variants.
- **Documentation** - docs for the new modules and concepts (`std::c`, `std::audio`, `std::gui`, `impl`/records, arrays, casts, nulls, operators, types), plus `examples/` templates for new modules.

### Changed

- All crates bumped to **1.0.0**.
- The bytecode VM is now the default execution backend of `rl-cli` (enabled by the `vm` feature, on by default).
- Dropped automatic numeric promotion between `int` and `float`.
- `handle`s changed from `i64` to `u64`.
- `Generic` type annotation now holds a `String` for the type name.
- `std::term::read_key` returns `result[arr[string]]`.
- `char` prints without surrounding quotes.

### Fixed

- `uint` value casts in the interpreter and VM.
- Parser test spans are now derived from source instead of hardcoded.
- Checker `?` operator now validates the inner type correctly.
- `map` type indexing in `ExpressionKind::Index`.

### Performance

- Optimized the VM dispatch loop (lazy spans, unchecked operands, cached frame base).
- Bench suite now simulates release mode with corrected programs.

[1.0.0]: https://github.com/rl-lang/rl-lang/releases/tag/v1.0.0
