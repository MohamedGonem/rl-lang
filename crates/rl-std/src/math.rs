//! `std::math` - mathematical functions and the `std::math::consts` submodule.
//!
//! Most functions accept both `int` and `float`; mixed types are handled
//! per-function. The value-polymorphic functions (`abs`, `ceil`, `floor`,
//! `round`, `clamp`, `max`, `min`, `mod`, `log`, `sqrt`, `log2`, `log10`,
//! `pow`) inspect the raw runtime value and build a language `result[...]`
//! themselves. The trig/exp-family and integer helpers take concrete
//! `f64`/`i64` arguments and return `f64`/`i64`/`bool` directly.
//!
//! Ported once from the former per-runtime `stdlib/math/*.rs` copies.

use rl_std_core::Runtime;
use rl_std_macros::native_fn;

// ---- value-polymorphic: same-type unary result ----------------------------

#[native_fn(module = "math",
    sig(int -> result[int]), sig(float -> result[float]),
    sig(byte -> result[byte]), sig(sbyte -> result[sbyte]),
    sig(bbyte -> result[bbyte]), sig(bsbyte -> result[bsbyte]),
    sig(sint -> result[sint]), sig(suint -> result[suint]),
    sig(uint -> result[uint]), sig(sfloat -> result[sfloat])
)]
pub fn abs<R: Runtime>(_cx: &mut R::Cx, a: R::Value) -> R::Value {
    if let Some(i) = R::as_i64(&a) {
        R::ok(R::from_i64(i.abs()))
    } else if let Some(f) = R::as_f64(&a) {
        R::ok(R::from_f64(f.abs()))
    } else {
        R::err(R::from_string(format!(
            "abs() expects a number, got {}",
            R::type_name(&a)
        )))
    }
}

#[native_fn(module = "math",
    sig(int -> result[int]), sig(float -> result[float]),
    sig(byte -> result[byte]), sig(sbyte -> result[sbyte]),
    sig(bbyte -> result[bbyte]), sig(bsbyte -> result[bsbyte]),
    sig(sint -> result[sint]), sig(suint -> result[suint]),
    sig(uint -> result[uint]), sig(sfloat -> result[sfloat])
)]
pub fn ceil<R: Runtime>(_cx: &mut R::Cx, a: R::Value) -> R::Value {
    if let Some(i) = R::as_i64(&a) {
        R::ok(R::from_i64(i))
    } else if let Some(f) = R::as_f64(&a) {
        R::ok(R::from_f64(f.ceil()))
    } else {
        R::err(R::from_string(format!(
            "ceil() expects a number, got {}",
            R::type_name(&a)
        )))
    }
}

#[native_fn(module = "math",
    sig(int -> result[int]), sig(float -> result[float]),
    sig(byte -> result[byte]), sig(sbyte -> result[sbyte]),
    sig(bbyte -> result[bbyte]), sig(bsbyte -> result[bsbyte]),
    sig(sint -> result[sint]), sig(suint -> result[suint]),
    sig(uint -> result[uint]), sig(sfloat -> result[sfloat])
)]
pub fn floor<R: Runtime>(_cx: &mut R::Cx, a: R::Value) -> R::Value {
    if let Some(i) = R::as_i64(&a) {
        R::ok(R::from_i64(i))
    } else if let Some(f) = R::as_f64(&a) {
        R::ok(R::from_f64(f.floor()))
    } else {
        R::err(R::from_string(format!(
            "floor expects a number, got {}",
            R::type_name(&a)
        )))
    }
}

#[native_fn(module = "math",
    sig(int -> result[int]), sig(float -> result[float]),
    sig(byte -> result[byte]), sig(sbyte -> result[sbyte]),
    sig(bbyte -> result[bbyte]), sig(bsbyte -> result[bsbyte]),
    sig(sint -> result[sint]), sig(suint -> result[suint]),
    sig(uint -> result[uint]), sig(sfloat -> result[sfloat])
)]
pub fn round<R: Runtime>(_cx: &mut R::Cx, a: R::Value) -> R::Value {
    if let Some(i) = R::as_i64(&a) {
        R::ok(R::from_i64(i))
    } else if let Some(f) = R::as_f64(&a) {
        R::ok(R::from_f64(f.round()))
    } else {
        R::err(R::from_string(format!(
            "round expects a number, got {}",
            R::type_name(&a)
        )))
    }
}

// ---- value-polymorphic: clamp / max / min ---------------------------------

