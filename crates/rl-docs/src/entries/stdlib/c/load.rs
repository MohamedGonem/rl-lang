use crate::entry::FnEntry;

pub static LOAD: FnEntry = FnEntry {
    signature: "load(path)",
    description: "`dlopen`s an already-built `.so`/`.dylib`/`.dll` at `path` directly, no compiler involved; shares the same handle table as `compile`, so `call`/`close` work identically on a handle from either one",
    example: r#"get std::c::load

dec handle h = std::res::result_unwrap(load("libm.so.6"))"#,
    expected_output: None,
    returns: "result[handle]",
    errors: Some("err(string) if the library at `path` can't be found or loaded"),
    see_also: &["compile", "call", "close"],
    since: Some("v0.4.0"),
};
