use crate::entry::{FnEntry, StdEntry};

mod resolve;
mod tcp_accept;
mod tcp_close;
mod tcp_connect;
mod tcp_listen;
mod tcp_local_addr;
mod tcp_peer_addr;
mod tcp_read;
mod tcp_set_nonblocking;
mod tcp_set_timeout;
mod tcp_shutdown;
mod tcp_write;
mod udp_bind;
mod udp_close;
mod udp_connect;
mod udp_recv;
mod udp_recv_from;
mod udp_send;
mod udp_send_to;

pub static NET: StdEntry = StdEntry {
    name: "net",
    description: "TCP and UDP networking built on std::net; blocking, single-threaded, handle-based",
    functions: FUNCTIONS,
    since: Some("v2.1.0"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &resolve::RESOLVE,
    &tcp_accept::TCP_ACCEPT,
    &tcp_close::TCP_CLOSE,
    &tcp_connect::TCP_CONNECT,
    &tcp_listen::TCP_LISTEN,
    &tcp_local_addr::TCP_LOCAL_ADDR,
    &tcp_peer_addr::TCP_PEER_ADDR,
    &tcp_read::TCP_READ,
    &tcp_set_nonblocking::TCP_SET_NONBLOCKING,
    &tcp_set_timeout::TCP_SET_TIMEOUT,
    &tcp_shutdown::TCP_SHUTDOWN,
    &tcp_write::TCP_WRITE,
    &udp_bind::UDP_BIND,
    &udp_close::UDP_CLOSE,
    &udp_connect::UDP_CONNECT,
    &udp_recv::UDP_RECV,
    &udp_recv_from::UDP_RECV_FROM,
    &udp_send::UDP_SEND,
    &udp_send_to::UDP_SEND_TO,
];
