use crate::entry::FnEntry;

pub static SET_OUTPUT_DEVICE: FnEntry = FnEntry {
    signature: "set_output_device(name)",
    description: "selects the audio output device used by sounds started afterwards; doesn't affect sounds already playing",
    example: r#"get std::audio::set_output_device
get std::res::result_unwrap

result_unwrap(set_output_device("Speakers"))"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) when no device with that name exists"),
    see_also: &["list_output_devices"],
    since: Some("v0.4.0"),
};