#[native_fn(
    module = "math",
    sig(int, int, int -> result[int]),
    sig(float, float, float -> result[float]),
    sig(byte, byte, byte -> result[byte]),
    sig(sbyte, sbyte, sbyte -> result[sbyte]),
    sig(bbyte, bbyte, bbyte -> result[bbyte]),
    sig(bsbyte, bsbyte, bsbyte -> result[bsbyte]),
    sig(sint, sint, sint -> result[sint]),
    sig(suint, suint, suint -> result[suint]),
    sig(uint, uint, uint -> result[uint]),
    sig(sfloat, sfloat, sfloat -> result[sfloat])
)]
pub fn clamp<R: Runtime>(
    _cx: &mut R::Cx,
    value: R::Value,
    min: R::Value,
    max: R::Value,
) -> R::Value {
    if let (Some(value), Some(low), Some(high)) =
        (R::as_i64(&value), R::as_i64(&min), R::as_i64(&max))
    {
        R::ok(R::from_i64(value.clamp(low, high)))
    } else if let (Some(value), Some(low), Some(high)) =
        (R::as_f64(&value), R::as_f64(&min), R::as_f64(&max))
    {
        R::ok(R::from_f64(value.clamp(low, high)))
    } else {
        R::err(R::from_string(format!(
            "clamp expects a number, got ({}, {}, {})",
            R::type_name(&value),
            R::type_name(&min),
            R::type_name(&max)
        )))
    }
}

#[native_fn(
    module = "math",
    sig(int, int -> result[int]),
    sig(float, float -> result[float]),
    sig(byte, byte -> result[byte]),
    sig(sbyte, sbyte -> result[sbyte]),
    sig(bbyte, bbyte -> result[bbyte]),
    sig(bsbyte, bsbyte -> result[bsbyte]),
    sig(sint, sint -> result[sint]),
    sig(suint, suint -> result[suint]),
    sig(uint, uint -> result[uint]),
    sig(sfloat, sfloat -> result[sfloat])
)]
pub fn max<R: Runtime>(_cx: &mut R::Cx, a: R::Value, b: R::Value) -> R::Value {
    if let (Some(a), Some(b)) = (R::as_i64(&a), R::as_i64(&b)) {
        R::ok(R::from_i64(a.max(b)))
    } else if let (Some(a), Some(b)) = (R::as_f64(&a), R::as_f64(&b)) {
        R::ok(R::from_f64(a.max(b)))
    } else {
        R::err(R::from_string(format!(
            "max expects a number, got ({}, {})",
            R::type_name(&a),
            R::type_name(&b)
        )))
    }
}

#[native_fn(
    module = "math",
    sig(int, int -> result[int]),
    sig(float, float -> result[float]),
    sig(byte, byte -> result[byte]),
    sig(sbyte, sbyte -> result[sbyte]),
    sig(bbyte, bbyte -> result[bbyte]),
    sig(bsbyte, bsbyte -> result[bsbyte]),
    sig(sint, sint -> result[sint]),
    sig(suint, suint -> result[suint]),
    sig(uint, uint -> result[uint]),
    sig(sfloat, sfloat -> result[sfloat])
)]
pub fn min<R: Runtime>(_cx: &mut R::Cx, a: R::Value, b: R::Value) -> R::Value {
    if let (Some(a), Some(b)) = (R::as_i64(&a), R::as_i64(&b)) {
        R::ok(R::from_i64(a.min(b)))
    } else if let (Some(a), Some(b)) = (R::as_f64(&a), R::as_f64(&b)) {
        R::ok(R::from_f64(a.min(b)))
    } else {
        R::err(R::from_string(format!(
            "min expects a number, got ({}, {})",
            R::type_name(&a),
            R::type_name(&b)
        )))
    }
}

// ---- value-polymorphic: mod / pow (mixed int/float overloads) -------------

// `mod` is a Rust keyword, so the fn is named `modulo`; `name = "mod"` restores
// the rl-level name.
#[native_fn(
    module = "math",
    name = "mod",
    sig(int, int -> result[int]),
    sig(int, float -> result[float]),
    sig(float, float -> result[float]),
    sig(float, int -> result[float]),
    sig(byte, byte -> result[byte]),
    sig(sbyte, sbyte -> result[sbyte]),
    sig(bbyte, bbyte -> result[bbyte]),
    sig(bsbyte, bsbyte -> result[bsbyte]),
    sig(sint, sint -> result[sint]),
    sig(suint, suint -> result[suint]),
    sig(uint, uint -> result[uint]),
    sig(sfloat, sfloat -> result[sfloat])
)]
pub fn modulo<R: Runtime>(_cx: &mut R::Cx, a: R::Value, b: R::Value) -> R::Value {
    if let (Some(a), Some(b)) = (R::as_i64(&a), R::as_i64(&b)) {
        R::ok(R::from_i64(a % b))
    } else if let (Some(a), Some(b)) = (R::as_f64(&a), R::as_f64(&b)) {
        R::ok(R::from_f64(a % b))
    } else {
        R::err(R::from_string(format!(
            "mod expects a number, got ({}, {})",
            R::type_name(&a),
            R::type_name(&b)
        )))
    }
}

