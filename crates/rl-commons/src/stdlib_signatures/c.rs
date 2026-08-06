//! Typed signatures for `std::c`.

use super::{fixed, handle, overloads, params, result};
use crate::{ModuleNames, StdFn};
use rl_ast::statements::{HandleKind, TypeAnnotation as T};

pub fn module() -> ModuleNames {
    ModuleNames::new("c")
        .with_typed_function(compile())
        .with_typed_function(load())
        .with_typed_function(call())
        .with_typed_function(has_symbol())
        .with_typed_function(close())
        .with_typed_function(clear_cache())
}

fn compile() -> StdFn {
    StdFn::typed(
        "compile",
        vec![(params(vec![T::String]), result(T::Handle(HandleKind::C)))],
    )
}

fn load() -> StdFn {
    StdFn::typed(
        "load",
        vec![(params(vec![T::String]), result(T::Handle(HandleKind::C)))],
    )
}

fn call() -> StdFn {
    StdFn::untyped("call")
}

fn has_symbol() -> StdFn {
    StdFn::typed(
        "has_symbol",
        overloads(
            vec![handle(HandleKind::C), fixed(T::String)],
            result(T::Bool),
        ),
    )
}

fn close() -> StdFn {
    StdFn::typed(
        "close",
        overloads(vec![handle(HandleKind::C)], result(T::Null)),
    )
}

fn clear_cache() -> StdFn {
    StdFn::typed("clear_cache", vec![(params(vec![]), result(T::Null))])
}
