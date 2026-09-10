use crate::common;
use rl_vm::VmValue;

#[test]
fn basic_variable_arithemtic() {
    let result = common::compile_and_run(
        r#"
        1 + 2 * 4 * 4 + 342 * 1
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Int(375))
}

#[test]
fn uint_arithmetic() {
    let result = common::compile_and_run(r#"(1000 as uint) + (1 as uint)"#).expect("vm run failed");
    assert_eq!(result, VmValue::UInt(1001))
}

#[test]
fn uint_multiplication() {
    let result = common::compile_and_run(r#"(10 as uint) * (5 as uint)"#).expect("vm run failed");
    assert_eq!(result, VmValue::UInt(50))
}

#[test]
fn uint_division() {
    let result = common::compile_and_run(r#"(10 as uint) / (3 as uint)"#).expect("vm run failed");
    assert_eq!(result, VmValue::UInt(3))
}

#[test]
fn small_uint_arithmetic() {
    let result = common::compile_and_run(r#"(1000 as small uint) + (1 as small uint)"#)
        .expect("vm run failed");
    assert_eq!(result, VmValue::SUInt(1001))
}

#[test]
fn uint_comparison() {
    let result = common::compile_and_run(r#"(1 as uint) < (2 as uint)"#).expect("vm run failed");
    assert_eq!(result, VmValue::Bool(true))
}

#[test]
fn uint_overflow_is_error() {
    assert!(common::compile_and_run(r#"(18446744073709551615 as uint) + (1 as uint)"#).is_err());
}

#[test]
fn uint_underflow_is_error() {
    assert!(common::compile_and_run(r#"(1 as uint) - (2 as uint)"#).is_err());
}

#[test]
fn cast_value_to_int() {
    let result = common::compile_and_run("65.5 as int").expect("vm run failed");
    assert_eq!(result, VmValue::Int(65));
}

#[test]
fn cast_value_to_float() {
    let result = common::compile_and_run("3 as float").expect("vm run failed");
    assert_eq!(result, VmValue::Float(3.0));
}

#[test]
fn cast_value_to_byte() {
    let result = common::compile_and_run("200 as byte").expect("vm run failed");
    assert_eq!(result, VmValue::Byte(200));
}

#[test]
fn cast_value_to_small_int() {
    let result = common::compile_and_run("65536 as small int").expect("vm run failed");
    assert_eq!(result, VmValue::SInt(65536));
}

#[test]
fn cast_value_to_small_float() {
    let result = common::compile_and_run("7 as small float").expect("vm run failed");
    assert_eq!(result, VmValue::SFloat(7.0));
}

#[test]
fn units_are_discarded_by_the_vm() {
    let result = common::compile_and_run(
        r#"
        dec float distance: m = 100.0
        dec float time: s = 4.0
        dec float speed: m/s = distance / time
        speed * time
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Float(100.0));
}

#[test]
fn const_with_unit_runs_in_vm() {
    let result = common::compile_and_run(
        r#"
        CONST float SPEED: m/s = 12.5
        dec float time: s = 4.0
        SPEED * time
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Float(50.0));
}

#[test]
fn convert_attribute_is_discarded_before_runtime() {
    let result = common::compile_and_run(
        r#"
        #![convert(kg=1000(g))]
        dec float weight_kg: kg = 2.5
        dec float weight_g: g = weight_kg
        weight_g
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Float(2.5));
}

#[test]
fn suffix_u8_literal() {
    let result = common::compile_and_run("10_u8").expect("vm run failed");
    assert_eq!(result, VmValue::Byte(10));
}

#[test]
fn suffix_i8_literal() {
    let result = common::compile_and_run("10_i8").expect("vm run failed");
    assert_eq!(result, VmValue::SByte(10));
}

#[test]
fn suffix_u16_literal() {
    let result = common::compile_and_run("1000_u16").expect("vm run failed");
    assert_eq!(result, VmValue::BByte(1000));
}

#[test]
fn suffix_i16_literal() {
    let result = common::compile_and_run("1000_i16").expect("vm run failed");
    assert_eq!(result, VmValue::BSByte(1000));
}

#[test]
fn suffix_i32_literal() {
    let result = common::compile_and_run("100_i32").expect("vm run failed");
    assert_eq!(result, VmValue::SInt(100));
}

#[test]
fn suffix_u32_literal() {
    let result = common::compile_and_run("100_u32").expect("vm run failed");
    assert_eq!(result, VmValue::SUInt(100));
}

#[test]
#[allow(clippy::approx_constant)]
fn suffix_f32_literal() {
    let result = common::compile_and_run("3.14_f32").expect("vm run failed");
    assert!(matches!(result, VmValue::SFloat(v) if (v - 3.14).abs() < 0.001));
}

#[test]
fn suffix_i64_literal() {
    let result = common::compile_and_run("100_i64").expect("vm run failed");
    assert_eq!(result, VmValue::Int(100));
}

#[test]
fn suffix_u64_literal() {
    let result = common::compile_and_run("100_u64").expect("vm run failed");
    assert_eq!(result, VmValue::UInt(100));
}

#[test]
#[allow(clippy::approx_constant)]
fn suffix_f64_literal() {
    let result = common::compile_and_run("3.14_f64").expect("vm run failed");
    assert_eq!(result, VmValue::Float(3.14));
}

#[test]
fn suffix_u8_arithmetic() {
    let result = common::compile_and_run("10_u8 + 5_u8").expect("vm run failed");
    assert_eq!(result, VmValue::Byte(15));
}

#[test]
fn suffix_i64_arithmetic() {
    let result = common::compile_and_run("100_i64 + 50_i64").expect("vm run failed");
    assert_eq!(result, VmValue::Int(150));
}

#[test]
fn suffix_u8_overflow_is_error() {
    assert!(common::compile_and_run("255_u8 + 1_u8").is_err());
}

#[test]
fn suffix_sbyte_negative() {
    let result = common::compile_and_run("-10_i8").expect("vm run failed");
    assert_eq!(result, VmValue::SByte(-10));
}

#[test]
fn suffix_matches_as_cast() {
    let result = common::compile_and_run(
        r#"
dec byte a = 10_u8
dec byte b = 10 as byte
a == b
"#,
    )
    .expect("vm run failed");
    assert_eq!(result, VmValue::Bool(true));
}

// ---- Phase 3: stdlib functions accepting all numeric types -----------------

#[test]
fn abs_with_u8() {
    let result = common::compile_and_run(r#"
get abs from std::math
dec byte a = 10_u8
abs(a)?
"#).expect("vm run failed");
    assert_eq!(result, VmValue::Int(10));
}

#[test]
fn abs_with_i8() {
    let result = common::compile_and_run(r#"
get abs from std::math
abs(-10_i8)?
"#).expect("vm run failed");
    assert_eq!(result, VmValue::Int(10));
}

#[test]
fn abs_with_u16() {
    let result = common::compile_and_run(r#"
get abs from std::math
abs(1000_u16)?
"#).expect("vm run failed");
    assert_eq!(result, VmValue::Int(1000));
}

#[test]
fn abs_with_i16() {
    let result = common::compile_and_run(r#"
get abs from std::math
abs(-1000_i16)?
"#).expect("vm run failed");
    assert_eq!(result, VmValue::Int(1000));
}

#[test]
fn abs_with_i32() {
    let result = common::compile_and_run(r#"
get abs from std::math
abs(-100_i32)?
"#).expect("vm run failed");
    assert_eq!(result, VmValue::Int(100));
}

#[test]
fn abs_with_u32() {
    let result = common::compile_and_run(r#"
get abs from std::math
abs(100_u32)?
"#).expect("vm run failed");
    assert_eq!(result, VmValue::Int(100));
}

#[test]
fn abs_with_u64() {
    let result = common::compile_and_run(r#"
get abs from std::math
dec uint a = 100
abs(a)?
"#).expect("vm run failed");
    assert_eq!(result, VmValue::Int(100));
}

#[test]
fn ceil_with_i32() {
    let result = common::compile_and_run(r#"
get ceil from std::math
ceil(3_i32)?
"#).expect("vm run failed");
    assert_eq!(result, VmValue::Int(3));
}

#[test]
fn floor_with_i32() {
    let result = common::compile_and_run(r#"
get floor from std::math
floor(3_i32)?
"#).expect("vm run failed");
    assert_eq!(result, VmValue::Int(3));
}

#[test]
fn round_with_i32() {
    let result = common::compile_and_run(r#"
get round from std::math
round(3_i32)?
"#).expect("vm run failed");
    assert_eq!(result, VmValue::Int(3));
}

#[test]
fn sqrt_with_u16() {
    let result = common::compile_and_run(r#"
get sqrt from std::math
sqrt(25_u16)?
"#).expect("vm run failed");
    assert!(matches!(result, VmValue::Float(v) if (v - 5.0).abs() < 0.001));
}

#[test]
fn sin_with_i32() {
    let result = common::compile_and_run(r#"
get sin from std::math
sin(0_i32)?
"#).expect("vm run failed");
    assert!(matches!(result, VmValue::Float(v) if v.abs() < 0.001));
}

#[test]
fn cos_with_u8() {
    let result = common::compile_and_run(r#"
get cos from std::math
cos(0_u8)?
"#).expect("vm run failed");
    assert!(matches!(result, VmValue::Float(v) if (v - 1.0).abs() < 0.001));
}

#[test]
fn factorial_with_u8() {
    let result = common::compile_and_run(r#"
get factorial from std::math
factorial(5_u8)?
"#).expect("vm run failed");
    assert_eq!(result, VmValue::Int(120));
}

#[test]
fn factorial_with_i32() {
    let result = common::compile_and_run(r#"
get factorial from std::math
factorial(5_i32)?
"#).expect("vm run failed");
    assert_eq!(result, VmValue::Int(120));
}

#[test]
fn is_prime_with_u8() {
    let result = common::compile_and_run(r#"
get is_prime from std::math
is_prime(7_u8)?
"#).expect("vm run failed");
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn gcd_with_u32() {
    let result = common::compile_and_run(r#"
get gcd from std::math
gcd(12_u32, 8_u32)?
"#).expect("vm run failed");
    assert_eq!(result, VmValue::Int(4));
}

#[test]
fn lcm_with_i32() {
    let result = common::compile_and_run(r#"
get lcm from std::math
lcm(4_i32, 6_i32)?
"#).expect("vm run failed");
    assert_eq!(result, VmValue::Int(12));
}

#[test]
fn max_with_u8() {
    let result = common::compile_and_run(r#"
get max from std::math
max(10_u8, 20_u8)?
"#).expect("vm run failed");
    assert_eq!(result, VmValue::Int(20));
}

#[test]
fn min_with_i32() {
    let result = common::compile_and_run(r#"
get min from std::math
min(10_i32, 20_i32)?
"#).expect("vm run failed");
    assert_eq!(result, VmValue::Int(10));
}

#[test]
fn clamp_with_u16() {
    let result = common::compile_and_run(r#"
get clamp from std::math
clamp(100_u16, 0_u16, 50_u16)?
"#).expect("vm run failed");
    assert_eq!(result, VmValue::Int(50));
}

#[test]
fn mod_with_u8() {
    let result = common::compile_and_run(r#"
get mod from std::math
dec byte a = 10_u8
mod(a, 3_u8)?
"#).expect("vm run failed");
    assert_eq!(result, VmValue::Int(1));
}

#[test]
fn pow_with_u8() {
    let result = common::compile_and_run(r#"
get pow from std::math
pow(2_u8, 3_u8)?
"#).expect("vm run failed");
    assert_eq!(result, VmValue::Int(8));
}

#[test]
fn log2_with_u32() {
    let result = common::compile_and_run(r#"
get log2 from std::math
log2(8_u32)?
"#).expect("vm run failed");
    assert!(matches!(result, VmValue::Float(v) if (v - 3.0).abs() < 0.001));
}

#[test]
fn log10_with_i32() {
    let result = common::compile_and_run(r#"
get log10 from std::math
log10(100_i32)?
"#).expect("vm run failed");
    assert!(matches!(result, VmValue::Float(v) if (v - 2.0).abs() < 0.001));
}

#[test]
fn bit_and_with_u8() {
    let result = common::compile_and_run(r#"
get bit_and from std::bitwise
dec byte a = 255_u8
bit_and(a, 15_u8)?
"#).expect("vm run failed");
    assert_eq!(result, VmValue::Byte(15));
}

#[test]
fn bit_or_with_u16() {
    let result = common::compile_and_run(r#"
get bit_or from std::bitwise
dec big byte a = 65280_u16
bit_or(a, 255_u16)?
"#).expect("vm run failed");
    assert_eq!(result, VmValue::Int(65535));
}

#[test]
fn bit_not_with_i32() {
    let result = common::compile_and_run(r#"
get bit_not from std::bitwise
bit_not(0_i32)?
"#).expect("vm run failed");
    assert_eq!(result, VmValue::Int(-1));
}

#[test]
fn bit_shift_left_with_u8() {
    let result = common::compile_and_run(r#"
get bit_shift_left from std::bitwise
bit_shift_left(1_u8, 4_u8)?
"#).expect("vm run failed");
    assert_eq!(result, VmValue::Byte(16));
}

#[test]
fn count_bits_with_u16() {
    let result = common::compile_and_run(r#"
get count_bits from std::bitwise
count_bits(255_u16)?
"#).expect("vm run failed");
    assert_eq!(result, VmValue::Int(8));
}