#[native_fn(
    module = "math",
    sig(int, int -> result[int]),
    sig(int, float -> result[float]),
    sig(float, float -> result[float]),
    sig(float, int -> result[float]),
    sig(byte, byte -> result[byte]),
    sig(sbyte, sbyte -> result[sbyte]),
    sig(bbyte, bbyte -> result[bbyte]),
    sig(bsbyte, bsbyte -> result[bsbyte]),
    sig(sint, sint -> result[sint]),
    sig(suint, suint -> result[suint]),
    sig(uint, uint -> result[uint]),
    sig(sfloat, sfloat -> result[sfloat])
)]
pub fn pow<R: Runtime>(_cx: &mut R::Cx, base: R::Value, exponent: R::Value) -> R::Value {
    match (
        R::as_i64(&base),
        R::as_f64(&base),
        R::as_i64(&exponent),
        R::as_f64(&exponent),
    ) {
        // (Int, Int) -> Int
        (Some(a), _, Some(b), _) => {
            let b = b as u32;
            R::ok(R::from_i64(a.pow(b)))
        }
        // (Int, Float) -> Float
        (Some(a), _, None, Some(b)) => R::ok(R::from_f64((a as f64).powf(b))),
        // (Float, Int) -> Float
        (None, Some(a), Some(b), _) => R::ok(R::from_f64(a.powi(b as i32))),
        // (Float, Float) -> Float
        (None, Some(a), None, Some(b)) => R::ok(R::from_f64(a.powf(b))),
        _ => R::err(R::from_string("pow expects numeric arguments".to_string())),
    }
}

// ---- value-polymorphic: log (always float) --------------------------------

#[native_fn(
    module = "math",
    sig(int, int -> result[float]),
    sig(int, float -> result[float]),
    sig(float, float -> result[float]),
    sig(float, int -> result[float]),
    sig(byte, byte -> result[float]),
    sig(sbyte, sbyte -> result[float]),
    sig(bbyte, bbyte -> result[float]),
    sig(bsbyte, bsbyte -> result[float]),
    sig(sint, sint -> result[float]),
    sig(suint, suint -> result[float]),
    sig(uint, uint -> result[float]),
    sig(sfloat, sfloat -> result[float])
)]
pub fn log<R: Runtime>(_cx: &mut R::Cx, a: R::Value, base: R::Value) -> R::Value {
    match (
        R::as_i64(&a),
        R::as_f64(&a),
        R::as_i64(&base),
        R::as_f64(&base),
    ) {
        (Some(i), _, Some(base), _) => R::ok(R::from_f64((i as f64).log(base as f64))),
        (Some(i), _, None, Some(base)) => R::ok(R::from_f64((i as f64).log(base))),
        (None, Some(f), Some(base), _) => R::ok(R::from_f64(f.log(base as f64))),
        (None, Some(f), None, Some(base)) => R::ok(R::from_f64(f.log(base))),
        _ => R::err(R::from_string(format!(
            "log expects a number, got ({}, {})",
            R::type_name(&a),
            R::type_name(&base)
        ))),
    }
}

// ---- value-polymorphic: float-result unary (int coerced to f64) -----------

#[native_fn(module = "math",
    sig(int -> result[float]), sig(float -> result[float]),
    sig(byte -> result[float]), sig(sbyte -> result[float]),
    sig(bbyte -> result[float]), sig(bsbyte -> result[float]),
    sig(sint -> result[float]), sig(suint -> result[float]),
    sig(uint -> result[float]), sig(sfloat -> result[float])
)]
pub fn sqrt<R: Runtime>(_cx: &mut R::Cx, a: R::Value) -> R::Value {
    if let Some(i) = R::as_i64(&a) {
        R::ok(R::from_f64((i as f64).sqrt()))
    } else if let Some(f) = R::as_f64(&a) {
        R::ok(R::from_f64(f.sqrt()))
    } else {
        R::err(R::from_string(format!(
            "sqrt expects a number, got {}",
            R::type_name(&a)
        )))
    }
}

