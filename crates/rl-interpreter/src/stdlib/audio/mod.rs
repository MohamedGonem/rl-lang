//! `std::audio` - audio playback built on `rodio`.

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

pub struct AudioHandle {
    pub sink: rodio::Player,
    pub stream: rodio::MixerDeviceSink,
    pub base_volume: f32,
}

