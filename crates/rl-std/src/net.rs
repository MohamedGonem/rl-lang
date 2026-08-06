//! `std::net` - TCP/UDP networking built on `std::net`. Handle module: sockets
//! are stored behind integer handles in the runtime's handle table, accessed
//! via the `NetStore` trait (implemented by each runtime).

use rl_ast::statements::HandleKind;
use rl_std_core::Runtime;
use rl_std_macros::native_fn;
use std::net::{TcpListener, TcpStream, UdpSocket};

/// A single native networking resource, stored behind an integer handle.
/// (Moved here from the per-runtime copies so both share one definition.)
pub enum NetHandle {
    TcpListener(TcpListener),
    TcpStream(TcpStream),
    UdpSocket(UdpSocket),
}

/// Per-runtime access to the `net` handle table. Implemented by `VmRuntime` /
/// `EvalRuntime` in the runtime crates.
pub trait NetStore: Runtime {
    fn net_insert(cx: &mut Self::Cx, h: NetHandle) -> u64;
    fn net_get(cx: &Self::Cx, id: u64) -> Option<&NetHandle>;
    fn net_get_mut(cx: &mut Self::Cx, id: u64) -> Option<&mut NetHandle>;
    fn net_remove(cx: &mut Self::Cx, id: u64) -> Option<NetHandle>;
}

/// Inserts a handle and returns its rl handle value.
fn insert_handle<R: NetStore>(cx: &mut R::Cx, h: NetHandle) -> R::Value {
    let id = R::net_insert(cx, h);
    R::make_handle(HandleKind::Net, id)
}

// ---- shared argument extraction (reproducing the old `extract_*` helpers) --

/// Reproduces the old `extract_string`: rejects non-strings with
/// `"<name>: expected string type, got <ty>"`.
fn extract_string<R: NetStore>(v: &R::Value, name: &str) -> Result<String, String> {
    match R::as_str(v) {
        Some(s) => Ok(s.to_owned()),
        None => Err(format!(
            "{}: expected string type, got {}",
            name,
            R::type_name(v)
        )),
    }
}

/// Reproduces the old `extract_number`: accepts an `int` or `byte`, rejecting
/// everything else with `"<name>: expected int or byte type, got <ty>"`.
fn extract_number<R: NetStore>(v: &R::Value, name: &str) -> Result<u64, String> {
    if let Some(i) = R::as_i64(v) {
        return Ok(i as u64);
    }
    if let Some(b) = R::as_u8(v) {
        return Ok(b as u64);
    }
    Err(format!(
        "{}: expected int or byte type, got {}",
        name,
        R::type_name(v)
    ))
}

/// Reproduces the old `extract_handle`: unwraps a `Net` handle into its id,
/// with the same wrong-kind / not-a-handle messages.
fn extract_handle<R: NetStore>(v: &R::Value, name: &str) -> Result<u64, String> {
    match R::as_handle(v, HandleKind::Net) {
        Some(id) => Ok(id),
        None => match R::as_handle(v, HandleKind::C)
            .map(|_| HandleKind::C)
            .or_else(|| R::as_handle(v, HandleKind::Http).map(|_| HandleKind::Http))
            .or_else(|| R::as_handle(v, HandleKind::Audio).map(|_| HandleKind::Audio))
            .or_else(|| R::as_handle(v, HandleKind::Gui).map(|_| HandleKind::Gui))
        {
            Some(kind) => Err(format!(
                "{}: expected a {:?} handle, got a {:?} handle",
                name,
                HandleKind::Net,
                kind
            )),
            None => Err(format!(
                "{}: expected a handle, got {}",
                name,
                R::type_name(v)
            )),
        },
    }
}

// ---- TCP -------------------------------------------------------------------

#[native_fn(module = "net", bound = "NetStore", sig(string -> result[handle(Net)]))]
pub fn tcp_listen<R: NetStore>(cx: &mut R::Cx, address: R::Value) -> R::Value {
    let addr = match extract_string::<R>(&address, "tcp_listen") {
        Ok(s) => s,
        Err(e) => return R::err(R::from_string(format!("tcp_listen: {} ", e))),
    };

    match TcpListener::bind(&addr) {
        Ok(listener) => R::ok(insert_handle::<R>(cx, NetHandle::TcpListener(listener))),
        Err(e) => R::err(R::from_string(format!("tcp_listen(\"{}\"): {}", addr, e))),
    }
}

