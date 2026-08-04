use std::rc::Rc;

use rl_vm::VmValue;

use crate::common::compile_and_run;

#[test]
fn float_to_int() {
    let result = compile_and_run(
        r#"
get to_int from std::types
dec int x = to_int(3.9)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(3));
}

#[test]
fn string_to_int() {
    let result = compile_and_run(
        r#"
get to_int from std::types
dec int x = to_int("42")?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(42));
}

#[test]
fn bool_true_to_int() {
    let result = compile_and_run(
        r#"
get to_int from std::types
dec int x = to_int(true)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(1));
}

#[test]
fn bool_false_to_int() {
    let result = compile_and_run(
        r#"
get to_int from std::types
dec int x = to_int(false)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(0));
}

#[test]
fn int_to_float() {
    let result = compile_and_run(
        r#"
get to_float from std::types
dec float x = to_float(5)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Float(5.0));
}

#[test]
fn string_to_float() {
    let result = compile_and_run(
        r#"
get to_float from std::types
dec float x = to_float("4.14")?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Float(4.14));
}

#[test]
fn int_to_string() {
    let result = compile_and_run(
        r#"
get to_string from std::types
dec string x = to_string(99)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("99")));
}

#[test]
fn bool_to_string() {
    let result = compile_and_run(
        r#"
get to_string from std::types
dec string x = to_string(true)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("true")));
}

#[test]
fn float_to_string() {
    let result = compile_and_run(
        r#"
get to_string from std::types
dec string x = to_string(1.5)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("1.5")));
}

#[test]
fn one_to_bool_true() {
    let result = compile_and_run(
        r#"
get to_bool from std::types
dec bool x = to_bool(1)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn zero_to_bool_false() {
    let result = compile_and_run(
        r#"
get to_bool from std::types
dec bool x = to_bool(0)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(false));
}

#[test]
fn is_int_true() {
    let result = compile_and_run(
        r#"
get is_int from std::types
dec bool x = is_int(42)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn is_int_false_for_float() {
    let result = compile_and_run(
        r#"
get is_int from std::types
dec bool x = is_int(1.0)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(false));
}

#[test]
fn is_float_true() {
    let result = compile_and_run(
        r#"
get is_float from std::types
dec bool x = is_float(3.14)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn is_string_true() {
    let result = compile_and_run(
        r#"
get is_string from std::types
dec bool x = is_string("hi")?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn is_bool_true() {
    let result = compile_and_run(
        r#"
get is_bool from std::types
dec bool x = is_bool(false)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn is_null_true() {
    let result = compile_and_run(
        r#"
get is_null from std::types
dec bool x = is_null(null)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn is_null_false_for_int() {
    let result = compile_and_run(
        r#"
get is_null from std::types
dec bool x = is_null(0)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(false));
}

#[test]
fn to_hex() {
    let result = compile_and_run(
        r#"
get to_hex from std::types
dec string x = to_hex(255)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("ff")));
}

#[test]
fn to_bin() {
    let result = compile_and_run(
        r#"
get to_bin from std::types
dec string x = to_bin(5)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("101")));
}

#[test]
fn to_oct() {
    let result = compile_and_run(
        r#"
get to_oct from std::types
dec string x = to_oct(8)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("10")));
}

#[test]
fn to_bool_from_true_string() {
    let result = compile_and_run(
        r#"
get to_bool from std::types
dec bool x = to_bool("true")?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn to_bool_from_int_one() {
    let result = compile_and_run(
        r#"
get to_bool from std::types
dec bool x = to_bool(1)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn to_bool_from_int_zero() {
    let result = compile_and_run(
        r#"
get to_bool from std::types
dec bool x = to_bool(0)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(false));
}

#[test]
fn to_string_from_int() {
    let result = compile_and_run(
        r#"
get to_string from std::types
dec string x = to_string(42)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("42")));
}

#[test]
fn to_string_from_float() {
    let result = compile_and_run(
        r#"
get to_string from std::types
dec string x = to_string(3.14)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("3.14")));
}

#[test]
fn to_string_from_bool() {
    let result = compile_and_run(
        r#"
get to_string from std::types
dec string x = to_string(true)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("true")));
}

#[test]
fn to_hex_from_int() {
    let result = compile_and_run(
        r#"
get to_hex from std::types
dec string x = to_hex(255)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("ff")));
}

#[test]
fn to_bin_from_int() {
    let result = compile_and_run(
        r#"
get to_bin from std::types
dec string x = to_bin(5)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("101")));
}

#[test]
fn to_oct_from_int() {
    let result = compile_and_run(
        r#"
get to_oct from std::types
dec string x = to_oct(8)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("10")));
}

#[test]
fn is_int_false() {
    let result = compile_and_run(
        r#"
get is_int from std::types
dec bool x = is_int("hello")?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(false));
}

#[test]
fn is_null_false() {
    let result = compile_and_run(
        r#"
get is_null from std::types
dec bool x = is_null(42)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(false));
}
