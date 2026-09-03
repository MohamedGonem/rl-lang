//! `std::http` - a minimal HTTP server (`tiny_http`) and client (`ureq`).
//! Handle module: servers and requests are stored behind integer handles in the
//! runtime's handle table, accessed via the `HttpStore` trait (implemented by
//! each runtime).

#[cfg(feature = "impls")]
use rl_ast::statements::HandleKind;
#[cfg(feature = "impls")]
use rl_std_core::Runtime;
use rl_std_macros::native_fn;
#[cfg(feature = "impls")]
use rl_utils::errors::Error;

#[cfg(feature = "impls")]
/// A single native HTTP resource, stored behind an `int` handle.
/// (Moved here from the per-runtime copies so both share one definition.)
pub enum HttpHandle {
    Server(tiny_http::Server),
    Request(tiny_http::Request),
}

#[cfg(feature = "impls")]
/// Per-runtime access to the `http` handle table. Implemented by `VmRuntime` /
/// `EvalRuntime` in the runtime crates.
pub trait HttpStore: Runtime {
    fn http_insert(cx: &mut Self::Cx, h: HttpHandle) -> u64;
    fn http_get(cx: &Self::Cx, id: u64) -> Option<&HttpHandle>;
    fn http_get_mut(cx: &mut Self::Cx, id: u64) -> Option<&mut HttpHandle>;
    fn http_remove(cx: &mut Self::Cx, id: u64) -> Option<HttpHandle>;
}

#[cfg(feature = "impls")]
/// Inserts a handle and returns its rl handle value.
fn insert_handle<R: HttpStore>(cx: &mut R::Cx, h: HttpHandle) -> R::Value {
    let id = R::http_insert(cx, h);
    R::make_handle(HandleKind::Http, id)
}

// ---- shared argument extraction (reproducing the old `extract_*` helpers) --

#[cfg(feature = "impls")]
/// Reproduces the old `extract_string`: rejects non-strings with
/// `"<name>: expected string type, got <ty>"`.
fn extract_string<R: HttpStore>(v: &R::Value, name: &str) -> Result<String, String> {
    match R::as_str(v) {
        Some(s) => Ok(s.to_owned()),
        None => Err(format!(
            "{}: expected string type, got {}",
            name,
            R::type_name(v)
        )),
    }
}

#[cfg(feature = "impls")]
/// Reproduces the old `extract_handle`: unwraps a `Http` handle into its id,
/// with the same wrong-kind / not-a-handle messages.
fn extract_handle<R: HttpStore>(v: &R::Value, name: &str) -> Result<u64, String> {
    match R::as_handle(v, HandleKind::Http) {
        Some(id) => Ok(id),
        None => match R::as_handle(v, HandleKind::C)
            .map(|_| HandleKind::C)
            .or_else(|| R::as_handle(v, HandleKind::Net).map(|_| HandleKind::Net))
            .or_else(|| R::as_handle(v, HandleKind::Audio).map(|_| HandleKind::Audio))
            .or_else(|| R::as_handle(v, HandleKind::Gui).map(|_| HandleKind::Gui))
            .or_else(|| R::as_handle(v, HandleKind::File).map(|_| HandleKind::File))
        {
            Some(kind) => Err(format!(
                "{}: expected a {:?} handle, got a {:?} handle",
                name,
                HandleKind::Http,
                kind
            )),
            None => Err(format!(
                "{}: expected a handle, got {}",
                name,
                R::type_name(v)
            )),
        },
    }
}

#[cfg(feature = "impls")]
/// Reproduces the old `check_arity_range`: raises a runtime error when the
/// argument count is outside `[from, to]`.
fn check_arity_range<R: HttpStore>(
    cx: &R::Cx,
    args: &[R::Value],
    from: usize,
    to: usize,
    name: &str,
    span: R::Span,
) -> Result<(), Error> {
    let len = args.len();
    if !(from..=to).contains(&len) {
        return Err(R::error(
            cx,
            format!(
                "{}: expected from {} to {} arg(s), got {}",
                name, from, to, len
            ),
            span,
        ));
    }
    Ok(())
}

#[cfg(feature = "impls")]
/// Reproduces the client-side `ureq_result_to_value`: turns a `ureq` outcome
/// into `ok((status, body))` (including for HTTP error statuses) or
/// `err("<url>: <error>")`.
fn ureq_result_to_value<R: HttpStore>(
    url: &str,
    result: Result<ureq::Response, ureq::Error>,
) -> R::Value {
    match result {
        Ok(response) => {
            let status = response.status() as i64;
            let body = response.into_string().unwrap_or_default();
            R::ok(R::tuple(vec![R::from_i64(status), R::from_string(body)]))
        }
        Err(ureq::Error::Status(code, response)) => {
            let body = response.into_string().unwrap_or_default();
            R::ok(R::tuple(vec![
                R::from_i64(code as i64),
                R::from_string(body),
            ]))
        }
        Err(e) => R::err(R::from_string(format!("{}: {}", url, e))),
    }
}

