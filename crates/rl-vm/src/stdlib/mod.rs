//! The VM's standard library - built-in modules registered under `std::*`.
//!
//! `io`, `collections`, `array`, `c`, and `audio` exist so far. Everything
//! else in `rl-interpreter`'s stdlib (math, string, fs, net, http,
//! process, ...) hasn't been ported yet - most of it needs `VmValue` to
//! grow more variants or `FromValue`/`IntoValue` impls first.

mod array;
pub(crate) mod audio;
pub(crate) mod c;
mod collections;
pub mod common;
mod io;
pub mod macros;

use crate::native::Module;

/// Builds the compiler-facing native module tree: an unnamed root holding
/// a `std` submodule, mirroring `rl-interpreter`'s `root_module` shape so
/// `std::io::println` resolves the same way in both.
pub fn root() -> Module {
    Module::new("root").with_module(
        Module::new("std")
            .with_module(io::module())
            .with_module(collections::module())
            .with_module(array::module())
            .with_module(c::module())
            .with_module(audio::module()),
    )
}
