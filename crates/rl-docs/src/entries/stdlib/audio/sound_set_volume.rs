use crate::entry::FnEntry;

pub static SOUND_SET_VOLUME: FnEntry = FnEntry {
    signature: "sound_set_volume(handle, volume)",
    description: "sets a sound's volume, where 1.0 is the original recorded volume; scaled further by set_master_volume",
    example: r#"get play_file_async, sound_set_volume from std::audio
get std::res::result_unwrap

dec handle sound = result_unwrap(play_file_async("assets/music.ogg"))
result_unwrap(sound_set_volume(sound, 0.5))"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) when the handle is unknown"),
    see_also: &["sound_get_volume", "set_master_volume"],
    since: Some("v0.4.0"),
};
