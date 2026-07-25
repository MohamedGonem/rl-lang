use crate::entry::FnEntry;

pub static CALL: FnEntry = FnEntry {
    signature: "call(handle, fn_name, args, arg_types, ret_type)",
    description: "calls `fn_name` in the library behind `handle` via libffi. `args` is a *tuple* (`(...)`, not `[...]`) since C argument lists mix types and rl-lang arrays are homogeneous. `arg_types` is an `arr[string]` matched positionally to `args`; each entry is one of `\"i32\"`/`\"i64\"`/`\"i16\"`/`\"u8\"`/`\"f32\"`/`\"f64\"`/`\"bool\"`, a mutable `\"str:N\"` (an in/out `char[N]` buffer, `N` a byte capacity including the null terminator), or a mutable `\"arr:TYPE:N\"` (an in/out numeric buffer, `TYPE` one of `i32`/`i64`/`f32`/`f64`/`u8`/`i16`, `N` an *exact* element count - unlike strings there's no terminator convention for numbers, so the array's length must match `N` exactly). `ret_type` is one of the same numeric names or `\"void\"` - never `\"str\"`/`\"arr\"`, since a C-returned pointer's ownership (static vs `malloc`'d, freed how) can't be known safely. With no `\"str:N\"`/`\"arr:TYPE:N\"` args, `call` returns the plain scalar named by `ret_type`; with one or more, it returns `(ret, outs)`, where `outs` is itself a tuple holding the post-call buffer contents, in the order those args appeared. rl-lang values are never mutated in place either way (rl-lang arrays/strings have no shared/reference semantics) - `outs` is how a mutation crosses back explicitly. `call` itself is registered untyped in the checker (its return shape depends on the runtime `ret_type`/`arg_types` strings, which the checker can't evaluate), so a bare `call(...)` site gets none of the usual static checking - a typo in `arg_types`, a mismatched arg count, or a wrong `ret_type` all only surface as a runtime `Err`. Wrapping it in a named `fn` or a `fn(...) {...}` lambda with its own declared signature gets that checking back for the wrapper's own params/return type, and reads like calling an ordinary function at every call site afterward - see the second and third examples below. Prefer one of those over calling `call` directly wherever the same C function gets called more than once",
    example: r#"
get std::c::compile
get std::c::call
get std::c::close

dec int h = result_unwrap(compile("int add(int a, int b) { return a + b; }"))

// 1. Bare call - works, but untyped: nothing here catches a wrong
//    arg_types entry, a swapped arg order, or a typo'd ret_type until
//    the call actually runs.
dec int sum = result_unwrap(call(h, "add", (3, 4), ["i32", "i32"], "i32"))

// 2. Wrapped as a named fn - the checker validates every call to add()
//    against this declared signature from here on.
fn add(int a, int b) -> result[int] {
    return call(h, "add", (a, b), ["i32", "i32"], "i32")
}
dec int sum2 = add(3, 4)?

// 3. Wrapped as a lambda - same benefit, assignable to a variable,
//    handy when the binding itself (not just the call site) needs to
//    be passed around.
dec add_fn = fn(int a, int b) -> result[int] {
    return call(h, "add", (a, b), ["i32", "i32"], "i32")
}
dec sum3 = add_fn(3, 4)?

close(h)"#,
    expected_output: None,
    returns: "result[T] matching ret_type, or result[(T, (...))] when any str:N/arr:TYPE:N arg is used",
    errors: Some(
        "err(string) for an unknown handle, a missing symbol, an args/arg_types length mismatch, a value that doesn't match its declared arg type, an out-of-range int for u8/i16/arr elements, a string or array that doesn't fit its declared str:N/arr:TYPE:N capacity exactly, or an unsupported arg_types/ret_type name",
    ),
    see_also: &["compile", "load", "close"],
    since: Some("v0.4.0"),
};