#[native_fn(module = "math",
    sig(int -> result[float]), sig(float -> result[float]),
    sig(byte -> result[float]), sig(sbyte -> result[float]),
    sig(bbyte -> result[float]), sig(bsbyte -> result[float]),
    sig(sint -> result[float]), sig(suint -> result[float]),
    sig(uint -> result[float]), sig(sfloat -> result[float])
)]
pub fn log2<R: Runtime>(_cx: &mut R::Cx, a: R::Value) -> R::Value {
    if let Some(i) = R::as_i64(&a) {
        R::ok(R::from_f64((i as f64).log2()))
    } else if let Some(f) = R::as_f64(&a) {
        R::ok(R::from_f64(f.log2()))
    } else {
        R::err(R::from_string(format!(
            "log2 expects a number, got {}",
            R::type_name(&a)
        )))
    }
}

#[native_fn(module = "math",
    sig(int -> result[float]), sig(float -> result[float]),
    sig(byte -> result[float]), sig(sbyte -> result[float]),
    sig(bbyte -> result[float]), sig(bsbyte -> result[float]),
    sig(sint -> result[float]), sig(suint -> result[float]),
    sig(uint -> result[float]), sig(sfloat -> result[float])
)]
pub fn log10<R: Runtime>(_cx: &mut R::Cx, a: R::Value) -> R::Value {
    if let Some(i) = R::as_i64(&a) {
        R::ok(R::from_f64((i as f64).log10()))
    } else if let Some(f) = R::as_f64(&a) {
        R::ok(R::from_f64(f.log10()))
    } else {
        R::err(R::from_string(format!(
            "log10 expects a number, got {}",
            R::type_name(&a)
        )))
    }
}

// ---- trig / exp family (any numeric -> float) ------------------------------

#[native_fn(module = "math",
    sig(int -> result[float]), sig(float -> result[float]),
    sig(byte -> result[float]), sig(sbyte -> result[float]),
    sig(bbyte -> result[float]), sig(bsbyte -> result[float]),
    sig(sint -> result[float]), sig(suint -> result[float]),
    sig(uint -> result[float]), sig(sfloat -> result[float])
)]
pub fn sin<R: Runtime>(_cx: &mut R::Cx, x: R::Value) -> R::Value {
    if let Some(f) = R::as_f64(&x) {
        R::ok(R::from_f64(f.sin()))
    } else {
        R::err(R::from_string(format!("sin expects a number, got {}", R::type_name(&x))))
    }
}

#[native_fn(module = "math",
    sig(int -> result[float]), sig(float -> result[float]),
    sig(byte -> result[float]), sig(sbyte -> result[float]),
    sig(bbyte -> result[float]), sig(bsbyte -> result[float]),
    sig(sint -> result[float]), sig(suint -> result[float]),
    sig(uint -> result[float]), sig(sfloat -> result[float])
)]
pub fn cos<R: Runtime>(_cx: &mut R::Cx, x: R::Value) -> R::Value {
    if let Some(f) = R::as_f64(&x) {
        R::ok(R::from_f64(f.cos()))
    } else {
        R::err(R::from_string(format!("cos expects a number, got {}", R::type_name(&x))))
    }
}

#[native_fn(module = "math",
    sig(int -> result[float]), sig(float -> result[float]),
    sig(byte -> result[float]), sig(sbyte -> result[float]),
    sig(bbyte -> result[float]), sig(bsbyte -> result[float]),
    sig(sint -> result[float]), sig(suint -> result[float]),
    sig(uint -> result[float]), sig(sfloat -> result[float])
)]
pub fn tan<R: Runtime>(_cx: &mut R::Cx, x: R::Value) -> R::Value {
    if let Some(f) = R::as_f64(&x) {
        R::ok(R::from_f64(f.tan()))
    } else {
        R::err(R::from_string(format!("tan expects a number, got {}", R::type_name(&x))))
    }
}

#[native_fn(module = "math",
    sig(int -> result[float]), sig(float -> result[float]),
    sig(byte -> result[float]), sig(sbyte -> result[float]),
    sig(bbyte -> result[float]), sig(bsbyte -> result[float]),
    sig(sint -> result[float]), sig(suint -> result[float]),
    sig(uint -> result[float]), sig(sfloat -> result[float])
)]
pub fn atan<R: Runtime>(_cx: &mut R::Cx, x: R::Value) -> R::Value {
    if let Some(f) = R::as_f64(&x) {
        R::ok(R::from_f64(f.atan()))
    } else {
        R::err(R::from_string(format!("atan expects a number, got {}", R::type_name(&x))))
    }
}

