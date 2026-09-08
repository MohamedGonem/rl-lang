use crate::entry::{FnEntry, StdEntry};

pub static LEN: StdEntry = StdEntry {
    name: "len",
    description: "root-level length function",
    functions: FUNCTIONS,
    since: Some("v2.1.0"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[&STD_LEN];

static STD_LEN: FnEntry = FnEntry {
    signature: "len(x)",
    description: "returns the length of a string, array, or tuple",
    example: "dec my_arr = [1, 2, 3]\nstd::io::println(len(my_arr))",
    expected_output: Some("3"),
    returns: "int",
    errors: Some(
        "Will panic at runtime (not a catchable `result[..]` err) if `x` is not a\nstring, array, or tuple.",
    ),
    see_also: &["arr_count", "arr_is_empty"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
