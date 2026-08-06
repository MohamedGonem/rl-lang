use rl_interpreter::values::Value;

use crate::common::eval_program;

use crate::interpreter::arrays::common::int_array;


#[test]
fn arr_push() {
    let ev = eval_program(
        r#"
get arr_push from std::array
dec arr[int] x = [1, 2]
x = arr_push(x, 3)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(int_array(vec![1, 2, 3])));
}

#[test]
fn arr_pop() {
    let ev = eval_program(
        r#"
get arr_pop from std::array
dec arr[int] x = [1, 2, 3]
x = arr_pop(x)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(int_array(vec![1, 2])));
}

#[test]
fn arr_insert() {
    let ev = eval_program(
        r#"
get arr_insert from std::array
dec arr[int] x = [1, 2, 4]
x = arr_insert(x, 3, 2)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(int_array(vec![1, 2, 3, 4])));
}

#[test]
fn arr_remove() {
    let ev = eval_program(
        r#"
get arr_remove from std::array
dec arr[int] x = [1, 2, 3, 4]
x = arr_remove(x, 1)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(int_array(vec![1, 3, 4])));
}

#[test]
fn array_index_write() {
    let ev = eval_program(
        r#"
dec arr[int] array = [1, 2, 3]
array[0] = 99
dec int x = array[0]
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(99)));
}