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

## What Works Today

Everything here goes through the full pipeline and produces a working binary.

**Types:** `int`, `uint`, `small int`, `small uint`, `float`, `small float`, `byte`, `sbyte`, `big byte`, `big sbyte`, `bool`, `char`, `string`, `null`

**Language:**
- `dec`/`CONST` declarations
- Functions with params and return types
- `if`/`else if`/`else`, `while`, `for` (with `break`/`continue`)
- `return` with value
- Arithmetic (`+`, `-`, `*`, `/`), comparisons (`==`, `!=`, `<`, `>`, `<=`, `>=`), booleans (`and`, `or`, `!`)
- Casts (`value as type`), escape sequences in strings
- `println`/`print` with type-dispatched C11 `_Generic` macros

**Runtime:** reference-counted `rl_string`, typed print functions for all numeric sizes, bools, chars, strings.

## What Transpiles But Has No Runtime Yet

These pass rl's type checker and emit valid C, but the generated code references types and functions that don't exist in `rl_runtime.c` yet.

- Tuples, arrays, records/structs, enums/tags
- Match expressions (compiles to `if`/`else if` chains on tag values)
- `result` type, `ok()`/`err()`/`error()`, the `?` propagation operator
- Closures (placeholder `rl_closure` type)
- Map/set literals (unhandled)

## Architecture

```
src/
  lib.rs                    - transpile(), transpile_to_string()
  codegen/
    mod.rs                  - CCodegen, emit_program(), emit_header()
    statements.rs           - compile_statement(), write_conditional(), compile_match()
    expressions.rs          - compile_expr(), compile_func_call(), compile_method_call()
    scope.rs                - declare(), lookup(), push/pop scope, temp_var()
    ops.rs                  - token_to_c_op()
  runtime/
    header.rs               - RUNTIME_H (C11 _Generic print macros)
    implementation.rs       - RUNTIME_C (typed print functions)
  types.rs                  - type_to_c()
  writer.rs                 - CWriter (indented C source builder)
  name_mangle.rs            - mangle(), escape_c_string(), escape_c_char()
```

## Notes

- Targets C99. The `_Generic` macros need C11 or higher, but most compilers handle it with `-std=c99`.
- `big`/`small` modifiers use keyword syntax: `big byte`, `small int`.
- Boolean operators are keywords: `and`, `or`, `!` (not `&&`/`||`).
- Match currently emits `if`/`else if` chains. Tagged union dispatch is planned.
- All compound types need struct/union definitions added to the runtime before they'll link.
