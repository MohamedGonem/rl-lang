use crate::{
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
    vm_logic::Vm,
};
use std::net::ToSocketAddrs;
use std::rc::Rc;

pub fn func(_: &mut Vm, host_port: String) -> VmValue {
    match host_port.to_socket_addrs() {
        Ok(addrs) => {
            let items: Vec<VmValue> = addrs.map(|a| vs!(a.ip().to_string())).collect();
            vok!(VmValue::Arr(Rc::new(items)))
        }
        Err(e) => verr!(vs!(format!("resolve(\"{}\"): {}", host_port, e))),
    }
}
