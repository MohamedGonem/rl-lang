use crate::entry::FnEntry;

pub static TCP_SET_TIMEOUT: FnEntry = FnEntry {
    signature: "tcp_set_timeout(stream, millis)",
    description: "sets both the read and write timeout on a TCP stream; `0` clears the timeout (blocks indefinitely again)",
    example: r#"get std::net::tcp_connect
get std::net::tcp_set_timeout

dec handle stream = result_unwrap(tcp_connect("example.com:80"))
result_unwrap(tcp_set_timeout(stream, 5000))"#,
    expected_output: None,
    returns: "Result[null]",
    errors: Some("Err(string) if the timeout can't be set"),
    see_also: &["tcp_set_nonblocking"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
