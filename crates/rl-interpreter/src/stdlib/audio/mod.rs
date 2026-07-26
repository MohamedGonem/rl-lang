//! `std::audio` - audio playback built on `rodio`.

mod audio_duration;
mod common;

pub struct AudioHandle {
    pub sink: rodio::Player,
    pub stream: rodio::MixerDeviceSink,
    pub base_volume: f32,
}

