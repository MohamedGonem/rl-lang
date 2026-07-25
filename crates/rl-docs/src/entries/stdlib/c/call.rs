use crate::entry::FnEntry;

pub static CALL: FnEntry = FnEntry {
    signature: "call(handle, fn_name, args, arg_types, ret_type)",
    description: "calls `fn_name` in the library behind `handle` via libffi. `args` is a *tuple* (`(...)`, not `[...]`) since C argument lists mix types and rl-lang arrays are homogeneous. `arg_types` is an `arr[string]` matched positionally to `args`; each entry is one of `\"i32\"`/`\"i64\"`/`\"i16\"`/`\"u8\"`/`\"f32\"`/`\"f64\"`/`\"bool\"`, a mutable `\"str:N\"` (an in/out `char[N]` buffer, `N` a byte capacity including the null terminator), or a mutable `\"arr:TYPE:N\"` (an in/out numeric buffer, `TYPE` one of `i32`/`i64`/`f32`/`f64`/`u8`/`i16`, `N` an *exact* element count - unlike strings there's no terminator convention for numbers, so the array's length must match `N` exactly). `ret_type` is one of the same numeric names or `\"void\"` - never `\"str\"`/`\"arr\"`, since a C-returned pointer's ownership (static vs `malloc`'d, freed how) can't be known safely. With no `\"str:N\"`/`\"arr:TYPE:N\"` args, `call` returns the plain scalar named by `ret_type`; with one or more, it returns `(ret, outs)`, where `outs` is itself a tuple holding the post-call buffer contents, in the order those args appeared. rl-lang values are never mutated in place either way (rl-lang arrays/strings have no shared/reference semantics) - `outs` is how a mutation crosses back explicitly",
    example: r#"
get std::c::compile
get std::c::call
get std::c::close

dec int h = std::res::result_unwrap(compile("int add(int a, int b) { return a + b; }"))
dec int sum = std::res::result_unwrap(call(h, "add", (3, 4), ["i32", "i32"], "i32"))
close(h)"#,
    expected_output: None,
    returns: "result[T] matching ret_type, or result[(T, (...))] when any str:N/arr:TYPE:N arg is used",
    errors: Some(
        "err(string) for an unknown handle, a missing symbol, an args/arg_types length mismatch, a value that doesn't match its declared arg type, an out-of-range int for u8/i16/arr elements, a string or array that doesn't fit its declared str:N/arr:TYPE:N capacity exactly, or an unsupported arg_types/ret_type name",
    ),
    see_also: &["compile", "load", "close"],
    since: Some("v0.4.0"),
};
