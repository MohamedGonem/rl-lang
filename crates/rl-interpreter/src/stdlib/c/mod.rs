use crate::native::Module;
use libloading::Library;

mod call;
mod close;
mod common;
mod compile;
pub enum CHandle {
    Library(Library),
}

pub fn module() -> Module {
    Module::new("c")
        .with_function("compile", compile::func)
        .with_function("call", call::func)
        .with_function("close", close::func)
}
