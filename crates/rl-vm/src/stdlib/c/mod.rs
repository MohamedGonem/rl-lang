//! `std::c` - compile or load a C shared library and call into it.

use crate::native::Module;
use libloading::Library;

mod call;
mod close;
mod common;
mod compile;
mod has_symbol;
mod load;

/// A single native C-interop resource, stored behind an `int` handle.
pub enum CHandle {
    Library(Library),
}

pub fn module() -> Module {
    Module::new("c")
        .with_function("compile", compile::std_compile)
        .with_function("load", load::std_load)
        .with_function("call", call::std_call)
        .with_function("has_symbol", has_symbol::std_has_symbol)
        .with_function("close", close::std_close)
}
