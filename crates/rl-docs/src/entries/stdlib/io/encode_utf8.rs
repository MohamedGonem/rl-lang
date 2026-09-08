use crate::entry::FnEntry;

pub static ENCODE_UTF8: FnEntry = FnEntry {
    signature: "encode_utf8(string)",
    description: "encodes a string into a UTF-8 byte array",
    example: r#"get std::io::encode_utf8

dec arr[byte] bytes = encode_utf8("Hello")?"#,
    expected_output: None,
    returns: "arr[byte]",
    errors: None,
    see_also: &["decode_utf8"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
