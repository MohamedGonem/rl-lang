use crate::entry::FnEntry;

pub static HTTP_SERVER_STOP: FnEntry = FnEntry {
    signature: "http_server_stop(server)",
    description: "stops and closes an HTTP server handle",
    example: r#"get std::http::http_server_start
get std::http::http_server_stop

dec handle server = result_unwrap(http_server_start("0.0.0.0:8080"))
http_server_stop(server)"#,
    expected_output: None,
    returns: "Result[null]",
    errors: None,
    see_also: &["http_server_start"],
    since: Some("v0.1.5"),
};