#[native_fn(module = "net", bound = "NetStore", sig(handle(Net) -> result[handle(Net)]))]
pub fn tcp_accept<R: NetStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "tcp_accept") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let accept_result = match R::net_get(cx, id) {
        Some(NetHandle::TcpListener(listener)) => listener.accept(),
        Some(_) => {
            return R::err(R::from_string(format!(
                "tcp_accept: handle {} is not a TCP listener",
                id
            )));
        }
        None => return R::err(R::from_string(format!("tcp_accept: unknown handle {}", id))),
    };

    match accept_result {
        Ok((stream, _addr)) => R::ok(insert_handle::<R>(cx, NetHandle::TcpStream(stream))),
        Err(e) => R::err(R::from_string(format!("tcp_accept(): {}", e))),
    }
}

#[native_fn(module = "net", bound = "NetStore", sig(string -> result[handle(Net)]))]
pub fn tcp_connect<R: NetStore>(cx: &mut R::Cx, address: R::Value) -> R::Value {
    let addr = match extract_string::<R>(&address, "tcp_connect") {
        Ok(s) => s,
        Err(e) => return R::err(R::from_string(format!("tcp_connect: {} ", e))),
    };

    match TcpStream::connect(&addr) {
        Ok(stream) => R::ok(insert_handle::<R>(cx, NetHandle::TcpStream(stream))),
        Err(e) => R::err(R::from_string(format!("tcp_connect(\"{}\"): {}", addr, e))),
    }
}

#[native_fn(module = "net", bound = "NetStore", sig(handle(Net), handle(Net) -> result[string]))]
pub fn tcp_read<R: NetStore>(cx: &mut R::Cx, handle: R::Value, max_bytes: R::Value) -> R::Value {
    use std::io::Read;

    let id = match extract_handle::<R>(&handle, "tcp_read") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let max_bytes = match extract_number::<R>(&max_bytes, "tcp_read") {
        Ok(a) => a as usize,
        Err(e) => return R::err(R::from_string(format!("tcp_read: {}", e))),
    };

    let stream = match R::net_get_mut(cx, id) {
        Some(NetHandle::TcpStream(stream)) => stream,
        Some(_) => {
            return R::err(R::from_string(format!(
                "tcp_read: handle {} is not a TCP stream",
                id
            )));
        }
        None => return R::err(R::from_string(format!("tcp_read: unknown handle {}", id))),
    };

    let mut buf = vec![0u8; max_bytes];
    match stream.read(&mut buf) {
        Ok(n) => {
            buf.truncate(n);
            R::ok(R::from_string(String::from_utf8_lossy(&buf).into_owned()))
        }
        Err(e) => R::err(R::from_string(format!("tcp_read: {}", e))),
    }
}

#[native_fn(module = "net", bound = "NetStore", sig(handle(Net), string -> result[int]))]
pub fn tcp_write<R: NetStore>(cx: &mut R::Cx, handle: R::Value, data: R::Value) -> R::Value {
    use std::io::Write;

    let id = match extract_handle::<R>(&handle, "tcp_local_addr") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let data = match extract_string::<R>(&data, "tcp_write") {
        Ok(s) => s,
        Err(e) => return R::err(R::from_string(format!("tcp_write: {} ", e))),
    };

    let stream = match R::net_get_mut(cx, id) {
        Some(NetHandle::TcpStream(stream)) => stream,
        Some(_) => {
            return R::err(R::from_string(format!(
                "tcp_write: handle {} is not a TCP stream",
                id
            )));
        }
        None => return R::err(R::from_string(format!("tcp_write: unknown handle {}", id))),
    };

    match stream.write(data.as_bytes()) {
        Ok(n) => R::ok(R::from_i64(n as i64)),
        Err(e) => R::err(R::from_string(format!("tcp_write: {}", e))),
    }
}

#[native_fn(module = "net", bound = "NetStore", sig(handle(Net) -> result[string]))]
pub fn tcp_peer_addr<R: NetStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "tcp_peer_addr") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let stream = match R::net_get(cx, id) {
        Some(NetHandle::TcpStream(stream)) => stream,
        Some(_) => {
            return R::err(R::from_string(format!(
                "tcp_peer_addr: handle {} is not a TCP stream",
                id
            )));
        }
        None => return R::err(R::from_string(format!("tcp_peer_addr: unknown handle {}", id))),
    };
    match stream.peer_addr() {
        Ok(addr) => R::ok(R::from_string(addr.to_string())),
        Err(e) => R::err(R::from_string(format!("tcp_peer_addr: {}", e))),
    }
}

