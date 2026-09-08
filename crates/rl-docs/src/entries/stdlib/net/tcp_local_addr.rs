use crate::entry::FnEntry;

pub static TCP_LOCAL_ADDR: FnEntry = FnEntry {
    signature: "tcp_local_addr(stream)",
    description: "returns the local address of a connected TCP stream",
    example: r#"get std::net::tcp_connect
get std::net::tcp_local_addr

dec handle stream = result_unwrap(tcp_connect("example.com:80"))
tcp_local_addr(stream)"#,
    expected_output: None,
    returns: "Result[string]",
    errors: None,
    see_also: &["tcp_peer_addr"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
