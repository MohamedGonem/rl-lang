use crate::entry::FnEntry;

pub static SOUND_RESUME: FnEntry = FnEntry {
    signature: "sound_resume(handle)",
    description: "resumes a sound previously paused with sound_pause",
    example: r#"get std::audio::sound_resume
get std::res::result_unwrap

result_unwrap(sound_resume(sound))"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) when the handle is unknown"),
    see_also: &["sound_pause", "sound_is_paused"],
    since: Some("v0.4.0"),
};
