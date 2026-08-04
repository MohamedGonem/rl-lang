use crate::entry::FnEntry;

pub static SOUND_IS_FINISHED: FnEntry = FnEntry {
    signature: "sound_is_finished(handle)",
    description: "returns whether a sound has finished playing (its queue is empty)",
    example: r#"get play_file_async, sound_is_finished from std::audio
get std::res::result_unwrap

dec handle sound = result_unwrap(play_file_async("assets/music.ogg"))
dec bool done = result_unwrap(sound_is_finished(sound))"#,
    expected_output: None,
    returns: "result[bool]",
    errors: Some("err(string) when the handle is unknown"),
    see_also: &["sound_wait", "sound_stop"],
    since: Some("v0.4.0"),
};