// ---- server ----------------------------------------------------------------

#[native_fn(module = "http", bound = "HttpStore", sig(string -> result[handle(Http)]))]
pub fn http_server_start<R: HttpStore>(cx: &mut R::Cx, addr: String) -> R::Value {
    match tiny_http::Server::http(&addr) {
        Ok(server) => R::ok(insert_handle::<R>(cx, HttpHandle::Server(server))),
        Err(e) => R::err(R::from_string(format!(
            "http_server_start(\"{}\"): {}",
            addr, e
        ))),
    }
}

#[native_fn(module = "http", bound = "HttpStore", sig(handle(Http) -> result[handle(Http)]))]
pub fn http_server_recv<R: HttpStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "http_server_recv") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let recv_result = match R::http_get(cx, id) {
        Some(HttpHandle::Server(server)) => server.recv(),
        Some(_) => {
            return R::err(R::from_string(format!(
                "http_server_recv: handle {} is not a server",
                id
            )));
        }
        None => {
            return R::err(R::from_string(format!(
                "http_server_recv: unknown handle {}",
                id
            )));
        }
    };
    match recv_result {
        Ok(request) => R::ok(insert_handle::<R>(cx, HttpHandle::Request(request))),
        Err(e) => R::err(R::from_string(format!("http_server_recv: {}", e))),
    }
}

#[native_fn(module = "http", bound = "HttpStore", untyped)]
pub fn http_server_try_recv<R: HttpStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "http_server_try_recv") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let try_recv_result = match R::http_get(cx, id) {
        Some(HttpHandle::Server(server)) => server.try_recv(),
        Some(_) => {
            return R::err(R::from_string(format!(
                "http_server_try_recv(): handle {} is not a server",
                id
            )));
        }
        None => {
            return R::err(R::from_string(format!(
                "http_server_try_recv: unknown handle {}",
                id
            )));
        }
    };
    match try_recv_result {
        Ok(Some(request)) => R::ok(insert_handle::<R>(cx, HttpHandle::Request(request))),
        Ok(None) => R::ok(R::null()),
        Err(e) => R::err(R::from_string(format!("http_server_try_recv: {}", e))),
    }
}

#[native_fn(module = "http", bound = "HttpStore", sig(handle(Http) -> result[null]))]
pub fn http_server_stop<R: HttpStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "http_server_stop") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::http_get(cx, id) {
        Some(HttpHandle::Server(_)) => {
            R::http_remove(cx, id);
            R::ok(R::null())
        }
        Some(_) => R::err(R::from_string(format!(
            "http_server_stop: handle {} is not a server",
            id
        ))),
        None => R::err(R::from_string(format!(
            "http_server_stop: unknown handle {}",
            id
        ))),
    }
}

// ---- request accessors -----------------------------------------------------

