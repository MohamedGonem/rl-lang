//! The rl standard library - all built-in modules registered under `std::*`.
//!
//! Only the legacy `rl` and `len` functions remain here; every other module now
//! lives in the shared `rl-std` crate. `common` still backs `rl`, but some of
//! its helpers are unused until `rl` migrates too.

#[allow(dead_code, unused_macros, unused_imports)]
mod common;
pub mod len;
pub mod rl;
