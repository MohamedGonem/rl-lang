//! `std::audio` - audio playback built on `rodio`/`cpal`/`symphonia`. Handle
//! module: playing sounds are stored behind integer handles in the runtime's
//! handle table, accessed via the `AudioStore` trait (implemented by each
//! runtime).
//!
//! Ported once from the former per-runtime `stdlib/audio/*.rs` copies (the VM
//! is canonical). The error strings and behaviour are reproduced verbatim,
//! including the double-prefixing quirk carried over from the original sources
//! (each function calls the extractor with its own name, then wraps the
//! resulting error under its own name again, e.g. `play_file: play_file: ...`).
//!
//! In addition to the handle table, this module reads/writes two pieces of
//! per-runtime context state - the selected output device
//! (`audio_output_device: Option<String>`) and the master volume
//! (`audio_master_volume: f32`) - through the `AudioStore` trait.

use cpal::traits::{DeviceTrait, HostTrait};
use rl_ast::statements::{HandleKind, TypeAnnotation};
use rl_std_core::Runtime;
use rl_std_macros::native_fn;
use rodio::source::{SineWave, Source};
use std::fs::File;
use std::time::Duration;

/// A single native audio-playback resource, stored behind an `int` handle.
/// Owns its `MixerDeviceSink` (rodio's renamed `OutputStream`, as of 0.22.2)
/// so playback keeps working as long as the handle is alive - dropping it
/// (via `sound_stop`) tears the device connection down.
/// (Moved here from the per-runtime copies so both share one definition.)
pub struct AudioHandle {
    pub sink: rodio::Player,
    #[allow(dead_code)]
    pub stream: rodio::MixerDeviceSink,
    pub base_volume: f32,
}

/// Per-runtime access to the `audio` handle table and the audio context state.
/// Implemented by `VmRuntime` / `EvalRuntime` in the runtime crates.
pub trait AudioStore: Runtime {
    fn audio_insert(cx: &mut Self::Cx, h: AudioHandle) -> u64;
    fn audio_get(cx: &Self::Cx, id: u64) -> Option<&AudioHandle>;
    fn audio_get_mut(cx: &mut Self::Cx, id: u64) -> Option<&mut AudioHandle>;
    fn audio_remove(cx: &mut Self::Cx, id: u64) -> Option<AudioHandle>;

    /// The output device selected via `set_output_device` (None = system
    /// default). Mutable so `set_output_device` can store the chosen name.
    fn audio_output_device(cx: &mut Self::Cx) -> &mut Option<String>;
    /// The master volume applied to every sink. Mutable so `set_master_volume`
    /// can update it.
    fn audio_master_volume(cx: &mut Self::Cx) -> &mut f32;

    /// Iterates every currently-live handle. Used by `set_master_volume` to
    /// rescale all playing sounds at once.
    fn audio_handles_values<'a>(cx: &'a Self::Cx) -> Box<dyn Iterator<Item = &'a AudioHandle> + 'a>;
}

/// Inserts a handle and returns its rl handle value.
fn insert_handle<R: AudioStore>(cx: &mut R::Cx, h: AudioHandle) -> R::Value {
    let id = R::audio_insert(cx, h);
    R::make_handle(HandleKind::Audio, id)
}

// ---- shared argument extraction (reproducing the old `extract_*` helpers) --

/// Reproduces the old `extract_string`: rejects non-strings with
/// `"<name>: expected string type, got <ty>"`.
fn extract_string<R: AudioStore>(v: &R::Value, name: &str) -> Result<String, String> {
    match R::as_str(v) {
        Some(s) => Ok(s.to_owned()),
        None => Err(format!(
            "{}: expected string type, got {}",
            name,
            R::type_name(v)
        )),
    }
}

/// Reproduces the old `extract_number`: accepts an `int` or `byte`, rejecting
/// everything else with `"<name>: expected int or byte type, got <ty>"`.
fn extract_number<R: AudioStore>(v: &R::Value, name: &str) -> Result<u64, String> {
    if let Some(i) = R::as_i64(v) {
        return Ok(i as u64);
    }
    if let Some(b) = R::as_u8(v) {
        return Ok(b as u64);
    }
    Err(format!(
        "{}: expected int or byte type, got {}",
        name,
        R::type_name(v)
    ))
}

