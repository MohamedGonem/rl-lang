use crate::entry::{FnEntry, StdEntry};

mod call;
mod clear_cache;
mod close;
mod compile;
mod has_symbol;
mod load;

pub static C: StdEntry = StdEntry {
    name: "c",
    description: "compile or load a C shared library (via a system compiler or dlopen) and call into it through libffi",
    functions: FUNCTIONS,
    since: Some("v0.4.0"),
    unstable: true,
};

static FUNCTIONS: &[&FnEntry] = &[
    &call::CALL,
    &clear_cache::CLEAR_CACHE,
    &close::CLOSE,
    &compile::COMPILE,
    &has_symbol::HAS_SYMBOL,
    &load::LOAD,
];
