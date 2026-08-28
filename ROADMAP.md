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
|   +-- [TODO] #427 - allow more statements to have newlines
+-- rl-ast            (AST node types, arena)
|   +-- [TODO] #190 - refactor AST
+-- rl-parser         (source to AST)
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
|   +-- [TODO] #425 - new math and consts functions
|   +-- [TODO] #414 - more string std functions
|   +-- [TODO] #437 - refactor term_set_title
+-- rl-std-macros     (stdlib procedural macros)
+-- rl-std            (standard library)
|   +-- [TODO] #431 - std functions aliasing
|   +-- [TODO] #338 - std functions tests
+-- rl-commons        (shared utilities)
+-- rl-cli            (CLI binary: run, dev, repl, transpile)
|   +-- [TODO] #426 - add support for more OS
+-- rl-repl           (interactive REPL)
+-- rl-lsp            (language server)
|   +-- [TODO] #298 - LSP module
+-- rl-docs           (documentation generator)
|   +-- [TODO] #188 - update notes/
+-- rl-tooling        (packaging, install helpers)
+-- rl-tests          (integration tests)
|   +-- [TODO] #333 - test units
|   +-- [TODO] #337 - test edge cases coverage
|   +-- [TODO] #344 - interpreter tests
+-- rl-benches        (benchmarks)
+-- rl-cc             (C transpiler)
|   |
|   +-- [DONE] type mapping (int -> int64_t, string -> rl_string, etc.)
|   +-- [DONE] variable/constant/function declarations
|   +-- [DONE] control flow (if/else, while, for, break, continue)
|   +-- [DONE] return with value
|   +-- [DONE] string escape sequences
|   +-- [DONE] C11 _Generic runtime for type-dispatched printing
|   +-- [DONE] reference-counted string type
|   +-- [DONE] transpile CLI command (--runtime, --compile, --opt)
|   +-- [DONE] boolean operators (and, or, !)
|   +-- [DONE] comparison operators (==, !=, <, >, <=, >=)
|   +-- [DONE] cast expressions (as)
|   +-- [DONE] byte/sbyte/big byte/big sbyte/small int/small uint/small float types
|   +-- [WIP] tuple literals (transpiles, no rl_tuple in runtime)
|   +-- [WIP] array literals and index access (transpiles, no rl_array in runtime)
|   +-- [WIP] record/struct literals and field access (transpiles, no rl_Record in runtime)
|   +-- [WIP] enum/tag literals and match (transpiles, no RL_TAG_ defines in runtime)
|   +-- [WIP] result type, ok/err/error literals (transpiles, no rl_result in runtime)
|   +-- [WIP] error propagation ? operator (transpiles, no rl_result in runtime)
|   +-- [TODO] runtime: rl_tuple, rl_array, rl_Record, rl_result struct definitions
|   +-- [TODO] runtime: rl_ok, rl_err, rl_error function implementations
|   +-- [TODO] runtime: rl_arr_from_vals, RL_TAG_ constant definitions
|   +-- [TODO] tagged union dispatch for match on enums
|   +-- [TODO] closure/function pointer support (rl_closure placeholder)
|   +-- [TODO] map/set operations in runtime
|   +-- [TODO] multi-file / module transpilation
+-- (language)
|   +-- [TODO] #462 - pipe operator |>
|   +-- [TODO] #429 - type aliasing
|   +-- [TODO] #375 - more types
+-- (meta)
    +-- [TODO] #280 - related issues tracker
```

## Open Issues (github.com/rl-lang/rl-lang/issues)

| # | Title | Labels |
|---|-------|--------|
| 462 | feat pipe `\|>` | enhancement, language |
| 437 | refactor `term_set_title` | documentation, enhancement, stdlib, good first issue |
| 431 | feat: std functions aliasing | enhancement, language |
| 429 | feat: add `type` aliasing | enhancement, language |
| 427 | Allow more statements to have newlines | enhancement, good first issue |
| 426 | Add support for more OS | enhancement, help wanted |
| 425 | New `math` and its `consts` functions | enhancement, stdlib, good first issue |
| 414 | feat Some more str std functions | - |
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

### rl-cc C Transpiler

Core language works end-to-end: types, arithmetic, booleans, comparisons, control flow, functions, casts, println/print. Compound types (tuples, arrays, records, enums, result) transpile fine but the C runtime doesn't define the structs/functions they reference yet.

**Next:** Add rl_tuple, rl_array, rl_Record, rl_result, rl_ok, rl_err, RL_TAG_* to the runtime, then tackle tagged union match and closures.