#[native_fn(module = "math",
    sig(int -> result[float]), sig(float -> result[float]),
    sig(byte -> result[float]), sig(sbyte -> result[float]),
    sig(bbyte -> result[float]), sig(bsbyte -> result[float]),
    sig(sint -> result[float]), sig(suint -> result[float]),
    sig(uint -> result[float]), sig(sfloat -> result[float])
)]
pub fn acos<R: Runtime>(_cx: &mut R::Cx, x: R::Value) -> R::Value {
    if let Some(f) = R::as_f64(&x) {
        R::ok(R::from_f64(f.acos()))
    } else {
        R::err(R::from_string(format!("acos expects a number, got {}", R::type_name(&x))))
    }
}

#[native_fn(module = "math",
    sig(int -> result[float]), sig(float -> result[float]),
    sig(byte -> result[float]), sig(sbyte -> result[float]),
    sig(bbyte -> result[float]), sig(bsbyte -> result[float]),
    sig(sint -> result[float]), sig(suint -> result[float]),
    sig(uint -> result[float]), sig(sfloat -> result[float])
)]
pub fn asin<R: Runtime>(_cx: &mut R::Cx, x: R::Value) -> R::Value {
    if let Some(f) = R::as_f64(&x) {
        R::ok(R::from_f64(f.asin()))
    } else {
        R::err(R::from_string(format!("asin expects a number, got {}", R::type_name(&x))))
    }
}

#[native_fn(module = "math",
    sig(int -> result[float]), sig(float -> result[float]),
    sig(byte -> result[float]), sig(sbyte -> result[float]),
    sig(bbyte -> result[float]), sig(bsbyte -> result[float]),
    sig(sint -> result[float]), sig(suint -> result[float]),
    sig(uint -> result[float]), sig(sfloat -> result[float])
)]
pub fn degrees<R: Runtime>(_cx: &mut R::Cx, x: R::Value) -> R::Value {
    if let Some(f) = R::as_f64(&x) {
        R::ok(R::from_f64(f.to_degrees()))
    } else {
        R::err(R::from_string(format!("degrees expects a number, got {}", R::type_name(&x))))
    }
}

#[native_fn(module = "math",
    sig(int -> result[float]), sig(float -> result[float]),
    sig(byte -> result[float]), sig(sbyte -> result[float]),
    sig(bbyte -> result[float]), sig(bsbyte -> result[float]),
    sig(sint -> result[float]), sig(suint -> result[float]),
    sig(uint -> result[float]), sig(sfloat -> result[float])
)]
pub fn radians<R: Runtime>(_cx: &mut R::Cx, x: R::Value) -> R::Value {
    if let Some(f) = R::as_f64(&x) {
        R::ok(R::from_f64(f.to_radians()))
    } else {
        R::err(R::from_string(format!("radians expects a number, got {}", R::type_name(&x))))
    }
}

#[native_fn(module = "math",
    sig(int -> result[float]), sig(float -> result[float]),
    sig(byte -> result[float]), sig(sbyte -> result[float]),
    sig(bbyte -> result[float]), sig(bsbyte -> result[float]),
    sig(sint -> result[float]), sig(suint -> result[float]),
    sig(uint -> result[float]), sig(sfloat -> result[float])
)]
pub fn exp<R: Runtime>(_cx: &mut R::Cx, x: R::Value) -> R::Value {
    if let Some(f) = R::as_f64(&x) {
        R::ok(R::from_f64(f.exp()))
    } else {
        R::err(R::from_string(format!("exp expects a number, got {}", R::type_name(&x))))
    }
}

#[native_fn(module = "math",
    sig(int -> result[float]), sig(float -> result[float]),
    sig(byte -> result[float]), sig(sbyte -> result[float]),
    sig(bbyte -> result[float]), sig(bsbyte -> result[float]),
    sig(sint -> result[float]), sig(suint -> result[float]),
    sig(uint -> result[float]), sig(sfloat -> result[float])
)]
pub fn sign<R: Runtime>(_cx: &mut R::Cx, x: R::Value) -> R::Value {
    if let Some(f) = R::as_f64(&x) {
        R::ok(R::from_f64(if f > 0.0 { 1.0 } else if f < 0.0 { -1.0 } else { 0.0 }))
    } else {
        R::err(R::from_string(format!("sign expects a number, got {}", R::type_name(&x))))
    }
}

