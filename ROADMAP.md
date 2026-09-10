# Roadmap

Current and planned work for the rl-lang project.

<!--
## How to use this file

### Tree format

The project tree uses plain ASCII:
  - `|` for vertical lines
  - `+--` for branches
  - Indent with 4 spaces for sub-items under a branch

### Adding a new crate branch

  1. Add a `+-- rl-crate-name    (short description)` line under the root.
  2. Sub-items go under it indented with 4 extra spaces and `+--`.
  3. Tag each sub-item with a status: `[DONE]`, `[TODO]`, `[WIP]`, or `[BLOCKED]`.

### Adding sub-features to a crate

  - Use `[DONE]` for completed work.
  - Use `[TODO]` for planned work not yet started.
  - Use `[WIP]` for work in progress.
  - Use `[BLOCKED]` for work blocked on another task (note the dependency).

### Active Work section

  - One `### Crate Name` heading per crate with active tasks.
  - Each task is a numbered bold item with a short description.
  - Remove tasks from Active Work once they move to `[DONE]` in the tree.
  - Keep the list short: only current/priority work goes here.

### Rules

  - Tree must stay in sync with the Active Work section.
  - Don't add vague items. Each branch/task must be concrete and testable.
  - One `[WIP]` item per crate at a time. Finish it before starting another.
  - `[BLOCKED]` items must note what they are blocked on.
  - When a release ships, move `[DONE]` items out and clear Active Work.
-->

## Project Structure

