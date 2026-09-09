use crate::entry::FnEntry;

pub static UDP_SEND: FnEntry = FnEntry {
    signature: "udp_send(socket, data)",
    description: "sends `data` to the socket's connected peer (see `udp_connect`)",
    example: r#"get std::net::udp_bind
get std::net::udp_send

dec handle socket = result_unwrap(udp_bind("127.0.0.1:9000"))
result_unwrap(udp_send(socket, "ping"))"#,
    expected_output: None,
    returns: "Result[int]",
    errors: Some("Err(string) if the socket has no connected peer, or on send failure"),
    see_also: &["udp_connect", "udp_send_to"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
