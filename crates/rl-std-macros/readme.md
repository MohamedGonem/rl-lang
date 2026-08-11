# rl-std-macros

> Procedural macros for the rl-lang standard library: `#[native_fn]` lowers a Rust function into a thin-pointer native descriptor plus its checker signature

Part of the [rl-lang](https://github.com/rl-lang/rl-lang) workspace.

## Overview

`#[native_fn(...)]` lowers an annotated stdlib function into:

- a generic thin-`fn`-pointer wrapper (`wrapper::<R>`) that arity-checks, extracts each argument, calls the body, and converts the result
- a `signature()` builder producing the checker's `StdFn`
- a `handle::<R>()` builder producing a `NativeHandle`

The three are placed in a `mod <fn-name>` beside the (unchanged) function, so `mod::handle::<R>()` / `mod::signature()` can be aggregated by the module builders in `rl-std`.

Each Rust parameter is classified structurally:

- a leading `&mut R::Cx` -> the runtime context (forwarded, not an rl arg)
- `R::Value` -> a raw value argument (identity extraction)
- `Vec<R::Value>` -> variadic (the whole argument vector)
- `R::Span` -> the call span
- anything else -> a typed argument extracted via `FromValueR`

Return values are classified as:

- `Result<T, Error>` -> fallible; the error propagates, sig return is `T`
- `Result<T, String>` -> a language `result[T]` value (Ok/Err)
- `R::Value` -> a raw value (requires an explicit `sig`)
- anything else -> converted via `IntoValueR`, sig derived from the type

## Modules

| Module | Contents |
|---|---|
| `attr` | Parsing of the `#[native_fn]` attribute arguments |
| `codegen` | Expansion of the annotated function into wrapper / signature / handle |

## Usage

Used by `rl-std` to annotate its function implementations:

```rust
#[native_fn(module = "math", sig(int -> result[int]), sig(float -> result[float]))]
pub fn abs<R: Runtime>(_cx: &mut R::Cx, a: R::Value) -> R::Value {
    // ...
}
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option - see [LICENSE.md](../../LICENSE.md) for the full text and why both are offered.
