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

#[native_fn(module = "math", sig(int -> result[int]), sig(float -> result[float]))]
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

#[native_fn(module = "math", sig(int -> result[int]), sig(float -> result[float]))]
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

#[native_fn(module = "math", sig(int -> result[int]), sig(float -> result[float]))]
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

#[native_fn(module = "math", sig(int -> result[int]), sig(float -> result[float]))]
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
    sig(float, float, float -> result[float])
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
    sig(float, float -> result[float])
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
    sig(float, float -> result[float])
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
    sig(float, int -> result[float])
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
    sig(float, int -> result[float])
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
    sig(float, int -> result[float])
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

#[native_fn(module = "math", sig(int -> result[float]), sig(float -> result[float]))]
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

#[native_fn(module = "math", sig(int -> result[float]), sig(float -> result[float]))]
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

#[native_fn(module = "math", sig(int -> result[float]), sig(float -> result[float]))]
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

// ---- plain trig / exp family (concrete f64 -> f64) -------------------------

#[native_fn(module = "math")]
pub fn sin(x: f64) -> f64 {
    x.sin()
}

#[native_fn(module = "math")]
pub fn cos(x: f64) -> f64 {
    x.cos()
}

#[native_fn(module = "math")]
pub fn tan(x: f64) -> f64 {
    x.tan()
}

#[native_fn(module = "math")]
pub fn atan(x: f64) -> f64 {
    x.atan()
}

#[native_fn(module = "math")]
pub fn acos(x: f64) -> f64 {
    x.acos()
}

#[native_fn(module = "math")]
pub fn asin(x: f64) -> f64 {
    x.asin()
}

#[native_fn(module = "math")]
pub fn degrees(x: f64) -> f64 {
    x.to_degrees()
}

#[native_fn(module = "math")]
pub fn radians(x: f64) -> f64 {
    x.to_radians()
}

#[native_fn(module = "math")]
pub fn exp(x: f64) -> f64 {
    x.exp()
}

#[native_fn(module = "math")]
pub fn sign(x: f64) -> f64 {
    if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        0.0
    }
}

#[native_fn(module = "math")]
pub fn atan2(x: f64, y: f64) -> f64 {
    y.atan2(x)
}

#[native_fn(module = "math")]
pub fn hypot(x: f64, y: f64) -> f64 {
    x.hypot(y)
}

#[native_fn(module = "math")]
pub fn lerp(x: f64, y: f64, t: f64) -> f64 {
    x + (y - x) * t
}

#[native_fn(module = "math")]
pub fn map_range(value: f64, in_min: f64, out_min: f64, in_max: f64, out_max: f64) -> f64 {
    (value - in_min) / (in_max - in_min) * (out_max - out_min) + out_min
}

// ---- plain integer helpers (concrete i64 -> i64 / bool) --------------------

#[native_fn(module = "math")]
pub fn factorial(x: i64) -> i64 {
    (1..=x).product()
}

#[native_fn(module = "math")]
pub fn fibonacci(x: i64) -> i64 {
    let (mut a, mut b) = (0, 1);
    for _ in 0..x {
        (a, b) = (b, a + b);
    }
    a
}

#[native_fn(module = "math")]
pub fn gcd(x: i64, y: i64) -> i64 {
    let mut a = x as u64;
    let mut b = y as u64;
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a as i64
}

#[native_fn(module = "math")]
pub fn lcm(x: i64, y: i64) -> i64 {
    let mut a = x as u64;
    let mut b = y as u64;
    let a_ = x as u64;
    let b_ = y as u64;
    while b != 0 {
        (a, b) = (b, a % b);
    }
    (a_ / a * b_) as i64
}

#[native_fn(module = "math")]
pub fn is_prime(x: i64) -> bool {
    let x = x as u64;
    if x < 2 {
        return false;
    }
    if x < 4 {
        return true;
    }
    if x.is_multiple_of(2) || x.is_multiple_of(3) {
        return false;
    }
    let mut i = 5;
    while i * i <= x {
        if x.is_multiple_of(i) || x.is_multiple_of(i + 2) {
            return false;
        }
        i += 6;
    }
    true
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
