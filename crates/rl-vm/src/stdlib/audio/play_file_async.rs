use std::fs::File;

use crate::{
    Vm,
    stdlib::{
        audio::{
            AudioHandle,
            common::{insert_handle, new_sink},
        },
        common::extract_string,
        macros::{verr, vok, vs},
    },
    values::VmValue,
};

pub fn func(vm: &mut Vm, path: VmValue) -> VmValue {
    let path = match extract_string(path, "play_file_async") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("play_file_async: {}", e))),
    };

    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => return verr!(vs!(format!("play_file_async(\"{}\"): {}", path, e))),
    };

    let source = match rodio::Decoder::try_from(file) {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("play_file_async(\"{}\"): {}", path, e))),
    };

    let (sink, stream) = match new_sink(vm) {
        Ok(pair) => pair,
        Err(e) => return verr!(vs!(format!("play_file_async: {}", e))),
    };

    sink.append(source);

    let handle = insert_handle(
        vm,
        AudioHandle {
            sink,
            stream,
            base_volume: 1.0,
        },
    );
    vok!(handle)
}
