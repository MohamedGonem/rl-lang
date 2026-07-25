//! `std::c` - compile or load a C shared library and call into it.

use crate::native::Module;
use libloading::Library;

mod close;
mod common;
mod load;

/// A single native C-interop resource, stored behind an `int` handle.
pub enum CHandle {
    Library(Library),
}