/// Reproduces the old `audio::common::extract_float`: accepts an `int` or
/// `float`, rejecting everything else with
/// `"<name>: expected int or float, got <ty>"`.
fn extract_float<R: AudioStore>(v: &R::Value, name: &str) -> Result<f64, String> {
    if let Some(f) = R::as_f64(v) {
        return Ok(f);
    }
    if let Some(i) = R::as_i64(v) {
        return Ok(i as f64);
    }
    Err(format!(
        "{}: expected int or float, got {}",
        name,
        R::type_name(v)
    ))
}

/// Reproduces the old `extract_handle`: unwraps an `Audio` handle into its id,
/// with the same wrong-kind / not-a-handle messages.
fn extract_handle<R: AudioStore>(v: &R::Value, name: &str) -> Result<u64, String> {
    match R::as_handle(v, HandleKind::Audio) {
        Some(id) => Ok(id),
        None => match R::as_handle(v, HandleKind::C)
            .map(|_| HandleKind::C)
            .or_else(|| R::as_handle(v, HandleKind::Http).map(|_| HandleKind::Http))
            .or_else(|| R::as_handle(v, HandleKind::Net).map(|_| HandleKind::Net))
            .or_else(|| R::as_handle(v, HandleKind::Gui).map(|_| HandleKind::Gui))
        {
            Some(kind) => Err(format!(
                "{}: expected a {:?} handle, got a {:?} handle",
                name,
                HandleKind::Audio,
                kind
            )),
            None => Err(format!(
                "{}: expected a handle, got {}",
                name,
                R::type_name(v)
            )),
        },
    }
}

// ---- device / stream helpers (reproducing `audio::common`) -----------------

