//! The single, runtime-agnostic implementation of the rl-lang standard library.
//!
//! Each function is written once, generic over `rl_std_core::Runtime`, and
//! annotated with `#[native_fn]` so it is available to both the VM and the
//! interpreter as a thin function pointer, and to the checker as a signature.
//!
//! ## Feature split
//! - `signatures` (default): only the pure `signature()`/`KEYWORDS` data is
//!   compiled - no runtime, no OS-facing dependencies. This is what the type
//!   checker and language server use.
//! - `impls`: the actual function bodies + `handles::<R>()` builders, pulling
//!   in the OS-facing crates. The VM and interpreter enable this.
//!
//! `#[native_fn]` gates the body/wrapper/handle behind `impls` per function, so
//! a module's `signature()` data stays available without compiling the runtime.
//! The pure-`std` modules below expose their signatures in either build; the
//! OS-facing modules (`audio`, `c`, `gui`, `http`, `process`, `terminal`) also
//! gate their hand-written handle types/helpers, so they are compiled only
//! under `impls` for now - the checker still sources those six modules' + `rl`'s
//! signatures from `rl-commons` until their per-file gating is finished.

// In a signatures-only build the per-function bodies (and the imports/helpers
// they use) are gated out by `#[native_fn]`, leaving some imports unused. That
// is expected; keep the (light) checker/LSP build warning-free without hiding
// real unused warnings in the `impls` build.
#![cfg_attr(
    not(feature = "impls"),
    allow(unused_imports, dead_code, unused_macros)
)]

pub mod array;
pub mod bitwise;
pub mod collections;
pub mod debug;
pub mod fs;
pub mod io;
pub mod math;
pub mod net;
pub mod path;
pub mod random;
pub mod result;
pub mod string;
pub mod time;
pub mod types;

// OS-facing modules: gated until their hand-written handle types/helpers are
// split from the signature data (see module docs above).
#[cfg(feature = "impls")]
pub mod audio;
#[cfg(feature = "impls")]
pub mod c;
#[cfg(feature = "impls")]
pub mod gui;
#[cfg(feature = "impls")]
pub mod http;
#[cfg(feature = "impls")]
pub mod process;
#[cfg(feature = "impls")]
pub mod terminal;

/// The checker signature tree for the pure-`std` standard-library modules
/// (everything except the six OS-facing modules and `rl`). Built from the same
/// `#[native_fn]` annotations that generate the runtime handles, so signatures
/// can never drift from implementations. Available without the `impls` feature.
pub fn signatures() -> rl_std_core::ModuleNames {
    rl_std_core::ModuleNames::new("std")
        .with_module(array::signatures())
        .with_module(bitwise::signatures())
        .with_module(collections::signatures())
        .with_module(debug::signatures())
        .with_module(fs::signatures())
        .with_module(io::signatures())
        .with_module(math::signatures())
        .with_module(net::signatures())
        .with_module(path::signatures())
        .with_module(random::signatures())
        .with_module(result::signatures())
        .with_module(string::signatures())
        .with_module(time::signatures())
        .with_module(types::signatures())
}
