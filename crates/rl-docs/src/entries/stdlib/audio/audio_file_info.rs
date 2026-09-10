use crate::entry::FnEntry;

pub static AUDIO_FILE_INFO: FnEntry = FnEntry {
    signature: "audio_file_info(path)",
    description: "returns (channels, sample_rate, duration_ms, format) for an audio file, without playing it",
    example: r#"get std::audio::audio_file_info
get std::res::result_unwrap

dec (int, int, int, string) info = result_unwrap(audio_file_info("assets/music.ogg"))"#,
    expected_output: None,
    returns: "result[(int, int, int, string)]",
    errors: Some("err(string) when the file can't be opened or probed"),
    see_also: &["audio_duration"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
