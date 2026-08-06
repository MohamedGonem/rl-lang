use crate::entry::FnEntry;

pub static BEEP: FnEntry = FnEntry {
    signature: "beep(freq, duration_ms)",
    description: "plays a sine-wave tone at `freq` Hz for `duration_ms` milliseconds, blocking until it finishes",
    example: r#"get std::audio::beep
get std::res::result_unwrap

result_unwrap(beep(440.0, 250))"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) when no output device is available"),
    see_also: &["play_file"],
    since: Some("v0.4.0"),
};