#[native_fn(module = "math",
    sig(int, int -> result[float]), sig(float, float -> result[float]),
    sig(byte, byte -> result[float]), sig(sbyte, sbyte -> result[float]),
    sig(bbyte, bbyte -> result[float]), sig(bsbyte, bsbyte -> result[float]),
    sig(sint, sint -> result[float]), sig(suint, suint -> result[float]),
    sig(uint, uint -> result[float]), sig(sfloat, sfloat -> result[float])
)]
pub fn atan2<R: Runtime>(_cx: &mut R::Cx, x: R::Value, y: R::Value) -> R::Value {
    match (R::as_f64(&x), R::as_f64(&y)) {
        (Some(x), Some(y)) => R::ok(R::from_f64(y.atan2(x))),
        _ => R::err(R::from_string(format!(
            "atan2 expects numbers, got ({}, {})",
            R::type_name(&x), R::type_name(&y)
        ))),
    }
}

#[native_fn(module = "math",
    sig(int, int -> result[float]), sig(float, float -> result[float]),
    sig(byte, byte -> result[float]), sig(sbyte, sbyte -> result[float]),
    sig(bbyte, bbyte -> result[float]), sig(bsbyte, bsbyte -> result[float]),
    sig(sint, sint -> result[float]), sig(suint, suint -> result[float]),
    sig(uint, uint -> result[float]), sig(sfloat, sfloat -> result[float])
)]
pub fn hypot<R: Runtime>(_cx: &mut R::Cx, x: R::Value, y: R::Value) -> R::Value {
    match (R::as_f64(&x), R::as_f64(&y)) {
        (Some(x), Some(y)) => R::ok(R::from_f64(x.hypot(y))),
        _ => R::err(R::from_string(format!(
            "hypot expects numbers, got ({}, {})",
            R::type_name(&x), R::type_name(&y)
        ))),
    }
}

#[native_fn(module = "math",
    sig(int, int, int -> result[float]), sig(float, float, float -> result[float]),
    sig(byte, byte, byte -> result[float]), sig(sbyte, sbyte, sbyte -> result[float]),
    sig(bbyte, bbyte, bbyte -> result[float]), sig(bsbyte, bsbyte, bsbyte -> result[float]),
    sig(sint, sint, sint -> result[float]), sig(suint, suint, suint -> result[float]),
    sig(uint, uint, uint -> result[float]), sig(sfloat, sfloat, sfloat -> result[float])
)]
pub fn lerp<R: Runtime>(_cx: &mut R::Cx, x: R::Value, y: R::Value, t: R::Value) -> R::Value {
    match (R::as_f64(&x), R::as_f64(&y), R::as_f64(&t)) {
        (Some(x), Some(y), Some(t)) => R::ok(R::from_f64(x + (y - x) * t)),
        _ => R::err(R::from_string(format!(
            "lerp expects numbers, got ({}, {}, {})",
            R::type_name(&x), R::type_name(&y), R::type_name(&t)
        ))),
    }
}

#[native_fn(module = "math",
    sig(int, int, int, int, int -> result[float]),
    sig(float, float, float, float, float -> result[float])
)]
pub fn map_range<R: Runtime>(
    _cx: &mut R::Cx,
    value: R::Value,
    in_min: R::Value,
    out_min: R::Value,
    in_max: R::Value,
    out_max: R::Value,
) -> R::Value {
    match (
        R::as_f64(&value), R::as_f64(&in_min), R::as_f64(&out_min),
        R::as_f64(&in_max), R::as_f64(&out_max),
    ) {
        (Some(value), Some(in_min), Some(out_min), Some(in_max), Some(out_max)) =>
            R::ok(R::from_f64((value - in_min) / (in_max - in_min) * (out_max - out_min) + out_min)),
        _ => R::err(R::from_string("map_range expects numeric arguments".to_string())),
    }
}

// ---- integer helpers (any integer -> int / bool) ---------------------------

#[native_fn(module = "math",
    sig(int -> result[int]),
    sig(byte -> result[int]), sig(sbyte -> result[int]),
    sig(bbyte -> result[int]), sig(bsbyte -> result[int]),
    sig(sint -> result[int]), sig(suint -> result[int]),
    sig(uint -> result[int])
)]
pub fn factorial<R: Runtime>(_cx: &mut R::Cx, x: R::Value) -> R::Value {
    match R::as_i64(&x) {
        Some(x) => R::ok(R::from_i64((1..=x).product())),
        None => R::err(R::from_string(format!("factorial expects an integer, got {}", R::type_name(&x)))),
    }
}

