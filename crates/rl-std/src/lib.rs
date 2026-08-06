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
//! `#[native_fn]` gates the body/wrapper/handle behind `impls` per function, and
//! the OS-facing modules (`audio`, `c`, `gui`, `http`, `process`, `terminal`)
//! gate their hand-written handle types/helpers/imports too, so *every* module's
//! `signature()` data is available in a signatures-only build - the checker/LSP
//! read the whole tree via [`signatures`] without compiling eframe/rodio/libffi.

// In a signatures-only build the per-function bodies (and the imports/helpers
// they use) are gated out, leaving some imports unused. That is expected; keep
// the (light) checker/LSP build warning-free without hiding real unused warnings
// in the `impls` build.
#![cfg_attr(
    not(feature = "impls"),
    allow(unused_imports, dead_code, unused_macros)
)]

pub mod array;
pub mod audio;
pub mod bitwise;
pub mod c;
pub mod collections;
pub mod debug;
pub mod fs;
pub mod gui;
pub mod http;
pub mod io;
pub mod math;
pub mod net;
pub mod path;
pub mod process;
pub mod random;
pub mod result;
pub mod string;
pub mod terminal;
pub mod time;
pub mod types;

/// The full `std::*` checker signature tree. Built from the same `#[native_fn]`
/// annotations that generate the runtime handles, so signatures can never drift
/// from implementations. Available without the `impls` feature (no OS-facing
/// deps), so the checker and LSP stay light.
pub fn signatures() -> rl_std_core::ModuleNames {
    rl_std_core::ModuleNames::new("std")
        .with_module(array::signatures())
        .with_module(audio::signatures())
        .with_module(bitwise::signatures())
        .with_module(c::signatures())
        .with_module(collections::signatures())
        .with_module(debug::signatures())
        .with_module(fs::signatures())
        .with_module(gui::signatures())
        .with_module(http::signatures())
        .with_module(io::signatures())
        .with_module(math::signatures())
        .with_module(net::signatures())
        .with_module(path::signatures())
        .with_module(process::signatures())
        .with_module(random::signatures())
        .with_module(result::signatures())
        .with_module(string::signatures())
        .with_module(terminal::signatures())
        .with_module(time::signatures())
        .with_module(types::signatures())
}
