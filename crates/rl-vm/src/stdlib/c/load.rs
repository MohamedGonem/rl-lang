use crate::{
    Vm,
    stdlib::{
        c::{CHandle, common::insert_handle},
        common::extract_string,
        macros::{verr, vok, vs},
    },
    values::VmValue,
};

pub fn std_load(vm: &mut Vm, path: VmValue) -> VmValue {
    let path = match extract_string(path, "load") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("load: {}", e))),
    };

    // SAFETY: loading a library the caller pointed us at by path. Its static
    // initializers (if any) run here, same as `compile`'s load step and same
    // as `dlopen` in C itself - this is inherently trusting whatever code is
    // at that path, not something a Rust-side check can make "safe".
    let lib = match unsafe { libloading::Library::new(&path) } {
        Ok(l) => l,
        Err(e) => {
            return verr!(vs!(format!("load: failed to load \"{}\": {}", path, e)));
        }
    };

    let handle = insert_handle(vm, CHandle::Library(lib));
    vok!(handle)
}
