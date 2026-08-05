use crate::entry::FnEntry;

pub static CLOSE: FnEntry = FnEntry {
    signature: "close(handle)",
    description: "unloads the library behind `handle` (`dlclose`) and removes it from the handle table - works the same whether `handle` came from `compile` or `load`",
    example: r#"get std::c::compile, std::c::close

dec handle h = std::res::result_unwrap(compile("void noop() {}"))
close(h)"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) if `handle` is unknown (already closed, or never valid)"),
    see_also: &["compile", "load", "call"],
    since: Some("v0.4.1"),
};
