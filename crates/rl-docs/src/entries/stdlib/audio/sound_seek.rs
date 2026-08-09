use crate::entry::FnEntry;

pub static SOUND_SEEK: FnEntry = FnEntry {
    signature: "sound_seek(handle, position_ms)",
    description: "seeks a sound to the given position, in milliseconds from the start",
    example: r#"get play_file_async, sound_seek from std::audio
get std::res::result_unwrap

dec handle sound = result_unwrap(play_file_async("assets/music.ogg"))
result_unwrap(sound_seek(sound, 5000))"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) when the handle is unknown or the format doesn't support seeking"),
    see_also: &["sound_wait", "audio_duration"],
    since: Some("v0.4.0"),
};
