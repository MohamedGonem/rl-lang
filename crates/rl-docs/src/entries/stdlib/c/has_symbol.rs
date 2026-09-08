use crate::entry::FnEntry;

pub static HAS_SYMBOL: FnEntry = FnEntry {
    signature: "has_symbol(handle, fn_name)",
    description: "checks whether `fn_name` exists in the library behind `handle`, without calling it - useful for libraries that expose different optional symbols depending on how they were built. Unlike `call`, a missing symbol here is a normal `Ok(false)`, not an error; `Err` is reserved for an unknown `handle`",
    example: r#"get std::c::load, std::c::has_symbol

dec handle h = result_unwrap(load("libm.so.6"))
dec bool found = result_unwrap(has_symbol(h, "sqrt"))"#,
    expected_output: None,
    returns: "result[bool]",
    errors: Some("err(string) if `handle` is unknown"),
    see_also: &["compile", "load", "call", "close"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
