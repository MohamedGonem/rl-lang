use crate::entry::{FnEntry, StdEntry};

mod audio_duration;
mod audio_file_info;
mod beep;
mod list_output_devices;
mod play_file;
mod play_file_async;
mod set_master_volume;
mod set_output_device;
mod sound_get_volume;
mod sound_is_finished;
mod sound_is_paused;
mod sound_pause;
mod sound_resume;
mod sound_seek;
mod sound_set_speed;
mod sound_set_volume;
mod sound_stop;
mod sound_wait;

pub static AUDIO: StdEntry = StdEntry {
    name: "audio",
    description: "audio playback built on rodio; blocking and handle-based non-blocking playback, volume/speed/seek control, and file metadata",
    functions: FUNCTIONS,
    since: Some("v0.4.0"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &audio_duration::AUDIO_DURATION,
    &audio_file_info::AUDIO_FILE_INFO,
    &beep::BEEP,
    &list_output_devices::LIST_OUTPUT_DEVICES,
    &play_file::PLAY_FILE,
    &play_file_async::PLAY_FILE_ASYNC,
    &set_master_volume::SET_MASTER_VOLUME,
    &set_output_device::SET_OUTPUT_DEVICE,
    &sound_get_volume::SOUND_GET_VOLUME,
    &sound_is_finished::SOUND_IS_FINISHED,
    &sound_is_paused::SOUND_IS_PAUSED,
    &sound_pause::SOUND_PAUSE,
    &sound_resume::SOUND_RESUME,
    &sound_seek::SOUND_SEEK,
    &sound_set_speed::SOUND_SET_SPEED,
    &sound_set_volume::SOUND_SET_VOLUME,
    &sound_stop::SOUND_STOP,
    &sound_wait::SOUND_WAIT,
];