#[native_fn(module = "math",
    sig(int -> result[int]),
    sig(byte -> result[int]), sig(sbyte -> result[int]),
    sig(bbyte -> result[int]), sig(bsbyte -> result[int]),
    sig(sint -> result[int]), sig(suint -> result[int]),
    sig(uint -> result[int])
)]
pub fn fibonacci<R: Runtime>(_cx: &mut R::Cx, x: R::Value) -> R::Value {
    match R::as_i64(&x) {
        Some(x) => {
            let (mut a, mut b) = (0i64, 1i64);
            for _ in 0..x {
                (a, b) = (b, a + b);
            }
            R::ok(R::from_i64(a))
        }
        None => R::err(R::from_string(format!("fibonacci expects an integer, got {}", R::type_name(&x)))),
    }
}

#[native_fn(module = "math",
    sig(int, int -> result[int]),
    sig(byte, byte -> result[int]), sig(sbyte, sbyte -> result[int]),
    sig(bbyte, bbyte -> result[int]), sig(bsbyte, bsbyte -> result[int]),
    sig(sint, sint -> result[int]), sig(suint, suint -> result[int]),
    sig(uint, uint -> result[int])
)]
pub fn gcd<R: Runtime>(_cx: &mut R::Cx, x: R::Value, y: R::Value) -> R::Value {
    match (R::as_i64(&x), R::as_i64(&y)) {
        (Some(x), Some(y)) => {
            let mut a = x as u64;
            let mut b = y as u64;
            while b != 0 {
                (a, b) = (b, a % b);
            }
            R::ok(R::from_i64(a as i64))
        }
        _ => R::err(R::from_string(format!(
            "gcd expects integers, got ({}, {})",
            R::type_name(&x), R::type_name(&y)
        ))),
    }
}

#[native_fn(module = "math",
    sig(int, int -> result[int]),
    sig(byte, byte -> result[int]), sig(sbyte, sbyte -> result[int]),
    sig(bbyte, bbyte -> result[int]), sig(bsbyte, bsbyte -> result[int]),
    sig(sint, sint -> result[int]), sig(suint, suint -> result[int]),
    sig(uint, uint -> result[int])
)]
pub fn lcm<R: Runtime>(_cx: &mut R::Cx, x: R::Value, y: R::Value) -> R::Value {
    match (R::as_i64(&x), R::as_i64(&y)) {
        (Some(x), Some(y)) => {
            let a = x as u64;
            let b = y as u64;
            let mut ra = a;
            let mut rb = b;
            while rb != 0 {
                (ra, rb) = (rb, ra % rb);
            }
            R::ok(R::from_i64((a / ra * b) as i64))
        }
        _ => R::err(R::from_string(format!(
            "lcm expects integers, got ({}, {})",
            R::type_name(&x), R::type_name(&y)
        ))),
    }
}

#[native_fn(module = "math",
    sig(int -> result[bool]),
    sig(byte -> result[bool]), sig(sbyte -> result[bool]),
    sig(bbyte -> result[bool]), sig(bsbyte -> result[bool]),
    sig(sint -> result[bool]), sig(suint -> result[bool]),
    sig(uint -> result[bool])
)]
pub fn is_prime<R: Runtime>(_cx: &mut R::Cx, x: R::Value) -> R::Value {
    match R::as_i64(&x) {
        Some(x) => {
            let x = x as u64;
            if x < 2 {
                return R::ok(R::from_bool(false));
            }
            if x < 4 {
                return R::ok(R::from_bool(true));
            }
            if x.is_multiple_of(2) || x.is_multiple_of(3) {
                return R::ok(R::from_bool(false));
            }
            let mut i = 5;
            while i * i <= x {
                if x.is_multiple_of(i) || x.is_multiple_of(i + 2) {
                    return R::ok(R::from_bool(false));
                }
                i += 6;
            }
            R::ok(R::from_bool(true))
        }
        None => R::err(R::from_string(format!("is_prime expects an integer, got {}", R::type_name(&x)))),
    }
}

// ---- constants submodule (`std::math::consts`) -----------------------------

/// `std::math::consts` - mathematical constants from [`std::f64::consts`].
///
/// Every constant is a zero-argument function returning `f64` (called as
/// `PI()`, not accessed as a bare value). `is_inf`/`is_nan` are the two
/// exceptions: they take a `float` and return `bool`.
pub mod constants {
    use rl_std_macros::native_fn;

    #[native_fn(module = "consts", name = "E")]
    pub fn e() -> f64 {
        std::f64::consts::E
    }

    #[native_fn(module = "consts", name = "PI")]
    pub fn pi() -> f64 {
        std::f64::consts::PI
    }

    #[native_fn(module = "consts", name = "PHI")]
    pub fn phi() -> f64 {
        std::f64::consts::GOLDEN_RATIO
    }

