//! `std::audio` - audio playback built on `rodio`.

mod audio_duration;
mod audio_file_info;
mod beep;
mod common;
mod play_file;
mod play_file_async;

pub struct AudioHandle {
    pub sink: rodio::Player,
    pub stream: rodio::MixerDeviceSink,
    pub base_volume: f32,
}

