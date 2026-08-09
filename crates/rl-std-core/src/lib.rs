//! Runtime-agnostic core for the rl-lang standard library.
//!
//! This crate holds everything the shared stdlib (`rl-std`) and both runtimes
//! (`rl-vm`, `rl-interpreter`) need in common, without referencing either
//! runtime's value type - so it can sit below both in the dependency graph:
//!
//! - [`Runtime`] / [`HandleStore`] - the abstraction each runtime implements,
//! - [`NativeHandle`] / [`Arity`] - the thin-`fn`-pointer native descriptor,
//! - [`ValueType`] / [`FromValueR`] / [`IntoValueR`] - value <-> Rust type
//!   conversions,
//! - [`StdFn`] / [`ModuleNames`] - the checker signature types (moved here from
//!   `rl-commons`),
//! - [`Xoshiro256`] - the shared PRNG.

pub mod convert;
pub mod handle;
#[macro_use]
pub mod module;
pub mod rng;
pub mod runtime;
pub mod signatures;

pub use convert::{FromValueR, IntoValueR, ValueType};
pub use handle::{Arity, NativeHandle, NativeThunk};
pub use rng::Xoshiro256;
pub use runtime::{HandleStore, Runtime};
pub use signatures::{ModuleNames, StdFn};