#[native_fn(module = "net", bound = "NetStore", sig(handle(Net) -> result[string]))]
pub fn tcp_local_addr<R: NetStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "tcp_local_addr") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let stream = match R::net_get(cx, id) {
        Some(NetHandle::TcpStream(stream)) => stream,
        Some(_) => {
            return R::err(R::from_string(format!(
                "tcp_local_addr: handle {} is not a TCP stream",
                id
            )));
        }
        None => return R::err(R::from_string(format!("tcp_local_addr: unknown handle {}", id))),
    };
    match stream.local_addr() {
        Ok(addr) => R::ok(R::from_string(addr.to_string())),
        Err(e) => R::err(R::from_string(format!("tcp_local_addr: {}", e))),
    }
}

#[native_fn(module = "net", bound = "NetStore", sig(handle(Net), handle(Net) -> result[null]))]
pub fn tcp_set_timeout<R: NetStore>(cx: &mut R::Cx, handle: R::Value, millis: R::Value) -> R::Value {
    use std::time::Duration;

    let id = match extract_handle::<R>(&handle, "tcp_set_timeout") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let millis = match extract_number::<R>(&millis, "tcp_set_timeout") {
        Ok(a) => a,
        Err(e) => return R::err(R::from_string(format!("tcp_set_timeout: {}", e))),
    };

    let stream = match R::net_get(cx, id) {
        Some(NetHandle::TcpStream(stream)) => stream,
        Some(_) => {
            return R::err(R::from_string(format!(
                "tcp_set_timeout: handle {} is not a TCP stream",
                id
            )));
        }
        None => return R::err(R::from_string(format!("tcp_set_timeout: unknown handle {}", id))),
    };
    let duration = if millis == 0 {
        None
    } else {
        Some(Duration::from_millis(millis))
    };
    let result = stream
        .set_read_timeout(duration)
        .and_then(|_| stream.set_write_timeout(duration));
    match result {
        Ok(()) => R::ok(R::null()),
        Err(e) => R::err(R::from_string(format!("tcp_set_timeout: {}", e))),
    }
}

#[native_fn(module = "net", bound = "NetStore", sig(handle(Net), bool -> result[null]))]
pub fn tcp_set_nonblocking<R: NetStore>(cx: &mut R::Cx, handle: R::Value, flag: bool) -> R::Value {
    let id = match extract_handle::<R>(&handle, "tcp_set_nonblocking") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let stream = match R::net_get(cx, id) {
        Some(NetHandle::TcpStream(stream)) => stream,
        Some(_) => {
            return R::err(R::from_string(format!(
                "tcp_set_nonblocking: handle {} is not a TCP stream",
                id
            )));
        }
        None => {
            return R::err(R::from_string(format!(
                "tcp_set_nonblocking: unknown handle {}",
                id
            )));
        }
    };

    match stream.set_nonblocking(flag) {
        Ok(()) => R::ok(R::null()),
        Err(e) => R::err(R::from_string(format!("tcp_set_nonblocking: {}", e))),
    }
}

#[native_fn(module = "net", bound = "NetStore", sig(handle(Net), string -> result[null]))]
pub fn tcp_shutdown<R: NetStore>(cx: &mut R::Cx, handle: R::Value, mode: R::Value) -> R::Value {
    use std::net::Shutdown;

    let id = match extract_handle::<R>(&handle, "tcp_local_addr") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let mode = match extract_string::<R>(&mode, "tcp_shutdown") {
        Ok(s) => s,
        Err(e) => return R::err(R::from_string(format!("tcp_shutdown: {} ", e))),
    };

    let mode = match mode.as_str() {
        "read" => Shutdown::Read,
        "write" => Shutdown::Write,
        "both" => Shutdown::Both,
        other => {
            return R::err(R::from_string(format!(
                "tcp_shutdown: expected \"read\", \"write\", or \"both\", got \"{}\"",
                other
            )));
        }
    };
    let stream = match R::net_get(cx, id) {
        Some(NetHandle::TcpStream(stream)) => stream,
        Some(_) => {
            return R::err(R::from_string(format!(
                "tcp_shutdown: handle {} is not a TCP stream",
                id
            )));
        }
        None => return R::err(R::from_string(format!("tcp_shutdown: unknown handle {}", id))),
    };
    match stream.shutdown(mode) {
        Ok(()) => R::ok(R::null()),
        Err(e) => R::err(R::from_string(format!("tcp_shutdown(): {}", e))),
    }
}

