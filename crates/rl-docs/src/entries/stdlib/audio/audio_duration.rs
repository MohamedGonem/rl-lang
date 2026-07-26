use crate::entry::FnEntry;

pub static AUDIO_DURATION: FnEntry = FnEntry {
    signature: "audio_duration(path)",
    description: "returns an audio file's duration in milliseconds, without playing it",
    example: r#"get std::audio::audio_duration
get std::res::result_unwrap

dec int ms = result_unwrap(audio_duration("assets/music.ogg"))"#,
    expected_output: None,
    returns: "result[int]",
    errors: Some("err(string) when the file can't be opened or probed"),
    see_also: &["audio_file_info"],
    since: Some("v0.4.0"),
};
