use std::rc::Rc;

use rl_vm::VmValue;

use crate::common::compile_and_run;

fn temp_path(name: &str) -> std::path::PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("rl-vm-io-{}-{}", std::process::id(), name));
    path
}

#[test]
fn read_file_returns_contents() {
    let path = temp_path("read_file.txt");
    std::fs::write(&path, "hello from rl").unwrap();

    let path_str = path.to_str().unwrap();
    let result = compile_and_run(&format!(
        r#"
get read_file from std::io
get result_unwrap from std::res
dec result[string] r = read_file("{path_str}")
dec string x = result_unwrap(r)
x
"#
    ))
    .unwrap();
    assert_eq!(result, VmValue::Str(Rc::from("hello from rl")));

    let _ = std::fs::remove_file(&path);
}

#[test]
fn read_file_missing_returns_err() {
    let path = temp_path("does-not-exist.txt");
    let path_str = path.to_str().unwrap();
    let result = compile_and_run(&format!(
        r#"
get read_file from std::io
get is_err from std::res
dec result[string] r = read_file("{path_str}")
dec bool x = is_err(r)?
x
"#
    ))
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn read_lines_splits_by_line() {
    let path = temp_path("read_lines.txt");
    std::fs::write(&path, "one\ntwo\nthree").unwrap();

    let path_str = path.to_str().unwrap();
    let result = compile_and_run(&format!(
        r#"
get read_lines from std::io
get result_unwrap from std::res
dec result[arr[string]] r = read_lines("{path_str}")
dec arr[string] lines = result_unwrap(r)
lines
"#
    ))
    .unwrap();
    assert_eq!(
        result,
        VmValue::Arr(Rc::new(vec![
            VmValue::Str(Rc::from("one")),
            VmValue::Str(Rc::from("two")),
            VmValue::Str(Rc::from("three")),
        ]))
    );

    let _ = std::fs::remove_file(&path);
}

#[test]
fn read_bytes_returns_bytes() {
    let path = temp_path("read_bytes.bin");
    std::fs::write(&path, [0x01, 0x02, 0xff]).unwrap();

    let path_str = path.to_str().unwrap();
    let result = compile_and_run(&format!(
        r#"
get read_bytes from std::io
get result_unwrap from std::res
dec result[arr[byte]] r = read_bytes("{path_str}")
dec arr[byte] bytes = result_unwrap(r)
bytes
"#
    ))
    .unwrap();
    assert_eq!(
        result,
        VmValue::Arr(Rc::new(vec![
            VmValue::Byte(0x01),
            VmValue::Byte(0x02),
            VmValue::Byte(0xff),
        ]))
    );

    let _ = std::fs::remove_file(&path);
}

#[test]
fn write_file_writes_content() {
    let path = temp_path("write_file.txt");
    let path_str = path.to_str().unwrap();
    let result = compile_and_run(&format!(
        r#"
get write_file from std::io
get is_ok from std::res
dec result[string] r = write_file("{path_str}", "written")
dec bool x = is_ok(r)?
x
"#
    ))
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "written");

    let _ = std::fs::remove_file(&path);
}

#[test]
fn append_file_appends_content() {
    let path = temp_path("append_file.txt");
    std::fs::write(&path, "first").unwrap();
    let path_str = path.to_str().unwrap();
    let result = compile_and_run(&format!(
        r#"
get append_file from std::io
get is_ok from std::res
dec result[string] r = append_file("{path_str}", "-second")
dec bool x = is_ok(r)?
x
"#
    ))
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "first-second");

    let _ = std::fs::remove_file(&path);
}

#[test]
fn delete_file_removes_file() {
    let path = temp_path("delete_file.txt");
    std::fs::write(&path, "bye").unwrap();
    let path_str = path.to_str().unwrap();
    let result = compile_and_run(&format!(
        r#"
get delete_file from std::io
get is_ok from std::res
dec result[string] r = delete_file("{path_str}")
dec bool x = is_ok(r)?
x
"#
    ))
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
    assert!(!path.exists());
}

#[test]
fn delete_file_missing_returns_err() {
    let path = temp_path("never-created.txt");
    let path_str = path.to_str().unwrap();
    let result = compile_and_run(&format!(
        r#"
get delete_file from std::io
get is_err from std::res
dec result[string] r = delete_file("{path_str}")
dec bool x = is_err(r)?
x
"#
    ))
    .unwrap();
    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn read_rejects_too_many_args() {
    let result = compile_and_run(
        r#"
get read from std::io
get result_unwrap_err from std::res
dec result[string] r = read("a", "b")
dec string msg = result_unwrap_err(r)
msg
"#,
    )
    .unwrap();
    assert_eq!(
        result,
        VmValue::Str(Rc::from("read: expects 0 or 1 argument(s), got 2"))
    );
}

#[test]
fn read_int_rejects_too_many_args() {
    let result = compile_and_run(
        r#"
get read_int from std::io
get result_unwrap_err from std::res
dec result[int] r = read_int("a", "b")
dec string msg = result_unwrap_err(r)
msg
"#,
    )
    .unwrap();
    assert_eq!(
        result,
        VmValue::Str(Rc::from("read_int: expects 0 or 1 argument(s), got 2"))
    );
}

#[test]
fn read_float_rejects_too_many_args() {
    let result = compile_and_run(
        r#"
get read_float from std::io
get result_unwrap_err from std::res
dec result[float] r = read_float("a", "b")
dec string msg = result_unwrap_err(r)
msg
"#,
    )
    .unwrap();
    assert_eq!(
        result,
        VmValue::Str(Rc::from("read_float: expects 0 or 1 argument(s), got 2"))
    );
}
