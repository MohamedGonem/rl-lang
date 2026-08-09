use crate::entry::FnEntry;

pub static SOUND_STOP: FnEntry = FnEntry {
    signature: "sound_stop(handle)",
    description: "stops a sound and releases its handle; the handle can't be used again afterwards",
    example: r#"get play_file_async, sound_stop from std::audio
get std::res::result_unwrap

dec handle sound = result_unwrap(play_file_async("assets/music.ogg"))
result_unwrap(sound_stop(sound))"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) when the handle is unknown"),
    see_also: &["sound_pause", "play_file_async"],
    since: Some("v0.4.0"),
};