/// Opens an output stream on the device selected via `set_output_device`,
/// falling back to the system default when none was chosen.
fn open_stream<R: AudioStore>(cx: &mut R::Cx) -> Result<rodio::MixerDeviceSink, String> {
    let mut stream = match R::audio_output_device(cx) {
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

/// Builds a [`rodio::Player`] connected to a freshly opened output stream,
/// with volume/handle bookkeeping applied consistently across every
/// function that starts new playback (`play_file`, `play_file_async`, `beep`).
fn new_sink<R: AudioStore>(cx: &mut R::Cx) -> Result<(rodio::Player, rodio::MixerDeviceSink), String> {
    let stream = open_stream::<R>(cx)?;
    let sink = rodio::Player::connect_new(stream.mixer());
    sink.set_volume(*R::audio_master_volume(cx));
    Ok((sink, stream))
}

struct AudioProbeInfo {
    channels: usize,
    sample_rate: u32,
    duration_ms: i64,
    format_name: String,
}

/// Probes an audio file's container/codec metadata via `symphonia`, used by
/// `audio_duration` and `audio_file_info`. `rodio`'s `Decoder` doesn't expose
/// duration/channel-count reliably across formats, so this goes straight to
/// the underlying decoder library instead.
fn probe_file(path: &str) -> Result<AudioProbeInfo, String> {
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

// ---- playback --------------------------------------------------------------

#[native_fn(module = "audio", bound = "AudioStore", sig(string -> result[null]))]
pub fn play_file<R: AudioStore>(cx: &mut R::Cx, path: R::Value) -> R::Value {
    let path = match extract_string::<R>(&path, "play_file") {
        Ok(s) => s,
        Err(e) => return R::err(R::from_string(format!("play_file: {}", e))),
    };

    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => return R::err(R::from_string(format!("play_file(\"{}\"): {}", path, e))),
    };

    let source = match rodio::Decoder::try_from(file) {
        Ok(s) => s,
        Err(e) => return R::err(R::from_string(format!("play_file(\"{}\"): {}", path, e))),
    };

    let (sink, _stream) = match new_sink::<R>(cx) {
        Ok(pair) => pair,
        Err(e) => return R::err(R::from_string(format!("play_file: {}", e))),
    };

    sink.append(source);
    sink.sleep_until_end();
    R::ok(R::null())
}

#[native_fn(module = "audio", bound = "AudioStore", sig(string -> result[handle(Audio)]))]
pub fn play_file_async<R: AudioStore>(cx: &mut R::Cx, path: R::Value) -> R::Value {
    let path = match extract_string::<R>(&path, "play_file_async") {
        Ok(s) => s,
        Err(e) => return R::err(R::from_string(format!("play_file_async: {}", e))),
    };

    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => {
            return R::err(R::from_string(format!(
                "play_file_async(\"{}\"): {}",
                path, e
            )));
        }
    };

    let source = match rodio::Decoder::try_from(file) {
        Ok(s) => s,
        Err(e) => {
            return R::err(R::from_string(format!(
                "play_file_async(\"{}\"): {}",
                path, e
            )));
        }
    };

    let (sink, stream) = match new_sink::<R>(cx) {
        Ok(pair) => pair,
        Err(e) => return R::err(R::from_string(format!("play_file_async: {}", e))),
    };

    sink.append(source);

    R::ok(insert_handle::<R>(
        cx,
        AudioHandle {
            sink,
            stream,
            base_volume: 1.0,
        },
    ))
}

#[native_fn(module = "audio", bound = "AudioStore", sig(float, int -> result[null]))]
pub fn beep<R: AudioStore>(cx: &mut R::Cx, freq: R::Value, duration_ms: R::Value) -> R::Value {
    let freq = match extract_float::<R>(&freq, "beep") {
        Ok(f) => f,
        Err(e) => return R::err(R::from_string(format!("beep: {}", e))),
    };
    let duration_ms = match extract_number::<R>(&duration_ms, "beep") {
        Ok(n) => n,
        Err(e) => return R::err(R::from_string(format!("beep: {}", e))),
    };

    let (sink, _stream) = match new_sink::<R>(cx) {
        Ok(pair) => pair,
        Err(e) => return R::err(R::from_string(format!("beep: {}", e))),
    };

    let source = SineWave::new(freq as f32).take_duration(Duration::from_millis(duration_ms));
    sink.append(source);
    sink.sleep_until_end();
    R::ok(R::null())
}

// ---- per-sound control -----------------------------------------------------

#[native_fn(module = "audio", bound = "AudioStore", sig(handle(Audio) -> result[null]))]
pub fn sound_pause<R: AudioStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "sound_pause") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(format!("sound_pause: {}", e))),
    };

    match R::audio_get(cx, id) {
        Some(h) => {
            h.sink.pause();
            R::ok(R::null())
        }
        None => R::err(R::from_string(format!("sound_pause: unknown handle {}", id))),
    }
}

#[native_fn(module = "audio", bound = "AudioStore", sig(handle(Audio) -> result[null]))]
pub fn sound_resume<R: AudioStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "sound_resume") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(format!("sound_resume: {}", e))),
    };

    match R::audio_get(cx, id) {
        Some(h) => {
            h.sink.play();
            R::ok(R::null())
        }
        None => R::err(R::from_string(format!(
            "sound_resume: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "audio", bound = "AudioStore", sig(handle(Audio) -> result[null]))]
pub fn sound_stop<R: AudioStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "sound_stop") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(format!("sound_stop: {}", e))),
    };

    match R::audio_remove(cx, id) {
        Some(h) => {
            h.sink.stop();
            R::ok(R::null())
        }
        None => R::err(R::from_string(format!("sound_stop: unknown handle {}", id))),
    }
}

