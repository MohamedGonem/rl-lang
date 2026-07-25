use crate::native::Module;
use libloading::Library;

mod call;
pub enum CHandle {
    Library(Library),
}
