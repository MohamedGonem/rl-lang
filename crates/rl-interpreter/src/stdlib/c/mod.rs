use crate::native::Module;
use libloading::Library;

mod call;
pub enum CHandle {
    Library(Library),
}

pub fn module() -> Module {
    Module::new("c")
        .with_function("call", call::func)
}
