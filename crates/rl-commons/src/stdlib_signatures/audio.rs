//! Typed signatures for `std::audio`.

use super::{handle, overloads, params, result};
use crate::{ModuleNames, StdFn};
use rl_ast::statements::{HandleKind, TypeAnnotation as T};

pub fn module() -> ModuleNames {
    ModuleNames::new("audio")
        .with_typed_function(play_file())
        .with_typed_function(play_file_async())
        .with_typed_function(beep())
        .with_typed_function(sound_pause())
        .with_typed_function(sound_resume())
        .with_typed_function(sound_stop())
        .with_typed_function(sound_is_paused())
        .with_typed_function(sound_set_volume())
        .with_typed_function(sound_get_volume())
        .with_typed_function(sound_set_speed())
        .with_typed_function(sound_seek())
        .with_typed_function(sound_is_finished())
        .with_typed_function(sound_wait())
        .with_typed_function(list_output_devices())
        .with_typed_function(set_output_device())
        .with_typed_function(set_master_volume())
        .with_typed_function(audio_duration())
        .with_typed_function(audio_file_info())
}

fn play_file() -> StdFn {
    StdFn::typed(
        "play_file",
        vec![(params(vec![T::String]), result(T::Null))],
    )
}

fn play_file_async() -> StdFn {
    StdFn::typed(
        "play_file_async",
        vec![(
            params(vec![T::String]),
            result(T::Handle(HandleKind::Audio)),
        )],
    )
}

fn beep() -> StdFn {
    StdFn::typed(
        "beep",
        vec![(params(vec![T::Float, T::Int]), result(T::Null))],
    )
}

fn sound_pause() -> StdFn {
    StdFn::typed(
        "sound_pause",
        overloads(vec![handle(HandleKind::Audio)], result(T::Null)),
    )
}

fn sound_resume() -> StdFn {
    StdFn::typed(
        "sound_resume",
        overloads(vec![handle(HandleKind::Audio)], result(T::Null)),
    )
}

fn sound_stop() -> StdFn {
    StdFn::typed(
        "sound_stop",
        overloads(vec![handle(HandleKind::Audio)], result(T::Null)),
    )
}

fn sound_is_paused() -> StdFn {
    StdFn::typed(
        "sound_is_paused",
        overloads(vec![handle(HandleKind::Audio)], result(T::Bool)),
    )
}

fn sound_set_volume() -> StdFn {
    StdFn::typed(
        "sound_set_volume",
        overloads(
            vec![handle(HandleKind::Audio), vec![T::Float]],
            result(T::Null),
        ),
    )
}

fn sound_get_volume() -> StdFn {
    StdFn::typed(
        "sound_get_volume",
        overloads(vec![handle(HandleKind::Audio)], result(T::Float)),
    )
}

fn sound_set_speed() -> StdFn {
    StdFn::typed(
        "sound_set_speed",
        overloads(
            vec![handle(HandleKind::Audio), vec![T::Float]],
            result(T::Null),
        ),
    )
}

fn sound_seek() -> StdFn {
    StdFn::typed(
        "sound_seek",
        overloads(
            vec![handle(HandleKind::Audio), vec![T::Int]],
            result(T::Null),
        ),
    )
}

fn sound_is_finished() -> StdFn {
    StdFn::typed(
        "sound_is_finished",
        overloads(vec![handle(HandleKind::Audio)], result(T::Bool)),
    )
}

fn sound_wait() -> StdFn {
    StdFn::typed(
        "sound_wait",
        overloads(vec![handle(HandleKind::Audio)], result(T::Null)),
    )
}

fn list_output_devices() -> StdFn {
    StdFn::typed(
        "list_output_devices",
        vec![(params(vec![]), result(T::Array(Box::new(T::String))))],
    )
}

fn set_output_device() -> StdFn {
    StdFn::typed(
        "set_output_device",
        vec![(params(vec![T::String]), result(T::Null))],
    )
}

fn set_master_volume() -> StdFn {
    StdFn::typed(
        "set_master_volume",
        vec![(params(vec![T::Float]), result(T::Null))],
    )
}

fn audio_duration() -> StdFn {
    StdFn::typed(
        "audio_duration",
        vec![(params(vec![T::String]), result(T::Int))],
    )
}

fn audio_file_info() -> StdFn {
    StdFn::typed(
        "audio_file_info",
        vec![(
            params(vec![T::String]),
            result(T::Tuple(std::rc::Rc::new(vec![
                T::Int,
                T::Int,
                T::Int,
                T::String,
            ]))),
        )],
    )
}
