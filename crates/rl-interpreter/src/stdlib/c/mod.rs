//! `std::c` - compile or load a C shared library and call into it.

use crate::native::Module;
use libloading::Library;

mod call;
mod clear_cache;
mod close;
mod common;
mod compile;
mod has_symbol;
mod load;

pub use rl_commons::keywords::c::KEYWORDS;

/// A single native C-interop resource, stored behind an `int` handle.
pub enum CHandle {
    Library(Library),
}

pub fn module() -> Module {
    Module::new("c")
        .with_function("compile", compile::func)
        .with_function("load", load::func)
        .with_function("call", call::func)
        .with_function("has_symbol", has_symbol::func)
        .with_function("close", close::func)
        .with_function("clear_cache", clear_cache::func)
}
