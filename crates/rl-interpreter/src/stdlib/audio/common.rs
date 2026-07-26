use cpal::traits::{DeviceTrait, HostTrait};

use crate::{evaluator::Evaluator, stdlib::audio::AudioHandle, values::Value};

/// Registers a new handle and returns its id.
pub fn insert_handle(eval: &mut Evaluator, handle: AudioHandle) -> i64 {
    let id = eval.audio_next_handle;
    eval.audio_next_handle += 1;
    eval.audio_handles.insert(id, handle);
    id
}

/// Extracts an `f64` from an `int` or `float` [`Value`].
pub fn extract_float(value: Value, name: &str) -> Result<f64, String> {
    match value {
        Value::Float(f) => Ok(f),
        Value::Integer(i) => Ok(i as f64),
        other => Err(format!(
            "{}: expected int or float, got {}",
            name,
            other.type_name()
        )),
    }
}

/// Opens an output stream on the device selected via `set_output_device`,
/// falling back to the system default when none was chosen.
pub fn open_stream(eval: &Evaluator) -> Result<rodio::MixerDeviceSink, String> {
    let mut stream = match &eval.audio_output_device {
        Some(name) => {
            let device = find_device(name)?;
            rodio::DeviceSinkBuilder::from_device(device)
                .map_err(|e| e.to_string())?
                .open_sink_or_fallback()
                .map_err(|e| e.to_string())
        }
        None => rodio::DeviceSinkBuilder::open_default_sink().map_err(|e| e.to_string()),
    }?;
    stream.log_on_drop(false);
    Ok(stream)
}

fn find_device(name: &str) -> Result<cpal::Device, String> {
    let host = cpal::default_host();
    host.output_devices()
        .map_err(|e| e.to_string())?
        .find(|d| {
            d.description()
                .map(|desc| desc.name() == name)
                .unwrap_or(false)
        })
        .ok_or_else(|| format!("output device \"{}\" not found", name))
}

pub struct AudioProbeInfo {
    pub channels: usize,
    pub sample_rate: u32,
    pub duration_ms: i64,
    pub format_name: String,
}

/// Probes an audio file's container/codec metadata via `symphonia`, used by
/// `audio_duration` and `audio_file_info`. `rodio`'s `Decoder` doesn't expose
/// duration/channel-count reliably across formats, so this goes straight to
/// the underlying decoder library instead.
pub fn probe_file(path: &str) -> Result<AudioProbeInfo, String> {
    use symphonia::core::{
        formats::FormatOptions, io::MediaSourceStream, meta::MetadataOptions, probe::Hint,
    };

    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    if let Some(ext) = std::path::Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
    {
        hint.with_extension(ext);
    }

    let probed = symphonia::default::get_probe()
        .format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .map_err(|e| e.to_string())?;

    let format = probed.format;
    let track = format
        .default_track()
        .ok_or_else(|| "no default audio track found".to_string())?;

    let params = &track.codec_params;
    let channels = params.channels.map(|c| c.count()).unwrap_or(0);
    let sample_rate = params.sample_rate.unwrap_or(0);

    let duration_ms = match (params.n_frames, params.sample_rate) {
        (Some(frames), Some(rate)) if rate > 0 => ((frames as f64 / rate as f64) * 1000.0) as i64,
        _ => 0,
    };

    let format_name = symphonia::default::get_codecs()
        .get_codec(params.codec)
        .map(|desc| desc.short_name.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    Ok(AudioProbeInfo {
        channels,
        sample_rate,
        duration_ms,
        format_name,
    })
}

/// Builds a [`rodio::Player`] connected to a freshly opened output stream,
/// with volume/handle bookkeeping applied consistently across every
/// function that starts new playback (`play_file`, `play_file_async`, `beep`).
pub fn new_sink(eval: &Evaluator) -> Result<(rodio::Player, rodio::MixerDeviceSink), String> {
    let stream = open_stream(eval)?;
    let sink = rodio::Player::connect_new(stream.mixer());
    sink.set_volume(eval.audio_master_volume);
    Ok((sink, stream))
}
