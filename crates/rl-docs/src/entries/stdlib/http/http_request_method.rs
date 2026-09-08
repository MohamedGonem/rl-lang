use crate::entry::FnEntry;

pub static HTTP_REQUEST_METHOD: FnEntry = FnEntry {
    signature: "http_request_method(req)",
    description: "returns the HTTP method of a request (e.g. \"GET\", \"POST\")",
    example: r#"get std::http::http_server_start
get std::http::http_server_recv
get std::http::http_request_method

dec handle server = result_unwrap(http_server_start("0.0.0.0:8080"))
dec handle req = result_unwrap(http_server_recv(server))
http_request_method(req)"#,
    expected_output: None,
    returns: "Result[string]",
    errors: None,
    see_also: &["http_request_url"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
