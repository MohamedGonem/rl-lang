use rl_vm::VmValue;

use crate::common::compile_and_run;

#[test]
fn abs_positive() {
    let result = compile_and_run(
        r#"
get abs from std::math
dec int x = abs(5)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(5));
}

#[test]
fn abs_negative() {
    let result = compile_and_run(
        r#"
get abs from std::math
dec int x = abs(-7)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(7));
}

#[test]
fn max_returns_larger() {
    let result = compile_and_run(
        r#"
get max from std::math
dec int x = max(3, 9)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(9));
}

#[test]
fn min_returns_smaller() {
    let result = compile_and_run(
        r#"
get min from std::math
dec int x = min(3, 9)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(3));
}

#[test]
fn pow_integer() {
    let result = compile_and_run(
        r#"
get pow from std::math
dec int x = pow(2, 10)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(1024));
}

#[test]
fn sqrt_float() {
    let result = compile_and_run(
        r#"
get sqrt from std::math
dec float x = sqrt(9.0)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Float(3.0));
}

#[test]
fn floor_float() {
    let result = compile_and_run(
        r#"
get floor from std::math
dec float x = floor(3.9)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Float(3.0));
}

#[test]
fn ceil_float() {
    let result = compile_and_run(
        r#"
get ceil from std::math
dec float x = ceil(3.1)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Float(4.0));
}

#[test]
fn round_float_up() {
    let result = compile_and_run(
        r#"
get round from std::math
dec float x = round(2.6)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Float(3.0));
}

#[test]
fn round_float_down() {
    let result = compile_and_run(
        r#"
get round from std::math
dec float x = round(2.4)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Float(2.0));
}

#[test]
fn clamp_within_range() {
    let result = compile_and_run(
        r#"
get clamp from std::math
dec int x = clamp(5, 0, 10)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(5));
}

#[test]
fn clamp_below_min() {
    let result = compile_and_run(
        r#"
get clamp from std::math
dec int x = clamp(-5, 0, 10)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(0));
}

#[test]
fn clamp_above_max() {
    let result = compile_and_run(
        r#"
get clamp from std::math
dec int x = clamp(99, 0, 10)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(10));
}

#[test]
fn mod_operation() {
    let result = compile_and_run(
        r#"
get mod from std::math
dec int x = mod(10, 3)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(1));
}

#[test]
fn log2_eight() {
    let result = compile_and_run(
        r#"
get log2 from std::math
dec float x = log2(8.0)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Float(3.0));
}

#[test]
fn log10_thousand() {
    let result = compile_and_run(
        r#"
get log10 from std::math
dec float x = log10(1000.0)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Float(3.0));
}
