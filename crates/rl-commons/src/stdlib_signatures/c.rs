//! Typed signatures for `std::c`.

use super::{handle, overloads, params, result};
use crate::{ModuleNames, StdFn};
use rl_ast::statements::TypeAnnotation as T;

pub fn module() -> ModuleNames {
    ModuleNames::new("c")
        .with_typed_function(compile())
        .with_typed_function(call())
        .with_typed_function(close())
}

fn compile() -> StdFn {
    StdFn::typed("compile", vec![(params(vec![T::String]), result(T::Int))])
}

fn call() -> StdFn {
    StdFn::untyped("call")
}

fn close() -> StdFn {
    StdFn::typed("close", overloads(vec![handle()], result(T::Null)))
}