#[native_fn(module = "net", bound = "NetStore", sig(handle(Net) -> result[null]))]
pub fn tcp_close<R: NetStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "tcp_close") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::net_get(cx, id) {
        Some(NetHandle::TcpListener(_)) | Some(NetHandle::TcpStream(_)) => {
            R::net_remove(cx, id);
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "tcp_close(): handle {} is not a TCP handle",
            id
        ))),
        None => R::err(R::from_string(format!("tcp_close(): unknown handle {}", id))),
    }
}

// ---- UDP -------------------------------------------------------------------

#[native_fn(module = "net", bound = "NetStore", sig(string -> result[int]))]
pub fn udp_bind<R: NetStore>(cx: &mut R::Cx, address: R::Value) -> R::Value {
    let addr = match extract_string::<R>(&address, "udp_bind") {
        Ok(s) => s,
        Err(e) => return R::err(R::from_string(format!("udp_bind: {} ", e))),
    };
    match UdpSocket::bind(&addr) {
        Ok(socket) => R::ok(insert_handle::<R>(cx, NetHandle::UdpSocket(socket))),
        Err(e) => R::err(R::from_string(format!("udp_bind(\"{}\"): {}", addr, e))),
    }
}

#[native_fn(module = "net", bound = "NetStore", sig(handle(Net), string -> result[null]))]
pub fn udp_connect<R: NetStore>(cx: &mut R::Cx, handle: R::Value, address: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "udp_connect") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let addr = match extract_string::<R>(&address, "udp_connect") {
        Ok(s) => s,
        Err(e) => return R::err(R::from_string(format!("udp_connect: {} ", e))),
    };

    let socket = match R::net_get(cx, id) {
        Some(NetHandle::UdpSocket(socket)) => socket,
        Some(_) => {
            return R::err(R::from_string(format!(
                "udp_connect: handle {} is not a UDP socket",
                id
            )));
        }
        None => return R::err(R::from_string(format!("udp_connect: unknown handle {}", id))),
    };
    match socket.connect(&addr) {
        Ok(()) => R::ok(R::null()),
        Err(e) => R::err(R::from_string(format!("udp_connect(\"{}\"): {}", addr, e))),
    }
}

#[native_fn(module = "net", bound = "NetStore", sig(handle(Net), string -> result[int]))]
pub fn udp_send<R: NetStore>(cx: &mut R::Cx, handle: R::Value, data: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "udp_send") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let data = match extract_string::<R>(&data, "udp_send") {
        Ok(s) => s,
        Err(e) => return R::err(R::from_string(e.to_string())),
    };
    let socket = match R::net_get(cx, id) {
        Some(NetHandle::UdpSocket(socket)) => socket,
        Some(_) => {
            return R::err(R::from_string(format!(
                "udp_send: handle {} is not a UDP socket",
                id
            )));
        }
        None => return R::err(R::from_string(format!("udp_send: unknown handle {}", id))),
    };
    match socket.send(data.as_bytes()) {
        Ok(n) => R::ok(R::from_i64(n as i64)),
        Err(e) => R::err(R::from_string(format!("udp_send: {}", e))),
    }
}

#[native_fn(module = "net", bound = "NetStore", sig(handle(Net), string, string -> result[int]))]
pub fn udp_send_to<R: NetStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    data: R::Value,
    address: R::Value,
) -> R::Value {
    let id = match extract_handle::<R>(&handle, "udp_send_to") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let data = match extract_string::<R>(&data, "udp_send_to") {
        Ok(s) => s,
        Err(e) => return R::err(R::from_string(e.to_string())),
    };
    let addr = match extract_string::<R>(&address, "udp_send_to") {
        Ok(s) => s,
        Err(e) => return R::err(R::from_string(e.to_string())),
    };
    let socket = match R::net_get(cx, id) {
        Some(NetHandle::UdpSocket(socket)) => socket,
        Some(_) => {
            return R::err(R::from_string(format!(
                "udp_send_to: handle {} is not a UDP socket",
                id
            )));
        }
        None => return R::err(R::from_string(format!("udp_send_to: unknown handle {}", id))),
    };
    match socket.send_to(data.as_bytes(), &addr) {
        Ok(n) => R::ok(R::from_i64(n as i64)),
        Err(e) => R::err(R::from_string(format!("udp_send_to(\"{}\"): {}", addr, e))),
    }
}

