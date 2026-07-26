use crate::entry::FnEntry;

pub static SOUND_GET_VOLUME: FnEntry = FnEntry {
    signature: "sound_get_volume(handle)",
    description: "returns the volume last set with sound_set_volume (defaults to 1.0), independent of set_master_volume",
    example: r#"get std::audio::sound_get_volume
get std::res::result_unwrap

dec float volume = result_unwrap(sound_get_volume(sound))"#,
    expected_output: None,
    returns: "result[float]",
    errors: Some("err(string) when the handle is unknown"),
    see_also: &["sound_set_volume"],
    since: Some("v0.4.0"),
};
