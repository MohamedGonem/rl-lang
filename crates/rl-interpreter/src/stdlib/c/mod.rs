use crate::native::Module;
use libloading::Library;

pub enum CHandle {
    Library(Library),
}
