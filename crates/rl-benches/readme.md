# rl-benches

> Benchmark suite for the rl-lang pipeline

Part of the [rl-lang](https://github.com/rl-lang/rl-lang) workspace. Not published (`publish = false`) - measures the performance of the lexer, parser, and VM using [Criterion](https://github.com/bheisler/criterion.rs).

## Overview

Each file under `benches/` compiles as its own binary, so shared source snippets and pipeline-stage helpers live once in `src/lib.rs` and are imported with `use rl_benches::*;` instead of being duplicated per benchmark file.

All program fixtures are **self-validated before timing**: each one is run through the full pipeline (lex -> parse -> resolve -> compile -> execute) and asserted to produce its expected value. A broken fixture fails the bench loudly instead of being silently benchmarked as empty work. The realistic workloads end in a bare expression with a known value and are strictly asserted.

## Benchmarks

| Target | Measures |
|---|---|
| `lexer` | Tokenizing throughput across declaration, control-flow, function, and import snippets |
| `parser` | Lexing + parsing throughput on the same snippet set |
| `pipeline` | Discrete pipeline stages in isolation: `resolve`, `vm_compile`, `vm_run` |
| `vm` | VM bytecode `compile` and pure `run` (pre-compiled chunk, fresh VM per iteration) |

### Realistic workloads (`WORKLOAD_PROGRAMS`)

Each has a known final value that is asserted before benchmarking:

- `recursion` - naive `fib(26)` deep recursion
- `string_build` - 300 concatenations into a growing string, then count
- `array_pipeline` - 1000-element array through `arr_map`/`arr_filter`/`arr_reduce`
- `map_set` - growing a set and a map in loops (300 entries each)
- `closure_mutation` - a closure capturing an outer variable, called 1000 times
- `record` - records + `impl` methods in a 500-iteration loop

## Modules

| Module | Contents |
|---|---|
| `lib` | Shared source-code fixtures (`SRC_*` constants, `BASE_PROGRAMS`, `WORKLOAD_PROGRAMS`) and pipeline-stage helpers (`lex_only`, `lex_and_parse`, `parse_and_resolve`, `compile_resolved`, `run_chunk`, `resolve_only`, `verify_*`) |

## Dependencies

Builds on `rl-ast`, `rl-lexer`, `rl-parser`, `rl-resolver`, `rl-vm`, `rl-utils`, and `criterion`.

## Usage

```bash
# all benchmarks
cargo bench -p rl-benches

# a single target
cargo bench -p rl-benches --bench vm
```

HTML reports are written under `target/criterion/` (via the `html_reports` Criterion feature).

## CI

The `Benchmarks` workflow (`.github/workflows/bench.yaml`) runs the full suite on every push to `main` and can be triggered manually from the Actions tab with a target selector. It uploads `target/criterion/` (HTML + JSON estimates) as an artifact.

The workspace `[profile.bench]` mirrors `[profile.release]` (LTO, `codegen-units = 1`) so benchmark numbers reflect shipped binaries.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option - see [LICENSE.md](../../LICENSE.md) for the full text and why both are offered.
