use crate::entry::{FnEntry, StdEntry};

mod call;
mod close;
mod compile;
mod load;

pub static C: StdEntry = StdEntry {
    name: "c",
    description: "compile or load a C shared library (via a system compiler or dlopen) and call into it through libffi",
    functions: FUNCTIONS,
    since: Some("v0.4.0"),
    unstable: true,
};

static FUNCTIONS: &[&FnEntry] = &[&compile::COMPILE, &load::LOAD, &call::CALL, &close::CLOSE];
