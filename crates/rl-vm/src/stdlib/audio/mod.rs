//! `std::audio` - audio playback built on `rodio`.

mod audio_duration;
mod audio_file_info;
mod beep;
mod common;

pub struct AudioHandle {
    pub sink: rodio::Player,
    #[allow(dead_code)]
    pub stream: rodio::MixerDeviceSink,
    pub base_volume: f32,
}

