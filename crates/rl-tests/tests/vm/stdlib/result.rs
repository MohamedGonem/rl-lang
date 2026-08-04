use std::rc::Rc;

use rl_vm::VmValue;

use crate::common::compile_and_run;

#[test]
fn is_ok_on_ok() {
    let result = compile_and_run(
        r#"
get is_ok from std::res
dec bool x = is_ok(ok(42))
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn is_ok_on_err() {
    let result = compile_and_run(
        r#"
get is_ok from std::res
dec bool x = is_ok(err("oops"))
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(false));
}

#[test]
fn is_err_on_err() {
    let result = compile_and_run(
        r#"
get is_err from std::res
dec bool x = is_err(err("oops"))
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn is_err_on_ok() {
    let result = compile_and_run(
        r#"
get is_err from std::res
dec bool x = is_err(ok(42))
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(false));
}

#[test]
fn result_unwrap_ok() {
    let result = compile_and_run(
        r#"
get result_unwrap from std::res
dec int x = result_unwrap(ok(99))
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(99));
}

#[test]
fn result_unwrap_panics_on_err() {
    let result = compile_and_run(
        r#"
get result_unwrap from std::res
dec int x = result_unwrap(err("boom"))
"#,
    );
    assert!(result.is_err());
}

#[test]
fn result_unwrap_err_on_err() {
    let result = compile_and_run(
        r#"
get result_unwrap_err from std::res
dec string x = result_unwrap_err(err("failed"))
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("failed")));
}

#[test]
fn result_unwrap_err_panics_on_ok() {
    let result = compile_and_run(
        r#"
get result_unwrap_err from std::res
dec int x = result_unwrap_err(ok(42))
"#,
    );
    assert!(result.is_err());
}

#[test]
fn result_unwrap_or_ok_returns_inner() {
    let result = compile_and_run(
        r#"
get result_unwrap_or from std::res
dec int x = result_unwrap_or(ok(7), -1)
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(7));
}

#[test]
fn result_unwrap_or_err_returns_default() {
    let result = compile_and_run(
        r#"
get result_unwrap_or from std::res
dec int x = result_unwrap_or(err("gone"), -1)
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(-1));
}

#[test]
fn result_map_transforms_ok() {
    let result = compile_and_run(
        r#"
get result_map, result_unwrap from std::res
dec result[int] r = result_map(ok(5), fn(int n) -> int { return n * 2 })
dec int x = result_unwrap(r)
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(10));
}

#[test]
fn result_map_passes_through_err() {
    let result = compile_and_run(
        r#"
get result_map, is_err from std::res
dec result[int] r = result_map(err("dead"), fn(int n) -> int { return n * 2 })
dec bool x = is_err(r)?
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn result_map_err_transforms_err() {
    let result = compile_and_run(
        r#"
get result_map_err, result_unwrap_err from std::res
get concat from std::str
dec result[int] r = result_map_err(err("oops"), fn(string s) -> string { return concat("ERR: ", s) })
dec string x = result_unwrap_err(r)
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("ERR: oops")));
}

#[test]
fn result_map_err_passes_through_ok() {
    let result = compile_and_run(
        r#"
get result_map_err, result_unwrap from std::res
dec result[int] r = result_map_err(ok(42), fn(string s) -> string { return s })
dec int x = result_unwrap(r)
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(42));
}

#[test]
fn function_returns_ok() {
    let result = compile_and_run(
        r#"
get is_ok, result_unwrap from std::res
fn safe_div(int a, int b) -> result[int] {
    if b == 0 {
        return err("division by zero")
    }
    return ok(a / b)
}
dec result[int] r = safe_div(10, 2)
dec bool ok_flag = is_ok(r)?
dec int val = result_unwrap(r)
(ok_flag, val)
"#,
    )
    .unwrap();
    assert_eq!(
        result,
        VmValue::Tuple(Rc::new(vec![VmValue::Bool(true), VmValue::Int(5)]))
    );
}

#[test]
fn function_returns_err() {
    let result = compile_and_run(
        r#"
get is_err, result_unwrap_err from std::res
fn safe_div(int a, int b) -> result[int] {
    if b == 0 {
        return err("division by zero")
    }
    return ok(a / b)
}
dec result[int] r = safe_div(10, 0)
dec bool err_flag = is_err(r)?
dec string msg = result_unwrap_err(r)
(err_flag, msg)
"#,
    )
    .unwrap();
    assert_eq!(
        result,
        VmValue::Tuple(Rc::new(vec![
            VmValue::Bool(true),
            VmValue::Str(Rc::from("division by zero")),
        ]))
    );
}

#[test]
fn chained_result_map() {
    let result = compile_and_run(
        r#"
get result_map, result_unwrap from std::res
fn safe_div(int a, int b) -> result[int] {
    if b == 0 {
        return err("division by zero")
    }
    return ok(a / b)
}
dec result[int] r = result_map(safe_div(20, 4), fn(int n) -> int { return n + 1 })
dec int x = result_unwrap(r)
x
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(6));
}

#[test]
fn err_carries_int() {
    let result = compile_and_run(
        r#"
get result_unwrap_err from std::res
dec result[int] r = err(404)
dec int code = result_unwrap_err(r)
code
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Int(404));
}

#[test]
fn err_carries_bool() {
    let result = compile_and_run(
        r#"
get result_unwrap_err from std::res
dec result[int] r = err(false)
dec bool b = result_unwrap_err(r)
b
"#,
    )
    .unwrap();
    assert_eq!(result, VmValue::Bool(false));
}
