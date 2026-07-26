use crate::entry::FnEntry;

pub static SET_MASTER_VOLUME: FnEntry = FnEntry {
    signature: "set_master_volume(volume)",
    description: "sets a global volume scalar applied on top of every sound's own volume, including sounds already playing",
    example: r#"get std::audio::set_master_volume
get std::res::result_unwrap

result_unwrap(set_master_volume(0.5))"#,
    expected_output: None,
    returns: "result[null]",
    errors: None,
    see_also: &["sound_set_volume"],
    since: Some("v0.4.0"),
};
