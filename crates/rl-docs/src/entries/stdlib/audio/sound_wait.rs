use crate::entry::FnEntry;

pub static SOUND_WAIT: FnEntry = FnEntry {
    signature: "sound_wait(handle)",
    description: "blocks the current thread until a sound started with play_file_async finishes playing",
    example: r#"get play_file_async, sound_wait from std::audio
get std::res::result_unwrap

dec handle sound = result_unwrap(play_file_async("assets/music.ogg"))
result_unwrap(sound_wait(sound))"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) when the handle is unknown"),
    see_also: &["play_file_async", "sound_is_finished"],
    since: Some("v0.4.1"),
    deprecated: None,
    updated: Some("v0.4.1"),
};
