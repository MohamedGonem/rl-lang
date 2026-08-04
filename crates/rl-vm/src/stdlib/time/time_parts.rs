use crate::{
    Vm,
    stdlib::{
        common::{verr, vi, vok, vs},
        time::format_time::unix_to_parts,
    },
    values::VmValue,
};
use std::rc::Rc;

pub fn time_parts(_: &mut Vm, timestamp: i64) -> VmValue {
    if timestamp < 0 {
        return verr!(vs!("timestamp is negative".to_string()));
    }

    let (year, month, day, hour, minute, second) = unix_to_parts(timestamp);
    vok!(VmValue::Arr(Rc::new(vec![
        vi!(year as i64),
        vi!(month as i64),
        vi!(day as i64),
        vi!(hour as i64),
        vi!(minute as i64),
        vi!(second as i64),
    ],)))
}
