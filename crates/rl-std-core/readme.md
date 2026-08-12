# rl-std-core

> Runtime-agnostic core for the rl-lang standard library: the `Runtime` abstraction, native-function descriptors, value conversions, and stdlib signatures

Part of the [rl-lang](https://github.com/rl-lang/rl-lang) workspace.

## Overview

Holds what the shared stdlib (`rl-std`) and both runtimes (`rl-vm`, `rl-interpreter`) need in common, without referencing either runtime's value type, so it can sit below both in the dependency graph:

- The `Runtime` trait each runtime implements, plus the shared `HandleStore` for opaque resource handles
- The thin-`fn`-pointer native descriptor (`NativeHandle` / `Arity`)
- Value <-> Rust type conversions (`ValueType` / `FromValueR` / `IntoValueR`)
- The checker signature types (`StdFn` / `ModuleNames`)
- `Xoshiro256`, the shared PRNG

## Modules

| Module | Contents |
|---|---|
| `runtime` | `Runtime` trait, `HandleStore`, and `R::Cx` runtime context |
| `handle` | `NativeHandle`, `Arity`, `NativeThunk` - thin-pointer native descriptors |
| `convert` | `ValueType`, `FromValueR`, `IntoValueR` value <-> Rust type conversions |
| `module` | Module builders that aggregate per-function handles and signatures |
| `rng` | `Xoshiro256` - the shared PRNG |
| `signatures` | `StdFn`, `ModuleNames` - checker signature types |

## Dependencies

Depends only on `rl-ast` and `rl-utils`.

## Usage

```toml
[dependencies]
rl-std-core = { workspace = true }
```

```rust
use rl_std_core::Runtime;

fn process<R: Runtime>(_cx: &mut R::Cx, value: R::Value) -> R::Value {
    value
}
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option - see [LICENSE.md](../../LICENSE.md) for the full text and why both are offered.
