use crate::entry::FnEntry;

pub static TCP_WRITE: FnEntry = FnEntry {
    signature: "tcp_write(stream, data)",
    description: "writes `data` to a TCP stream, returning the number of bytes written",
    example: r#"get std::net::tcp_connect
get std::net::tcp_write

dec handle stream = result_unwrap(tcp_connect("example.com:80"))
result_unwrap(tcp_write(stream, "hello\n"))"#,
    expected_output: None,
    returns: "Result[int]",
    errors: Some("Err(string) on a write error"),
    see_also: &["tcp_read"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
