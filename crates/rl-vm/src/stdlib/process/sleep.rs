use crate::{Vm, stdlib::macros::vnl, values::VmValue};
use std::time::Duration;

pub fn std_sleep(_: &mut Vm, ms: i64) -> VmValue {
    std::thread::sleep(Duration::from_millis(ms.max(0) as u64));
    vnl!()
}
