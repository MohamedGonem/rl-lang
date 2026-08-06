//! The VM's standard library - built-in modules registered under `std::*`.

mod array;
pub(crate) mod audio;
mod bitwise;
pub(crate) mod c;
mod collections;
pub mod common;
mod debug;
mod fs;
pub(crate) mod gui;
pub(crate) mod http;
mod io;
mod macros;
mod math;
pub(crate) mod net;
mod path;
mod process;
pub(crate) mod random;
mod result;
mod rl;
mod string;
mod terminal;
mod time;
mod types;

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
            .with_module(audio::module())
            .with_module(gui::module())
            .with_module(bitwise::module())
            .with_module(debug::module())
            .with_module(fs::module())
            .with_module(http::module())
            .with_module(math::module())
            .with_module(net::module())
            .with_module(path::module())
            .with_module(process::module())
            .with_module(random::module())
            .with_module(result::module())
            .with_module(rl::module())
            .with_module(string::module())
            .with_module(terminal::module())
            .with_module(time::module())
            .with_module(types::module()),
    )
}
