use rl_vm::VmValue;

use crate::common::compile_and_run;

#[test]
fn bit_and_int() {
    let result = compile_and_run(
        r#"
get bit_and from std::bitwise
dec int x = bit_and(5, 3)?
x
        "#,
    )
    .unwrap();

    assert_eq!(result, VmValue::Int(1));
}

#[test]
fn bit_or_int() {
    let result = compile_and_run(
        r#"
get bit_or from std::bitwise
dec int x = bit_or(5, 3)?
x
        "#,
    )
    .unwrap();

    assert_eq!(result, VmValue::Int(7));
}

#[test]
fn bit_xor_int() {
    let result = compile_and_run(
        r#"
get bit_xor from std::bitwise
dec int x = bit_xor(5, 3)?
x
        "#,
    )
    .unwrap();

    assert_eq!(result, VmValue::Int(6));
}

#[test]
fn bit_not_byte() {
    let result = compile_and_run(
        r#"
get bit_not from std::bitwise
dec int x = bit_not(0)?
x
        "#,
    )
    .unwrap();

    assert_eq!(result, VmValue::Int(!0i64));
}

#[test]
fn bit_shift_left_int() {
    let result = compile_and_run(
        r#"
get bit_shift_left from std::bitwise
dec int x = bit_shift_left(5, 1)?
x
        "#,
    )
    .unwrap();

    assert_eq!(result, VmValue::Int(10));
}

#[test]
fn bit_shift_right_int() {
    let result = compile_and_run(
        r#"
get bit_shift_right from std::bitwise
dec int x = bit_shift_right(10, 1)?
x
        "#,
    )
    .unwrap();

    assert_eq!(result, VmValue::Int(5));
}

#[test]
fn count_bits_int() {
    let result = compile_and_run(
        r#"
get count_bits from std::bitwise
dec int x = count_bits(7)?
x
        "#,
    )
    .unwrap();

    assert_eq!(result, VmValue::Int(3));
}

#[test]
fn leading_zeros_int() {
    let result = compile_and_run(
        r#"
get leading_zeros from std::bitwise
dec int x = leading_zeros(8)?
x
        "#,
    )
    .unwrap();

    assert_eq!(result, VmValue::Int(8i64.leading_zeros() as i64));
}

#[test]
fn trailing_zeros_int() {
    let result = compile_and_run(
        r#"
get trailing_zeros from std::bitwise
dec int x = trailing_zeros(8)?
x
        "#,
    )
    .unwrap();

    assert_eq!(result, VmValue::Int(8i64.trailing_zeros() as i64));
}
