use crate::entry::FnEntry;

pub static HTTP_REQUEST_URL: FnEntry = FnEntry {
    signature: "http_request_url(req)",
    description: "returns the requested path, including query string",
    example: r#"get std::http::http_server_start
get std::http::http_server_recv
get std::http::http_request_url

dec handle server = result_unwrap(http_server_start("0.0.0.0:8080"))
dec handle req = result_unwrap(http_server_recv(server))
http_request_url(req)"#,
    expected_output: None,
    returns: "Result[string]",
    errors: None,
    see_also: &["http_request_method"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
