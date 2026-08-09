use crate::entry::FnEntry;

pub static LIST_OUTPUT_DEVICES: FnEntry = FnEntry {
    signature: "list_output_devices()",
    description: "returns the names of all available audio output devices",
    example: r#"get std::audio::list_output_devices
get std::res::result_unwrap

dec arr[string] devices = result_unwrap(list_output_devices())"#,
    expected_output: None,
    returns: "result[arr[string]]",
    errors: Some("err(string) when the host audio system can't be queried"),
    see_also: &["set_output_device"],
    since: Some("v0.4.0"),
};