```
rl-lang
|
+-- rl-utils          (error handling, source files, helpers)
+-- rl-lexer          (tokenization)
|   +-- [DONE] Arabic keyword aliases (all 38 keywords)
|   +-- [DONE] pipe operator `|>` token
|   +-- [TODO] #427 - allow more statements to have newlines
+-- rl-ast            (AST node types, arena)
|   +-- [TODO] #190 - refactor AST
+-- rl-parser         (source to AST)
|   +-- [DONE] optional semicolons as statement terminators
|   +-- [DONE] pipe operator `|>` desugaring (a |> f() -> a.f())
|   +-- [TODO] #341 - parser tests
+-- rl-resolver       (name resolution, imports)
|   +-- [TODO] #342 - resolver tests
+-- rl-checker        (type checking)
|   +-- [TODO] #348 - refinement and contracts
+-- rl-vm             (bytecode VM)
|   +-- [TODO] #345 - vm tests
|   +-- [TODO] #349 - property-based testing
+-- rl-cranelift      (JIT compilation via Cranelift)
+-- rl-std-core       (core stdlib modules)
|   +-- [DONE] #425 - new math and consts functions
|   +-- [DONE] #414 - more string std functions
|   +-- [DONE] #437 - refactor term_set_title
|   +-- [DONE] Phase 4 - extended stdlib (72 new functions across 11 modules)
+-- rl-std-macros     (stdlib procedural macros)
+-- rl-std            (standard library)
|   +-- [DONE] std::process - exec_background, wait_pid, term_pid, kill_pid, pipe, pipe_all, set_env, remove_env, env_keys, os_name, arch, num_cpus, parent_pid, process_exists, exec_with_stdin, exec_with_env, exec_with_cwd, exec_with_timeout (+ with_* variants)
|   +-- [DONE] std::path - path_normalize, path_is_absolute, path_is_relative, path_split, path_split_extension, path_with_file_name, path_absolute, path_relative, path_canonicalize, path_starts_with, path_ends_with, path_join_many, path_expand_home, path_components
|   +-- [DONE] std::fs - touch, truncate_file, glob, walk_dir, symlink, readlink, hardlink, temp_file, temp_file_in, file_created, file_accessed, file_permissions, set_permissions, list_dir_names, realpath, lock_file, unlock_file
|   +-- [DONE] std::io - handle-based I/O (open, close, read_handle, write_handle, seek, flush, read_all, readline) with HandleKind::File, plus read_all_stdin, decode_utf8, encode_utf8, isatty
|   +-- [DONE] HandleKind::File - new handle variant in rl-ast with IoStore trait
|   +-- [TODO] #431 - std functions aliasing
|   +-- [TODO] #338 - std functions tests
+-- rl-commons        (shared utilities)
+-- rl-cli            (CLI binary: run, dev, repl, transpile)
|   +-- [DONE] `rl print` command (token/parser/ast tree output)
|   +-- [DONE] `rl run -c` flag (inline code execution)
|   +-- [TODO] #426 - add support for more OS
+-- rl-repl           (interactive REPL)
+-- rl-lsp            (language server)
|   +-- [TODO] #298 - LSP module
+-- rl-docs           (documentation generator)
|   +-- [TODO] #188 - update notes/
+-- rl-tooling        (packaging, install helpers)
|   +-- [DONE] shebang scripts (`rl new --script`)
|   +-- [DONE] tree print module (box-drawing output for tokens/statements)
+-- rl-tests          (integration tests)
|   +-- [TODO] #333 - test units
|   +-- [TODO] #337 - test edge cases coverage
|   +-- [TODO] #344 - interpreter tests
+-- rl-benches        (benchmarks)
+-- rl-cc             (C transpiler)
|   |
|   +-- [DONE] type mapping (int -> int64_t, string -> rl_string, etc.)
|   +-- [DONE] variable/constant/function declarations
|   +-- [DONE] control flow (if/else, while, for, foreach, forrange, loop, break, continue)
|   +-- [DONE] return with value
|   +-- [DONE] string escape sequences
|   +-- [DONE] C11 _Generic runtime for type-dispatched printing
|   +-- [DONE] reference-counted string type
|   +-- [DONE] transpile CLI command (--runtime, --compile, --opt)
|   +-- [DONE] boolean operators (and, or, !)
|   +-- [DONE] comparison operators (==, !=, <, >, <=, >=)
|   +-- [DONE] cast expressions (as)
|   +-- [DONE] byte/sbyte/big byte/big sbyte/small int/small uint/small float types
|   +-- [DONE] tuple literals and tuple destruction
|   +-- [DONE] array literals, index access, and index assignment
|   +-- [DONE] record/struct literals, field access, and field assignment
|   +-- [DONE] enum/tag literals and match
|   +-- [DONE] result type, ok/err/error literals, and ? propagation
|   +-- [DONE] rl_result tagged union, rl_value, rl_closure runtime structs
|   +-- [DONE] per-program record/tuple/enum print functions
|   +-- [DONE] map/set declarations, literals, and len
|   +-- [DONE] impl methods on records
|   +-- [DONE] foreach, forrange, and loop
|   +-- [DONE] closures / function pointers
|   +-- [DONE] null printing (nullable_vars tracking, rl_print_raw/rl_println_raw)
|   +-- [DONE] 10 missing stdlib functions (io::read_bytes, types::error_unwrap, random::*)
|   +-- [DONE] enum display (rl_print_Enum_{name} with string table)
|   +-- [DONE] 9 closure-consuming runtime functions (arr_for_each, arr_all, arr_any, etc.)
|   +-- [DONE] std::c FFI module (compile, load, call, has_symbol, close, clear_cache)
|   +-- [DONE] std::net module (tcp_listen, tcp_accept, tcp_connect, tcp_read, tcp_write, tcp_peer_addr, tcp_local_addr, tcp_set_timeout, tcp_set_nonblocking, tcp_shutdown, tcp_close, udp_bind, udp_connect, udp_send, udp_send_to, udp_recv, udp_recv_from, udp_close, resolve)
|   +-- [DONE] result_unwrap type dispatch (str, f64, bool, i64)
|   +-- [DONE] result_unwrap_or fixed (.data.ok_value removed)
|   +-- [TODO] len() only works on strings — fails on arrays/maps/sets
|   +-- [TODO] Map literals silently drop non-string keys
|   +-- [TODO] arr_filter/arr_map/etc. fallthrough — named function args generate wrong C name
|   +-- [TODO] Lambda body compilation incomplete — missing While, Match, ForEach, Break, Continue
|   +-- [TODO] Lambda capture collection misses While, Match, ForEach bodies
|   +-- [TODO] Nested arrays coerce to int64 — ArrayLiteral hardcodes element type
|   +-- [TODO] tagged union dispatch for match on enums
|   +-- [DONE] std::http module (http_get, http_post, http_request, http_server_start/recv/try_recv/respond/stop)
|   +-- [DONE] multi-file / module transpilation
+-- (language)
|   +-- [DONE] pipe operator |>
|   +-- [TODO] #429 - type aliasing
|   +-- [TODO] #375 - more types
+-- (meta)
    +-- [TODO] #280 - related issues tracker
```

## Open Issues (github.com/rl-lang/rl-lang/issues)

| # | Title | Labels |
|---|-------|--------|
| 437 | refactor `term_set_title` | documentation, enhancement, stdlib, good first issue |
| 431 | feat: std functions aliasing | enhancement, language |
| 429 | feat: add `type` aliasing | enhancement, language |
| 427 | Allow more statements to have newlines | enhancement, good first issue |
| 426 | Add support for more OS | enhancement, help wanted |
| 425 | New `math` and its `consts` functions | ~~enhancement, stdlib, good first issue~~ **DONE** |
| 414 | feat Some more str std functions | ~~-~~ **DONE** |
| 375 | feat more types | enhancement, language |
| 349 | feat `Property-based testing` | language |
| 348 | feat `Refinement and Contracts` | language |
| 345 | `vm` tests | - |
| 344 | `interpreter` tests | - |
| 342 | `resolver` tests | - |
| 341 | `parser` tests | - |
| 338 | `std` functions tests | - |
| 337 | Tests edge cases coverage | - |
| 333 | `Test Units` | tracking |
| 298 | `LSP` module | help wanted, internals, refactor |
| 280 | related issues tracker | tracking |
| 190 | Refactor `AST` | internals, performance |
| 188 | Update `notes/` | documentation |

## Active Work

_No active work tracked for rl-cc. See the [DONE] items in the project tree for what has been completed._
