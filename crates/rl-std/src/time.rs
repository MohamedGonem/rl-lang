//! `std::time` - Unix timestamp functions and time formatting.
//!
//! All timestamps are Unix seconds as `i64`.
//! `format_time` uses a minimal strftime-like pattern (`%Y`, `%m`, `%d`, `%H`,
//! `%M`, `%S`). `time_parts` returns `[year, month, day, hour, minute, second]`
//! as an `arr[int]`. `time_add` and `time_diff` are trivial arithmetic helpers -
//! a proper time type is planned. Ported once from the former per-runtime
//! `stdlib/time/*.rs` copies.

use rl_std_macros::native_fn;
use std::sync::OnceLock;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

// ---- now ------------------------------------------------------------------

#[native_fn(module = "time")]
pub fn time_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

#[native_fn(module = "time")]
pub fn time_now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

// ---- arithmetic helpers ---------------------------------------------------

// yes... useless... for now
// should add timestamp or time type later
#[native_fn(module = "time")]
pub fn time_add(ts: i64, seconds: i64) -> i64 {
    ts + seconds
}

#[native_fn(module = "time")]
pub fn time_diff(a: i64, b: i64) -> i64 {
    a - b
}

// ---- Unix timestamp -> parts ----------------------------------------------

/// Decomposes a Unix timestamp into `(year, month, day, hour, minute, second)`
/// using the Gregorian calendar algorithm (proleptic calendar, UTC only, no
/// DST).
fn unix_to_parts(timestamp: i64) -> (i32, u32, u32, u32, u32, u32) {
    let total_seconds = timestamp;
    let time_of_day = total_seconds % 86400;
    let hour = time_of_day / 3600;
    let minute = (time_of_day % 3600) / 60;
    let second = time_of_day % 60;

    let days_since_epoch = total_seconds / 86400;
    let days_since_march0 = days_since_epoch + 719468;

    let century = days_since_march0.div_euclid(146097);
    let day_in_century = days_since_march0.rem_euclid(146097);
    let year_in_century = (day_in_century - day_in_century / 1460 + day_in_century / 36524
        - day_in_century / 146096)
        / 365;
    let day_in_year =
        day_in_century - (365 * year_in_century + year_in_century / 4 - year_in_century / 100);
    let month_index = (5 * day_in_year + 2) / 153;
    let day = day_in_year - (153 * month_index + 2) / 5 + 1;
    let month = if month_index < 10 {
        month_index + 3
    } else {
        month_index - 9
    };
    let year = year_in_century + century * 400 + if month <= 2 { 1 } else { 0 };

    (
        year as i32,
        month as u32,
        day as u32,
        hour as u32,
        minute as u32,
        second as u32,
    )
}

/// Simple string substitution on strftime-like tokens: `%Y` (4-digit year),
/// `%m` (month), `%d` (day), `%H` (hour), `%M` (minute), `%S` (second).
fn apply_pattern(
    pattern: &str,
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> String {
    pattern
        .replace("%Y", &format!("{:04}", year))
        .replace("%m", &format!("{:02}", month))
        .replace("%d", &format!("{:02}", day))
        .replace("%H", &format!("{:02}", hour))
        .replace("%M", &format!("{:02}", minute))
        .replace("%S", &format!("{:02}", second))
}

// ---- formatting (language `result[string]`) -------------------------------

#[native_fn(module = "time")]
pub fn format_time(timestamp: i64, pattern: String) -> Result<String, String> {
    if timestamp < 0 {
        return Err("timestamp is negative".to_string());
    }
    let (year, month, day, hour, minute, second) = unix_to_parts(timestamp);
    Ok(apply_pattern(
        &pattern, year, month, day, hour, minute, second,
    ))
}

#[native_fn(module = "time")]
pub fn format_date_str(timestamp: i64) -> Result<String, String> {
    if timestamp < 0 {
        return Err("timestamp is negative".to_string());
    }
    let (year, month, day, hour, minute, second) = unix_to_parts(timestamp);
    Ok(apply_pattern(
        "%Y-%m-%d", year, month, day, hour, minute, second,
    ))
}

#[native_fn(module = "time")]
pub fn format_time_str(timestamp: i64) -> Result<String, String> {
    if timestamp < 0 {
        return Err("timestamp is negative".to_string());
    }
    let (year, month, day, hour, minute, second) = unix_to_parts(timestamp);
    Ok(apply_pattern(
        "%H:%M:%S", year, month, day, hour, minute, second,
    ))
}

// ---- parts (language `result[array[int]]`) --------------------------------

#[native_fn(module = "time")]
pub fn time_parts(timestamp: i64) -> Result<Vec<i64>, String> {
    if timestamp < 0 {
        return Err("timestamp is negative".to_string());
    }
    let (year, month, day, hour, minute, second) = unix_to_parts(timestamp);
    Ok(vec![
        year as i64,
        month as i64,
        day as i64,
        hour as i64,
        minute as i64,
        second as i64,
    ])
}

// ---- monotonic clock ------------------------------------------------------

static MONO_START: OnceLock<Instant> = OnceLock::new();

#[native_fn(module = "time")]
pub fn monotonic_now() -> i64 {
    let start = MONO_START.get_or_init(Instant::now);
    start.elapsed().as_nanos() as i64
}

rl_std_core::native_module!("time";
    funcs: [
        time_now, time_now_ms,
        time_add, time_diff,
        format_time, format_date_str, format_time_str,
        time_parts,
        monotonic_now,
    ],
);
