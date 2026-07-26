use crate::entry::FnEntry;

pub static SOUND_SET_SPEED: FnEntry = FnEntry {
    signature: "sound_set_speed(handle, speed)",
    description: "sets a sound's playback speed (and pitch), where 1.0 is normal speed",
    example: r#"get std::audio::sound_set_speed
get std::res::result_unwrap

result_unwrap(sound_set_speed(sound, 1.5))"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) when the handle is unknown"),
    see_also: &["sound_set_volume"],
    since: Some("v0.4.0"),
};
