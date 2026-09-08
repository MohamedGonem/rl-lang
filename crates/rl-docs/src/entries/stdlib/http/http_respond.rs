use crate::entry::FnEntry;

pub static HTTP_RESPOND: FnEntry = FnEntry {
    signature: "http_respond(req, status, body, content_type?)",
    description: "sends a response for `req` and consumes the request handle; using the handle again afterward errors",
    example: r#"get std::http::http_server_start
get std::http::http_server_recv
get std::http::http_respond

dec handle server = result_unwrap(http_server_start("0.0.0.0:8080"))
dec handle req = result_unwrap(http_server_recv(server))
http_respond(req, 200, "hello world", "text/plain")"#,
    expected_output: None,
    returns: "Result[null]",
    errors: Some("Err(string) on a write/send failure"),
    see_also: &["http_server_recv"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