#[native_fn(module = "net", bound = "NetStore", sig(handle(Net), handle(Net) -> result[string]))]
pub fn udp_recv<R: NetStore>(cx: &mut R::Cx, handle: R::Value, max_bytes: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "udp_recv") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let max_bytes = match extract_number::<R>(&max_bytes, "udp_recv") {
        Ok(a) => a as usize,
        Err(e) => return R::err(R::from_string(format!("udp_recv: {}", e))),
    };
    let socket = match R::net_get(cx, id) {
        Some(NetHandle::UdpSocket(socket)) => socket,
        Some(_) => {
            return R::err(R::from_string(format!(
                "udp_recv: handle {} is not a UDP socket",
                id
            )));
        }
        None => return R::err(R::from_string(format!("udp_recv: unknown handle {}", id))),
    };
    let mut buf = vec![0u8; max_bytes];
    match socket.recv(&mut buf) {
        Ok(n) => {
            buf.truncate(n);
            R::ok(R::from_string(String::from_utf8_lossy(&buf).into_owned()))
        }
        Err(e) => R::err(R::from_string(format!("udp_recv: {}", e))),
    }
}

#[native_fn(module = "net", bound = "NetStore", sig(handle(Net), handle(Net) -> result[tuple[string, string]]))]
pub fn udp_recv_from<R: NetStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    max_bytes: R::Value,
) -> R::Value {
    let id = match extract_handle::<R>(&handle, "udp_recv_from") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let max_bytes = match extract_number::<R>(&max_bytes, "udp_recv_from") {
        Ok(a) => a as usize,
        Err(e) => return R::err(R::from_string(e.to_string())),
    };
    let socket = match R::net_get(cx, id) {
        Some(NetHandle::UdpSocket(socket)) => socket,
        Some(_) => {
            return R::err(R::from_string(format!(
                "udp_recv_from: handle {} is not a UDP socket",
                id
            )));
        }
        None => return R::err(R::from_string(format!("udp_recv_from: unknown handle {}", id))),
    };
    let mut buf = vec![0u8; max_bytes];
    match socket.recv_from(&mut buf) {
        Ok((n, sender)) => {
            buf.truncate(n);
            let data = String::from_utf8_lossy(&buf).into_owned();
            R::ok(R::tuple(vec![
                R::from_string(data),
                R::from_string(sender.to_string()),
            ]))
        }
        Err(e) => R::err(R::from_string(format!("udp_recv_from: {}", e))),
    }
}

#[native_fn(module = "net", bound = "NetStore", sig(handle(Net) -> result[null]))]
pub fn udp_close<R: NetStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "udp_close") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::net_get(cx, id) {
        Some(NetHandle::UdpSocket(_)) => {
            R::net_remove(cx, id);
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "udp_close: handle {} is not a UDP socket",
            id
        ))),
        None => R::err(R::from_string(format!("udp_close: unknown handle {}", id))),
    }
}

// ---- resolution ------------------------------------------------------------

// `host_port` is a typed `String` argument (extracted by the generated wrapper,
// exactly as in the old `func(_, host_port: String)`); the body only produces
// the raw `result[array[string]]` value.
#[native_fn(module = "net", bound = "NetStore", sig(string -> result[array[string]]))]
pub fn resolve<R: NetStore>(_cx: &mut R::Cx, host_port: String) -> R::Value {
    use rl_ast::statements::TypeAnnotation;
    use std::net::ToSocketAddrs;

    match host_port.to_socket_addrs() {
        Ok(addrs) => {
            let items: Vec<R::Value> = addrs
                .map(|a| R::from_string(a.ip().to_string()))
                .collect();
            R::ok(R::array(items, TypeAnnotation::String))
        }
        Err(e) => R::err(R::from_string(format!("resolve(\"{}\"): {}", host_port, e))),
    }
}

rl_std_core::native_module!("net";
    bound: NetStore;
    funcs: [
        tcp_listen, tcp_accept, tcp_connect, tcp_read, tcp_write,
        tcp_peer_addr, tcp_local_addr, tcp_set_timeout, tcp_set_nonblocking,
        tcp_shutdown, tcp_close,
        udp_bind, udp_connect, udp_send, udp_send_to, udp_recv, udp_recv_from,
        udp_close,
        resolve,
    ],
);
