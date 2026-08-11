use rl_interpreter::values::Value;

use crate::common::eval_program;

#[test]
fn array_index_read() {
    let ev = eval_program(
        r#"
dec arr[int] array = [10, 20, 30]
dec int x = array[1]
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(20)));
}

#[test]
fn arr_index_of_found() {
    let ev = eval_program(
        r#"
get arr_index_of from std::array
dec arr[int] array = [10, 20, 30]
dec int x = arr_index_of(array, 20)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(1)));
}
