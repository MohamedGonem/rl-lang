use crate::Vm;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn now(_: &mut Vm) -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

pub fn now_ms(_: &mut Vm) -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}
