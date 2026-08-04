use std::net::UdpSocket;

use crate::{
    Vm,
    stdlib::{
        common::{extract_string, verr, vok, vs},
        net::{NetHandle, common::insert_handle},
    },
    values::VmValue,
};

pub fn func(eval: &mut Vm, address: VmValue) -> VmValue {
    let addr = match extract_string(address, "udp_bind") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("udp_bind: {} ", e))),
    };
    match UdpSocket::bind(&addr) {
        Ok(socket) => {
            let id = insert_handle(eval, NetHandle::UdpSocket(socket));
            vok!(id)
        }
        Err(e) => verr!(vs!(format!("udp_bind(\"{}\"): {}", addr, e))),
    }
}
