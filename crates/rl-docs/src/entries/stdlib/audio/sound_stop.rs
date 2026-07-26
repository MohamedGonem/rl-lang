use crate::entry::FnEntry;

pub static SOUND_STOP: FnEntry = FnEntry {
    signature: "sound_stop(handle)",
    description: "stops a sound and releases its handle; the handle can't be used again afterwards",
    example: r#"get std::audio::sound_stop
get std::res::result_unwrap

result_unwrap(sound_stop(sound))"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) when the handle is unknown"),
    see_also: &["sound_pause", "play_file_async"],
    since: Some("v0.4.0"),
};
