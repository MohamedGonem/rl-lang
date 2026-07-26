use std::fs::File;

use crate::{
    Vm,
    stdlib::{
        audio::common::new_sink,
        common::extract_string,
        macros::{verr, vnl, vok, vs},
    },
    values::VmValue,
};

pub fn func(vm: &mut Vm, path: VmValue) -> VmValue {
    let path = match extract_string(path, "play_file") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("play_file: {}", e))),
    };

    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => return verr!(vs!(format!("play_file(\"{}\"): {}", path, e))),
    };

    let source = match rodio::Decoder::try_from(file) {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("play_file(\"{}\"): {}", path, e))),
    };

    let (sink, _stream) = match new_sink(vm) {
        Ok(pair) => pair,
        Err(e) => return verr!(vs!(format!("play_file: {}", e))),
    };

    sink.append(source);
    sink.sleep_until_end();
    vok!(vnl!())
}
