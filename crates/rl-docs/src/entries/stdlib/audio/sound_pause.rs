use crate::entry::FnEntry;

pub static SOUND_PAUSE: FnEntry = FnEntry {
    signature: "sound_pause(handle)",
    description: "pauses playback of a sound started with play_file_async",
    example: r#"get play_file_async, sound_pause from std::audio
get std::res::result_unwrap

dec handle sound = result_unwrap(play_file_async("assets/music.ogg"))
result_unwrap(sound_pause(sound))"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) when the handle is unknown"),
    see_also: &["sound_resume", "sound_is_paused"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