#[native_fn(module = "audio", bound = "AudioStore", sig(handle(Audio) -> result[bool]))]
pub fn sound_is_paused<R: AudioStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "sound_is_paused") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(format!("sound_is_paused: {}", e))),
    };

    match R::audio_get(cx, id) {
        Some(h) => R::ok(R::from_bool(h.sink.is_paused())),
        None => R::err(R::from_string(format!(
            "sound_is_paused: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "audio", bound = "AudioStore", sig(handle(Audio), float -> result[null]))]
pub fn sound_set_volume<R: AudioStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    volume: R::Value,
) -> R::Value {
    let id = match extract_handle::<R>(&handle, "sound_set_volume") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(format!("sound_set_volume: {}", e))),
    };
    let volume = match extract_float::<R>(&volume, "sound_set_volume") {
        Ok(v) => v as f32,
        Err(e) => return R::err(R::from_string(format!("sound_set_volume: {}", e))),
    };

    let master = *R::audio_master_volume(cx);
    match R::audio_get_mut(cx, id) {
        Some(h) => {
            h.base_volume = volume;
            h.sink.set_volume(volume * master);
            R::ok(R::null())
        }
        None => R::err(R::from_string(format!(
            "sound_set_volume: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "audio", bound = "AudioStore", sig(handle(Audio) -> result[float]))]
pub fn sound_get_volume<R: AudioStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "sound_get_volume") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(format!("sound_get_volume: {}", e))),
    };

    match R::audio_get(cx, id) {
        Some(h) => R::ok(R::from_f64(h.base_volume as f64)),
        None => R::err(R::from_string(format!(
            "sound_get_volume: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "audio", bound = "AudioStore", sig(handle(Audio), float -> result[null]))]
pub fn sound_set_speed<R: AudioStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    speed: R::Value,
) -> R::Value {
    let id = match extract_handle::<R>(&handle, "sound_set_speed") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(format!("sound_set_speed: {}", e))),
    };
    let speed = match extract_float::<R>(&speed, "sound_set_speed") {
        Ok(s) => s as f32,
        Err(e) => return R::err(R::from_string(format!("sound_set_speed: {}", e))),
    };

    match R::audio_get(cx, id) {
        Some(h) => {
            h.sink.set_speed(speed);
            R::ok(R::null())
        }
        None => R::err(R::from_string(format!(
            "sound_set_speed: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "audio", bound = "AudioStore", sig(handle(Audio), int -> result[null]))]
pub fn sound_seek<R: AudioStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    position_ms: R::Value,
) -> R::Value {
    let id = match extract_handle::<R>(&handle, "sound_seek") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(format!("sound_seek: {}", e))),
    };
    let position_ms = match extract_number::<R>(&position_ms, "sound_seek") {
        Ok(n) => n,
        Err(e) => return R::err(R::from_string(format!("sound_seek: {}", e))),
    };

    match R::audio_get(cx, id) {
        Some(h) => match h.sink.try_seek(Duration::from_millis(position_ms)) {
            Ok(()) => R::ok(R::null()),
            Err(e) => R::err(R::from_string(format!("sound_seek: {}", e))),
        },
        None => R::err(R::from_string(format!("sound_seek: unknown handle {}", id))),
    }
}

#[native_fn(module = "audio", bound = "AudioStore", sig(handle(Audio) -> result[bool]))]
pub fn sound_is_finished<R: AudioStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "sound_is_finished") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(format!("sound_is_finished: {}", e))),
    };

    match R::audio_get(cx, id) {
        Some(h) => R::ok(R::from_bool(h.sink.empty())),
        None => R::err(R::from_string(format!(
            "sound_is_finished: unknown handle {}",
            id
        ))),
    }
}

#[native_fn(module = "audio", bound = "AudioStore", sig(handle(Audio) -> result[null]))]
pub fn sound_wait<R: AudioStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "sound_wait") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(format!("sound_wait: {}", e))),
    };

    match R::audio_get(cx, id) {
        Some(h) => {
            h.sink.sleep_until_end();
            R::ok(R::null())
        }
        None => R::err(R::from_string(format!("sound_wait: unknown handle {}", id))),
    }
}

// ---- devices / master volume -----------------------------------------------

