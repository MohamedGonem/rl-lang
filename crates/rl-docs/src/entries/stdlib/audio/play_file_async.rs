use crate::entry::FnEntry;

pub static PLAY_FILE_ASYNC: FnEntry = FnEntry {
    signature: "play_file_async(path)",
    description: "starts playing an audio file without blocking and returns a sound handle",
    example: r#"get play_file_async, sound_wait from std::audio
get std::res::result_unwrap

dec handle sound = result_unwrap(play_file_async("assets/music.ogg"))
result_unwrap(sound_wait(sound))"#,
    expected_output: None,
    returns: "result[handle]",
    errors: Some(
        "err(string) when the file can't be opened, decoded, or no output device is available",
    ),
    see_also: &["play_file", "sound_wait", "sound_stop"],
    since: Some("v0.4.0"),
};
