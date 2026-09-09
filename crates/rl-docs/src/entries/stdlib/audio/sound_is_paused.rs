use crate::entry::FnEntry;

pub static SOUND_IS_PAUSED: FnEntry = FnEntry {
    signature: "sound_is_paused(handle)",
    description: "returns whether a sound is currently paused",
    example: r#"get play_file_async, sound_is_paused from std::audio
get std::res::result_unwrap

dec handle sound = result_unwrap(play_file_async("assets/music.ogg"))
dec bool paused = result_unwrap(sound_is_paused(sound))"#,
    expected_output: None,
    returns: "result[bool]",
    errors: Some("err(string) when the handle is unknown"),
    see_also: &["sound_pause", "sound_resume"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
