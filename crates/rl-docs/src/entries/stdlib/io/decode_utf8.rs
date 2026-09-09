use crate::entry::FnEntry;

pub static DECODE_UTF8: FnEntry = FnEntry {
    signature: "decode_utf8(bytes)",
    description: "decodes a byte array into a UTF-8 string",
    example: r#"get std::io::decode_utf8

dec string s = decode_utf8([72, 101, 108, 108, 111])?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error if the bytes are not valid UTF-8"),
    see_also: &["encode_utf8"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