#[native_fn(module = "http", bound = "HttpStore", sig(handle(Http) -> result[string]))]
pub fn http_request_method<R: HttpStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "http_request_method") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::http_get(cx, id) {
        Some(HttpHandle::Request(request)) => R::ok(R::from_string(request.method().to_string())),
        Some(_) => R::err(R::from_string(format!(
            "http_request_method: handle {} is not a request",
            id
        ))),
        None => R::err(R::from_string(format!(
            "http_request_method: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "http", bound = "HttpStore", sig(handle(Http) -> result[string]))]
pub fn http_request_url<R: HttpStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "http_request_url") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    match R::http_get(cx, id) {
        Some(HttpHandle::Request(request)) => R::ok(R::from_string(request.url().to_string())),
        Some(_) => R::err(R::from_string(format!(
            "http_request_url: handle {} is not a request",
            id
        ))),
        None => R::err(R::from_string(format!(
            "http_request_url: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "http", bound = "HttpStore", sig(handle(Http), string -> result[string]))]
pub fn http_request_header<R: HttpStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    name: String,
) -> R::Value {
    let id = match extract_handle::<R>(&handle, "http_request_header") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let request = match R::http_get(cx, id) {
        Some(HttpHandle::Request(request)) => request,
        Some(_) => {
            return R::err(R::from_string(format!(
                "http_request_header: handle {} is not a request",
                id
            )));
        }
        None => {
            return R::err(R::from_string(format!(
                "http_request_header: unknown handle {}",
                id
            )));
        }
    };
    let found = request
        .headers()
        .iter()
        .find(|h| h.field.to_string().eq_ignore_ascii_case(&name))
        .map(|h| h.value.to_string());
    match found {
        Some(value) => R::ok(R::from_string(value)),
        None => R::err(R::from_string(format!(
            "http_request_header: no \"{}\" header",
            name
        ))),
    }
}

#[native_fn(module = "http", bound = "HttpStore", sig(handle(Http) -> result[string]))]
pub fn http_request_body<R: HttpStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "http_request_body") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let request = match R::http_get_mut(cx, id) {
        Some(HttpHandle::Request(request)) => request,
        Some(_) => {
            return R::err(R::from_string(format!(
                "http_request_body(): handle {} is not a request",
                id
            )));
        }
        None => {
            return R::err(R::from_string(format!(
                "http_request_body(): unknown handle {}",
                id
            )));
        }
    };

    let mut body = String::new();
    match request.as_reader().read_to_string(&mut body) {
        Ok(_) => R::ok(R::from_string(body)),
        Err(e) => R::err(R::from_string(format!("http_request_body(): {}", e))),
    }
}

// ---- respond (raw: 3 or 4 args) --------------------------------------------

#[native_fn(module = "http", bound = "HttpStore",
    sig(handle(Http), int, string -> result[null]),
    sig(handle(Http), int, string, string -> result[null]))]
pub fn http_respond<R: HttpStore>(
    cx: &mut R::Cx,
    args: Vec<R::Value>,
    span: R::Span,
) -> Result<R::Value, Error> {
    check_arity_range::<R>(&*cx, &args, 3, 4, "http_post", span)?;

    let id = match extract_handle::<R>(&args[0], "http_respond") {
        Ok(id) => id,
        Err(e) => return Ok(R::err(R::from_string(e))),
    };

    let status = match R::as_i64(&args[1]) {
        Some(n) if (100..=599).contains(&n) => n as u16,
        _ => {
            return Ok(R::err(R::from_string(format!(
                "http_respond: expects a valid HTTP status int, got {}",
                R::type_name(&args[1])
            ))));
        }
    };

    let body = match extract_string::<R>(&args[2], "http_respond") {
        Ok(s) => s,
        Err(e) => return Ok(R::err(R::from_string(e.to_string()))),
    };
    let content_type = match args.get(3) {
        Some(v) if R::as_str(v).is_some() => Some(R::as_str(v).unwrap().to_string()),
        Some(other) => {
            return Ok(R::err(R::from_string(format!(
                "http_respond: expects a string content_type, got {}",
                R::type_name(other)
            ))));
        }
        None => None,
    };

    // Type-check without removing first: the old code removed the handle,
    // re-inserting it under the same id when it turned out not to be a request.
    // `HttpStore::http_insert` allocates a fresh id, so we instead check via
    // `http_get` and only `http_remove` once the request is confirmed - which
    // preserves the id of a wrongly-typed handle.
    match R::http_get(cx, id) {
        Some(HttpHandle::Request(_)) => {}
        Some(_) => {
            return Ok(R::err(R::from_string(format!(
                "http_respond: handle {} is not a request",
                id
            ))));
        }
        None => {
            return Ok(R::err(R::from_string(format!(
                "http_respond: unknown handle {}",
                id
            ))));
        }
    }
    let request = match R::http_remove(cx, id) {
        Some(HttpHandle::Request(request)) => request,
        // unreachable: just type-checked above.
        _ => {
            return Ok(R::err(R::from_string(format!(
                "http_respond: unknown handle {}",
                id
            ))));
        }
    };

    let mut response = tiny_http::Response::from_string(body).with_status_code(status);
    if let Some(ct) = content_type
        && let Ok(header) = tiny_http::Header::from_bytes(&b"Content-Type"[..], ct.as_bytes())
    {
        response = response.with_header(header);
    }

    match request.respond(response) {
        Ok(()) => Ok(R::ok(R::null())),
        Err(e) => Ok(R::err(R::from_string(format!("http_respond: {}", e)))),
    }
}

// ---- client ----------------------------------------------------------------

#[native_fn(module = "http", bound = "HttpStore", sig(string -> result[tuple[int, string]]))]
pub fn http_get<R: HttpStore>(_cx: &mut R::Cx, url: String) -> R::Value {
    let result = ureq::get(&url).call();
    ureq_result_to_value::<R>(&url, result)
}

#[native_fn(module = "http", bound = "HttpStore",
    sig(string, string -> result[tuple[int, string]]),
    sig(string, string, string -> result[tuple[int, string]]))]
