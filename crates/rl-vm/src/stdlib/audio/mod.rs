//! `std::audio` - audio playback built on `rodio`.

use crate::native::Module;

mod audio_duration;
mod audio_file_info;
mod beep;
mod common;
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

/// A single native audio-playback resource, stored behind an `int` handle.
/// Owns its `MixerDeviceSink` (rodio's renamed `OutputStream`, as of 0.22.2)
/// so playback keeps working as long as the handle is alive - dropping it
/// (via `sound_stop`) tears the device connection down.
pub struct AudioHandle {
    pub sink: rodio::Player,
    #[allow(dead_code)]
    pub stream: rodio::MixerDeviceSink,
    pub base_volume: f32,
}

pub fn module() -> Module {
    Module::new("audio")
        .with_function("play_file", play_file::func)
        .with_function("play_file_async", play_file_async::func)
        .with_function("beep", beep::func)
        .with_function("sound_pause", sound_pause::func)
        .with_function("sound_resume", sound_resume::func)
        .with_function("sound_stop", sound_stop::func)
        .with_function("sound_is_paused", sound_is_paused::func)
        .with_function("sound_set_volume", sound_set_volume::func)
        .with_function("sound_get_volume", sound_get_volume::func)
        .with_function("sound_set_speed", sound_set_speed::func)
        .with_function("sound_seek", sound_seek::func)
        .with_function("sound_is_finished", sound_is_finished::func)
        .with_function("sound_wait", sound_wait::func)
        .with_function("list_output_devices", list_output_devices::func)
        .with_function("set_output_device", set_output_device::func)
        .with_function("set_master_volume", set_master_volume::func)
        .with_function("audio_duration", audio_duration::func)
        .with_function("audio_file_info", audio_file_info::func)
}
