use crate::entry::FnEntry;

pub static TCP_CLOSE: FnEntry = FnEntry {
    signature: "tcp_close(handle)",
    description: "closes a TCP listener or stream handle and frees its slot; using the handle again afterward errors",
    example: r#"get std::net::tcp_connect
get std::net::tcp_close

dec handle stream = result_unwrap(tcp_connect("example.com:80"))
tcp_close(stream)"#,
    expected_output: None,
    returns: "Result[null]",
    errors: None,
    see_also: &["tcp_shutdown"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
