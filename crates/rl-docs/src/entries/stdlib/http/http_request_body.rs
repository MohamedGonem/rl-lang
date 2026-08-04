use crate::entry::FnEntry;

pub static HTTP_REQUEST_BODY: FnEntry = FnEntry {
    signature: "http_request_body(req)",
    description: "reads the request body as a string; the body can only be read once",
    example: r#"get std::http::http_server_start
get std::http::http_server_recv
get std::http::http_request_body

dec handle server = result_unwrap(http_server_start("0.0.0.0:8080"))
dec handle req = result_unwrap(http_server_recv(server))
dec string body = result_unwrap(http_request_body(req))"#,
    expected_output: None,
    returns: "Result[string]",
    errors: Some("Err(string) on a read error"),
    see_also: &["http_request_header"],
    since: Some("v0.1.5"),
};