#[native_fn(module = "audio", bound = "AudioStore", sig(-> result[array[string]]))]
pub fn list_output_devices<R: AudioStore>(_cx: &mut R::Cx) -> R::Value {
    let host = cpal::default_host();
    let devices = match host.output_devices() {
        Ok(d) => d,
        Err(e) => return R::err(R::from_string(format!("list_output_devices: {}", e))),
    };

    let items: Vec<R::Value> = devices
        .filter_map(|d| d.description().ok())
        .map(|desc| R::from_string(desc.name().to_string()))
        .collect();

    R::ok(R::array(items, TypeAnnotation::String))
}

#[native_fn(module = "audio", bound = "AudioStore", sig(string -> result[null]))]
pub fn set_output_device<R: AudioStore>(cx: &mut R::Cx, name: R::Value) -> R::Value {
    let name = match extract_string::<R>(&name, "set_output_device") {
        Ok(s) => s,
        Err(e) => return R::err(R::from_string(format!("set_output_device: {}", e))),
    };

    let host = cpal::default_host();
    let devices = match host.output_devices() {
        Ok(d) => d,
        Err(e) => return R::err(R::from_string(format!("set_output_device: {}", e))),
    };

    let found = devices
        .filter_map(|d| d.description().ok())
        .any(|desc| desc.name() == name);

    if !found {
        return R::err(R::from_string(format!(
            "set_output_device: output device \"{}\" not found",
            name
        )));
    }

    *R::audio_output_device(cx) = Some(name);
    R::ok(R::null())
}

#[native_fn(module = "audio", bound = "AudioStore", sig(float -> result[null]))]
pub fn set_master_volume<R: AudioStore>(cx: &mut R::Cx, volume: R::Value) -> R::Value {
    let volume = match extract_float::<R>(&volume, "set_master_volume") {
        Ok(v) => v as f32,
        Err(e) => return R::err(R::from_string(format!("set_master_volume: {}", e))),
    };

    *R::audio_master_volume(cx) = volume;
    // Rescale every currently playing sound, not just future ones.
    for handle in R::audio_handles_values(cx) {
        handle.sink.set_volume(handle.base_volume * volume);
    }
    R::ok(R::null())
}

// ---- file metadata ---------------------------------------------------------

#[native_fn(module = "audio", bound = "AudioStore", sig(string -> result[int]))]
pub fn audio_duration<R: AudioStore>(_cx: &mut R::Cx, path: R::Value) -> R::Value {
    let path = match extract_string::<R>(&path, "audio_duration") {
        Ok(s) => s,
        Err(e) => return R::err(R::from_string(format!("audio_duration: {}", e))),
    };

    match probe_file(&path) {
        Ok(info) => R::ok(R::from_i64(info.duration_ms)),
        Err(e) => R::err(R::from_string(format!(
            "audio_duration(\"{}\"): {}",
            path, e
        ))),
    }
}

#[native_fn(module = "audio", bound = "AudioStore", sig(string -> result[tuple[int, int, int, string]]))]
pub fn audio_file_info<R: AudioStore>(_cx: &mut R::Cx, path: R::Value) -> R::Value {
    let path = match extract_string::<R>(&path, "audio_file_info") {
        Ok(s) => s,
        Err(e) => return R::err(R::from_string(format!("audio_file_info: {}", e))),
    };

    match probe_file(&path) {
        Ok(info) => R::ok(R::tuple(vec![
            R::from_i64(info.channels as i64),
            R::from_i64(info.sample_rate as i64),
            R::from_i64(info.duration_ms),
            R::from_string(info.format_name),
        ])),
        Err(e) => R::err(R::from_string(format!(
            "audio_file_info(\"{}\"): {}",
            path, e
        ))),
    }
}

rl_std_core::native_module!("audio";
    bound: AudioStore;
    funcs: [
        play_file, play_file_async, beep,
        sound_pause, sound_resume, sound_stop, sound_is_paused,
        sound_set_volume, sound_get_volume, sound_set_speed, sound_seek,
        sound_is_finished, sound_wait,
        list_output_devices, set_output_device, set_master_volume,
        audio_duration, audio_file_info,
    ],
);