    #[native_fn(module = "consts", name = "TAU")]
    pub fn tau() -> f64 {
        std::f64::consts::TAU
    }

    #[native_fn(module = "consts", name = "INF")]
    pub fn inf() -> f64 {
        f64::INFINITY
    }

    #[native_fn(module = "consts", name = "NAN")]
    pub fn nan() -> f64 {
        f64::NAN
    }

    #[native_fn(module = "consts", name = "is_inf")]
    pub fn is_inf(x: f64) -> bool {
        x.is_infinite()
    }

    #[native_fn(module = "consts", name = "is_nan")]
    pub fn is_nan(x: f64) -> bool {
        x.is_nan()
    }

    #[native_fn(module = "consts", name = "FRAC_1_PI")]
    pub fn frac_1_pi() -> f64 {
        std::f64::consts::FRAC_1_PI
    }

    #[native_fn(module = "consts", name = "FRAC_2_PI")]
    pub fn frac_2_pi() -> f64 {
        std::f64::consts::FRAC_2_PI
    }

    #[native_fn(module = "consts", name = "FRAC_1_SQRT_2")]
    pub fn frac_1_sqrt_2() -> f64 {
        std::f64::consts::FRAC_1_SQRT_2
    }

    #[native_fn(module = "consts", name = "FRAC_2_SQRT_PI")]
    pub fn frac_2_sqrt_pi() -> f64 {
        std::f64::consts::FRAC_2_SQRT_PI
    }

    #[native_fn(module = "consts", name = "EULER_GAMMA")]
    pub fn euler_gamma() -> f64 {
        std::f64::consts::EULER_GAMMA
    }

    #[native_fn(module = "consts", name = "LN_10")]
    pub fn ln_10() -> f64 {
        std::f64::consts::LN_10
    }

    #[native_fn(module = "consts", name = "LN_2")]
    pub fn ln_2() -> f64 {
        std::f64::consts::LN_2
    }

    #[native_fn(module = "consts", name = "LOG2_E")]
    pub fn log2_e() -> f64 {
        std::f64::consts::LOG2_E
    }

    #[native_fn(module = "consts", name = "LOG2_10")]
    pub fn log2_10() -> f64 {
        std::f64::consts::LOG2_10
    }

    #[native_fn(module = "consts", name = "LOG10_2")]
    pub fn log10_2() -> f64 {
        std::f64::consts::LOG10_2
    }

    #[native_fn(module = "consts", name = "LOG10_E")]
    pub fn log10_e() -> f64 {
        std::f64::consts::LOG10_E
    }

    #[native_fn(module = "consts", name = "FRAC_PI_2")]
    pub fn frac_pi_2() -> f64 {
        std::f64::consts::FRAC_PI_2
    }

    #[native_fn(module = "consts", name = "FRAC_PI_3")]
    pub fn frac_pi_3() -> f64 {
        std::f64::consts::FRAC_PI_3
    }

    #[native_fn(module = "consts", name = "FRAC_PI_4")]
    pub fn frac_pi_4() -> f64 {
        std::f64::consts::FRAC_PI_4
    }

    #[native_fn(module = "consts", name = "FRAC_PI_6")]
    pub fn frac_pi_6() -> f64 {
        std::f64::consts::FRAC_PI_6
    }

    #[native_fn(module = "consts", name = "FRAC_PI_8")]
    pub fn frac_pi_8() -> f64 {
        std::f64::consts::FRAC_PI_8
    }

    #[native_fn(module = "consts", name = "SQRT_2")]
    pub fn sqrt_2() -> f64 {
        std::f64::consts::SQRT_2
    }

    rl_std_core::native_module!("consts";
        funcs: [
            e, pi, phi, tau, inf, nan,
            is_inf, is_nan,
            frac_1_pi, frac_2_pi, frac_1_sqrt_2, frac_2_sqrt_pi,
            euler_gamma, ln_10, ln_2,
            log2_e, log2_10, log10_2, log10_e,
            frac_pi_2, frac_pi_3, frac_pi_4, frac_pi_6, frac_pi_8,
            sqrt_2,
        ],
    );
}

rl_std_core::native_module!("math";
    funcs: [
        abs, ceil, floor, round,
        clamp, max, min,
        modulo, pow, log,
        sqrt, log2, log10,
        sin, cos, tan, atan, acos, asin,
        degrees, radians, exp, sign,
        atan2, hypot, lerp, map_range,
        factorial, fibonacci, gcd, lcm, is_prime,
    ],
    mods: [ constants ],
);