pub fn http_post<R: HttpStore>(
    cx: &mut R::Cx,
    args: Vec<R::Value>,
    span: R::Span,
) -> Result<R::Value, Error> {
    check_arity_range::<R>(&*cx, &args, 2, 3, "http_post", span)?;

    let url = match extract_string::<R>(&args[0], "http_post") {
        Ok(s) => s,
        Err(e) => return Ok(R::err(R::from_string(format!("http_post: {e}")))),
    };
    let body = match extract_string::<R>(&args[1], "http_post") {
        Ok(s) => s,
        Err(e) => return Ok(R::err(R::from_string(format!("http_post: {e}")))),
    };

    let content_type = match args.get(2) {
        Some(v) if R::as_str(v).is_some() => R::as_str(v).unwrap().to_string(),
        Some(other) => {
            return Ok(R::err(R::from_string(format!(
                "http_post: expects a string content_type, got {}",
                R::type_name(other)
            ))));
        }
        None => "text/plain".to_string(),
    };

    let result = ureq::post(&url)
        .set("Content-Type", &content_type)
        .send_string(&body);
    Ok(ureq_result_to_value::<R>(&url, result))
}

#[native_fn(module = "http", bound = "HttpStore",
    sig(string, string -> result[tuple[int, string]]),
    sig(string, string, string -> result[tuple[int, string]]),
    sig(string, string, string, array[tuple[string, string]] -> result[tuple[int, string]]))]
pub fn http_request<R: HttpStore>(
    cx: &mut R::Cx,
    args: Vec<R::Value>,
    span: R::Span,
) -> Result<R::Value, Error> {
    check_arity_range::<R>(&*cx, &args, 2, 4, "http_request", span)?;

    let method = match extract_string::<R>(&args[0], "http_request") {
        Ok(s) => s,
        Err(e) => return Ok(R::err(R::from_string(format!("http_request: {e}")))),
    };
    let url = match extract_string::<R>(&args[1], "http_request") {
        Ok(s) => s,
        Err(e) => return Ok(R::err(R::from_string(format!("http_request: {e}")))),
    };
    let body = match args.get(2) {
        Some(v) if R::as_str(v).is_some() => Some(R::as_str(v).unwrap().to_string()),
        Some(other) => {
            return Ok(R::err(R::from_string(format!(
                "http_request: expects a string body, got {}",
                R::type_name(other)
            ))));
        }
        None => None,
    };

    let mut request = ureq::request(&method, &url);
    if let Some((items, _)) = args.get(3).and_then(|v| R::as_array(v)) {
        for item in items.iter() {
            match tuple_pair::<R>(item) {
                Some((name, value)) => {
                    request = request.set(&name, &value);
                    continue;
                }
                None => {
                    return Ok(R::err(R::from_string(
                        "http_request: expects headers as an array of (string, string) tuples"
                            .to_string(),
                    )));
                }
            }
        }
    } else if args.get(3).is_some() {
        return Ok(R::err(R::from_string(
            "http_request: expects headers as an array of (string, string) tuples".to_string(),
        )));
    }

    let result = match &body {
        Some(b) => request.send_string(b),
        None => request.call(),
    };
    Ok(ureq_result_to_value::<R>(&url, result))
}

#[cfg(feature = "impls")]
/// Deconstructs a `(string, string)` tuple value into its two strings, matching
/// the old `VmValue::Tuple(pair) if pair.len() == 2` + `(Str, Str)` pattern.
///
/// NOTE: the `Runtime` trait exposes no tuple accessor (`as_array` matches only
/// `Arr`/`Values`, not `Tuple`), so this falls back to `as_array`. That means a
/// tuple header pair is currently NOT recognized by either runtime and this
/// returns `None` for it - a behavioral regression versus the old code. Add an
/// `as_tuple` (or similar) method to `Runtime` to restore parity, then switch
/// this helper to it.
fn tuple_pair<R: HttpStore>(v: &R::Value) -> Option<(String, String)> {
    let (items, _) = R::as_array(v)?;
    if items.len() != 2 {
        return None;
    }
    let name = R::as_str(&items[0])?.to_string();
    let value = R::as_str(&items[1])?.to_string();
    Some((name, value))
}

rl_std_core::native_module!("http";
    bound: HttpStore;
    funcs: [
        http_server_start, http_server_recv, http_server_try_recv,
        http_request_method, http_request_url, http_request_header,
        http_request_body, http_respond, http_server_stop,
        http_get, http_post, http_request,
    ],
);
