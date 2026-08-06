use crate::entry::FnEntry;

pub static PLAY_FILE: FnEntry = FnEntry {
    signature: "play_file(path)",
    description: "plays an audio file and blocks until playback finishes",
    example: r#"get std::audio::play_file
get std::res::result_unwrap

result_unwrap(play_file("assets/beep.wav"))"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        "err(string) when the file can't be opened, decoded, or no output device is available",
    ),
    see_also: &["play_file_async", "beep"],
    since: Some("v0.4.0"),
};
