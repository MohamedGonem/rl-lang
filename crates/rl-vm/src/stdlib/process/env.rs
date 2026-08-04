use crate::{
    Vm,
    stdlib::macros::{vnl, vs},
    values::VmValue,
};

pub fn std_env(_: &mut Vm, key: String) -> VmValue {
    match std::env::var(&key) {
        Ok(val) => vs!(val),
        Err(_) => vnl!(),
    }
}
