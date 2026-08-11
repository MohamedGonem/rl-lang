# rl-std

> The single, runtime-agnostic implementation of the rl-lang standard library, shared by the VM and the interpreter

Part of the [rl-lang](https://github.com/rl-lang/rl-lang) workspace.

## Overview

Each standard-library function is written once, generic over `rl_std_core::Runtime`, and annotated with `#[native_fn]` so it is available to both the VM and the interpreter as a thin function pointer, and to the checker as a signature.

The crate is split into two features:

- `signatures` (default) - only the pure `signature()`/`KEYWORDS` data is compiled, with no runtime and no OS-facing dependencies. This is what the type checker and language server use.
- `impls` - the actual function bodies plus `handles::<R>()` builders, pulling in the OS-facing crates. The VM and interpreter enable this.

`#[native_fn]` gates the body/wrapper/handle behind `impls` per function, so every module's `signature()` data is available in a signatures-only build without compiling eframe/rodio/libffi.

## Modules

| Module | Contents |
|---|---|
| `array` | `std::array` - array builders, slicing, and reductions |
| `audio` | `std::audio` - audio playback and volume (via `rodio`/`cpal`/`symphonia`) |
| `bitwise` | `std::bitwise` - bit-level operations on integers |
| `c` | `std::c` - call C library functions via `libffi` |
| `collections` | `std::collections` - map and set operations |
| `debug` | `std::debug` - runtime debugging helpers |
| `fs` | `std::fs` - filesystem access |
| `gui` | `std::gui` - windowed UI toolkit (via `eframe`) |
| `http` | `std::http` - HTTP requests (via `tiny_http`/`ureq`) |
| `io` | `std::io` - printing and stdin |
| `math` | `std::math` - math functions and `std::math::consts` |
| `net` | `std::net` - networking helpers |
| `path` | `std::path` - path manipulation |
| `process` | `std::process` - process execution |
| `random` | `std::random` - PRNG-backed generation |
| `result` | `std::result` - `result[T]` helpers |
| `string` | `std::str` - string manipulation |
| `terminal` | `std::terminal` - terminal / key input |
| `time` | `std::time` - time and timing helpers |
| `types` | `std::types` - type introspection |

## Dependencies

Depends on `rl-std-core`, `rl-std-macros`, `rl-ast`, and `rl-utils`. The OS-facing modules bring in `crossterm`, `tiny_http`, `ureq`, `libloading`, `libffi`, `rodio`, `cpal`, `symphonia`, `eframe`, and `shell-words` (only when the `impls` feature is enabled).

## Usage

```toml
[dependencies]
rl-std = { workspace = true, features = ["impls"] }
```

The checker and language server use the default `signatures` feature; the VM and interpreter depend on `rl-std` with the `impls` feature enabled.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option - see [LICENSE.md](../../LICENSE.md) for the full text and why both are offered.
