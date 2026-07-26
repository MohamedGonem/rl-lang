pub struct AudioHandle {
    pub sink: rodio::Player,
    #[allow(dead_code)]
    pub stream: rodio::MixerDeviceSink,
    pub base_volume: f32,
}

