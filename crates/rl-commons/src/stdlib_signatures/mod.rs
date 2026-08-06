//! Typed `(params, return_type)` signatures for `std` functions (rl-lang#250).
//!
//! Each submodule here corresponds to one `std::*` module and builds
//! [`crate::StdFn`] values for the functions in it that have a known,
//! non-generic signature. Modules are typed incrementally - anything not
//! covered yet stays registered via `ModuleNames::with_functions` (untyped,
//! unchecked) in [`crate::stdlib_names`].

use crate::StdFn;
use rl_ast::statements::{HandleKind, TypeAnnotation as T};
use std::rc::Rc;

// The 14 pure-`std` modules' signatures now come from `rl-std` (see
// `crate::stdlib_names`). Only the OS-facing modules and the runtime-specific
// `rl` module keep hand-written signatures here for now.
pub mod audio;
pub mod c;
pub mod gui;
pub mod http;
pub mod process;
pub mod rl;
pub mod terminal;

/// Builds the "input" half of a signature pair: a `Tuple` of the expected
/// argument types, in order. An empty `Vec` means "no arguments".
pub fn params(types: Vec<T>) -> T {
    T::Tuple(Rc::new(types))
}

/// Wraps a type as `Result[T]` - the return type of every stdlib function
/// that can fail and follows the `vok!`/`verr!` runtime convention.
pub fn result(inner: T) -> T {
    T::Result(Box::new(inner))
}

pub const NUMERIC: [T; 3] = [T::Int, T::Float, T::Byte];

/// The generic placeholder `T` - shared by every module (`array`,
/// `collections`, `random`, `res`) whose functions are generic over their
/// element type but still have a fixed argument/return "shape" (e.g.
/// `arr_first(array[T]) -> T`, `set_len(set[T]) -> int`).
pub fn t() -> T {
    T::Generic("T".into())
}

/// `array[T]` - shorthand for "array of the generic element", shared by
/// `array` and `random`.
pub fn arr_t() -> T {
    T::Array(Box::new(t()))
}

/// A single handle-typed argument slot, scoped to `kind`'s module.
pub fn handle(kind: HandleKind) -> Vec<T> {
    vec![T::Handle(kind)]
}

/// A single fixed-type argument slot, for use alongside [`handle`] in
/// [`combos`]/[`overloads`].
pub fn fixed(t: T) -> Vec<T> {
    vec![t]
}

/// Expands one option list per positional argument slot into every
/// combination, e.g. `combos(vec![vec![a, b], vec![c]])` =>
/// `[[a, c], [b, c]]`. Shared by `http`/`net`.
pub fn combos(parts: Vec<Vec<T>>) -> Vec<Vec<T>> {
    parts.into_iter().fold(vec![vec![]], |acc, options| {
        acc.into_iter()
            .flat_map(|prefix| {
                options.iter().map(move |o| {
                    let mut next = prefix.clone();
                    next.push(o.clone());
                    next
                })
            })
            .collect()
    })
}

/// Expands [`combos`] into `(params, return_type)` overload pairs sharing
/// one return type. Shared by `http`/`net`.
pub fn overloads(parts: Vec<Vec<T>>, ret: T) -> Vec<(T, T)> {
    combos(parts)
        .into_iter()
        .map(|combo| (params(combo), ret.clone()))
        .collect()
}

/// `handle_arg -> Result[string]` - a handle-only call that yields a
/// string (e.g. an address, a header value). Shared by `http`/`net`.
pub fn handle_to_string(name: &'static str, kind: HandleKind) -> StdFn {
    StdFn::typed(name, overloads(vec![handle(kind)], result(T::String)))
}

/// `(string) -> string` - shared by `path` and `str` for their many
/// scalar transforms (`path_stem`, `to_lower`, `trim`, ...).
pub fn string_to_string(name: &'static str) -> StdFn {
    StdFn::typed(name, vec![(params(vec![T::String]), T::String)])
}
