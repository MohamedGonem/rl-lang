# Changelog

All notable changes to the rl-lang toolchain are documented here. The format is based on [Keep a Changelog](https://keepachangelog.com/), and the project follows [Semantic Versioning](https://semver.org/) (see [VERSIONING.md](VERSIONING.md)). Full per-commit history is available on the [GitHub Releases](https://github.com/rl-lang/rl-lang/releases) page.

## [Unreleased]

### Added

- **Shebang support** - `.rl` files starting with `#!` are valid; the lexer strips the shebang line before tokenizing. `rl new --script <name>` creates a standalone executable `.rl` script with a shebang header pointing to the `rl` binary (detected via `current_exe` / PATH scan, falls back to `rlc`), a hello world body, and executable permissions (`0755`). Scripts can be run directly: `chmod +x hello.rl && ./hello.rl`.
- **C transpiler (`rl-cc`)** - transpiles rl to C99. Full pipeline works end-to-end: lex, parse, resolve, type-check, C codegen, optional `cc` compilation. Supported features: types, arithmetic, booleans, comparisons, control flow (if/else, while, for, foreach, forrange, loop, break, continue), functions, casts, println/print, tuples, tuple destruction, arrays, maps, sets, records/structs, enums/tags, match, result type (ok/err/error), error propagation (`?`), impl methods, escape sequences, closures, lambda expressions. Runtime includes `rl_string`, `rl_result` (tagged union with type tag enum), `rl_value` (tagged union for map/set storage), `rl_array`, `rl_map`, `rl_set`, `rl_closure`, and per-program record/tuple/enum print functions. CLI: `rl transpile file.rl --compile`.
- **Escape sequences** - string and character literals now support the full set of escape sequences: `\n`, `\t`, `\r`, `\0`, `\\`, `\"`, `\'`, `\a`, `\b`, `\f`, `\v`, `\e`, plus `\xHH` hex byte escapes (1–2 hex digits, e.g. `\x41`, `\xff`) and `\uHHHH` / `\u{HHHH...}` unicode codepoint escapes (e.g. `\u0041`, `\u{1F600}`).
- **`rl print` command** - `rl print <file> --tokens` prints the token stream, `--parser` prints the parsed statement tree, `--ast` prints the type-checked/resolved tree. Uses box-drawing characters for nested output.
- **`rl run -c` flag** - execute inline rl code directly: `rl run -c 'println("hello")'`.
- **Tree print module** (`rl-tooling`) - box-drawing character tree printer for tokens, parser statements, and resolved statements. Used by `rl print`.
- **Closure support (rl-cc)** - `rl_closure` struct with `rl_closure_new`, `rl_closure_call`, `rl_ok_closure`, `RL_TAG_CLOSURE`, `rl_print_closure`, and 9 closure-consuming runtime functions (`arr_for_each`, `arr_all`, `arr_any`, `arr_find_index`, `arr_sort_by`, `arr_flat_map`, `result_map`, `result_map_err`, `bench`).
- **Null printing fix (rl-cc)** - nullable variables tracked via `nullable_vars` set; `dec int x = null` now stores `rl_result x = rl_ok_null();`. Read/write/print correctly unwrap and re-wrap. New `rl_print_raw`/`rl_println_raw` runtime functions print inner values without `ok()`/`err()` wrapper.
- **10 missing stdlib functions (rl-cc)** - `io::read_bytes`, `types::error_unwrap`, `types::to_byte`, `types::to_char`, `random::rand_dices`, `random::rand_bytes`, `random::rand_choice`, `random::rand_choices`, `random::rand_sample`, `random::rand_shuffle`.
- **Enum display (rl-cc)** - generates `rl_print_Enum_{name}()` with static string table per enum type.
- **Float precision (rl-cc)** - strtod round-trip approach for shortest representation.
- **Record/tuple print for nested types (rl-cc)** - `emit_field_print()` helper for nested records and tuples in print statements.

### Changed

- **`rl_result` rewritten (rl-cc)** - tagged union with `enum rl_type_tag`, type-safe constructors (`rl_ok_null`, `rl_ok_i64`, `rl_ok_f64`, etc.), and `_Generic` macro dispatch.
- **Tuple dedup fixed (rl-cc)** - dedup by full field-type layout, not arity.
- **Map/set storage (rl-cc)** - uses `rl_map *map` and `rl_set *set` pointers (not by-value) via `rl_value` tagged union.

### Fixed

- **P1: map::get aborts on missing key** - returns `err("key not found in map")` instead of aborting.
- **P2: res::unwrap no error checking** - new `rl_result_unwrap_i64/f64/bool/str` functions abort on err.
- **P3: math::abs truncates floats** - dispatches `fabs()` for `RL_TAG_F64`.
- **P4: math::pow(int,int) returns float** - new `rl_math_pow` with integer exponentiation path.
- **P5: arr::sort only handles int64** - added `rl_arr_cmp_f64` and `rl_arr_cmp_str` comparators.
- **P6: Time formatting local TZ** - `localtime()` → `gmtime()` in all 4 formatting functions.
- **P7: File I/O error handling** - `rl_io_read_file/write_file/append_file` now return `rl_result` with `err()` on failure.
- **P8: arr::zip flat interleaving** - inline codegen with `memcpy` pairs.
- **P9: is_* stubs** - 8 new runtime functions with real tag checks.
- **P10: Dead duplicate match arms** - removed `sign`/`degrees`/`radians` from first match arm.
- **map_remove_s returns modified map** - matching VM reassignment semantics.
- **process_args** - stores argc/argv via `rl_store_args()` at main start.
- **`_GNU_SOURCE`** added at top of generated `.c` files for `M_PI`/`M_E`.
- **`-lm` required** for linking when `pow()` is used.

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
