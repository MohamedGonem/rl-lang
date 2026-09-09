use crate::entry::FnEntry;

pub static COMPILE: FnEntry = FnEntry {
    signature: "compile(source)",
    description: "compiles C `source` with a system compiler (`cc`/`clang`/`gcc`, or `$CC`) into a shared library, caching the result by content hash, then `dlopen`s it; returns a library handle for use with `call`/`close`. No compiler is bundled, so this requires one to already be on `$PATH` (or `$CC` set explicitly)",
    example: r#"get std::c::compile

dec handle h = std::res::result_unwrap(compile("int add(int a, int b) { return a + b; }"))"#,
    expected_output: None,
    returns: "result[handle]",
    errors: Some(
        "err(string) if no C compiler is found on $PATH/$CC, compilation fails (compiler diagnostics are included), or the resulting library fails to load",
    ),
    see_also: &["load", "call", "close"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
