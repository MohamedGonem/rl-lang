use crate::entry::FnEntry;

pub static TAN: FnEntry = FnEntry {
    signature: "tan(x)",
    description: "tangent of x in radians",
    example: "get std::math::tan\n\ntan(0.0)",
    expected_output: Some("0.0"),
    returns: "float",
    errors: None,
    see_also: &["sin", "cos"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
