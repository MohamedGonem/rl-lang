//! `std::c` - compile C source with the system compiler and call into it.

use crate::native::Module;
use libloading::Library;

mod call;
mod close;
mod common;
mod compile;

pub use rl_commons::keywords::c::KEYWORDS;

/// A single native C-interop resource, stored behind an `int` handle.
pub enum CHandle {
    Library(Library),
}

pub fn module() -> Module {
    Module::new("c")
        .with_function("compile", compile::func)
        .with_function("call", call::func)
        .with_function("close", close::func)
}
