# rl-cc

C transpiler for rl-lang. Turns rl source into C99, with an optional runtime and one-step `cc` build.

## Quick Start

```bash
# Transpile + compile in one shot
cargo run -p rl-cli -- transpile script.rl --compile

# Just transpile (emit .c + runtime files)
cargo run -p rl-cli -- transpile script.rl --runtime

# With optimization
cargo run -p rl-cli -- transpile script.rl --compile --opt 2
```

## What Works

Everything here goes through the full pipeline and produces a working binary.

**Types:** `int`, `uint`, `small int`, `small uint`, `float`, `small float`, `byte`, `sbyte`, `big byte`, `big sbyte`, `bool`, `char`, `string`, `null`

**Declarations:** `dec`, `CONST`, functions, tuples, arrays, maps, sets, records/structs, enums/tags

**Control flow:** `if`/`else if`/`else`, `while`, `for` (C-style), `foreach`, `forrange`, `loop`, `break`, `continue`, `return`

**Expressions:** arithmetic, comparisons, booleans (`and`, `or`, `!`), casts (`as`), field access, index access, index assignment, `match`, `ok()`/`err()`/`error()`, `?` propagation

**Types in C:** tuples (`rl_tuple_N`), arrays (`rl_array`), maps (`rl_map`), sets (`rl_set`), records (`rl_Record_Name`), result (`rl_result`), enums (`int64_t` with `#define RL_TAG_*`)

**Methods:** `impl` blocks on records, dispatched as `impl_RecordName_method(params...)`

**Runtime:** `rl_string`, `rl_result`, `rl_array`, `rl_map`, `rl_set`, typed print functions, per-program record/tuple print functions, `_Generic` macros for `println`/`print`

## What's Not Supported Yet

- Closures / function pointers
- Tagged union dispatch for match (currently uses `if`/`else if` chains on tag values)
- Multi-file / module transpilation

## Architecture

```
src/
  lib.rs                    - transpile(), transpile_to_string()
  codegen/
    mod.rs                  - CCodegen, emit_program(), emit_header(), per-type emission
    statements.rs           - compile_statement(), compile_match()
    expressions.rs          - compile_expr(), compile_func_call(), compile_method_call()
    scope.rs                - declare(), lookup(), push/pop scope, temp_var()
    ops.rs                  - token_to_c_op()
  runtime/
    header.rs               - RUNTIME_H (embedded C header)
    implementation.rs       - RUNTIME_C (embedded C source)
  types.rs                  - type_to_c()
  writer.rs                 - CWriter (indented C source builder)
  name_mangle.rs            - mangle(), escape_c_string(), escape_c_char()
```

Runtime files are also on disk at `crates/rl-cc/runtime/` for clangd support.

## Notes

- Targets C99. The `_Generic` macros need C11 or higher, but most compilers handle it with `-std=c99`.
- `big`/`small` modifiers use keyword syntax: `big byte`, `small int`.
- Boolean operators are keywords: `and`, `or`, `!` (not `&&`/`||`).
- Map keys must be strings in the current runtime (linear scan, `strcmp`-based).
- `record`/`tag`/`impl` declarations are emitted in the header section before `main`.
- Function and impl declarations are hoisted before `main`.
