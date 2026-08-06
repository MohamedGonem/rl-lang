use rl_interpreter::values::Value;

use crate::common::eval_program;

use crate::interpreter::arrays::common::int_array;

#[test]
fn for_in_array_sums_elements() {
    let ev = eval_program(
        r#"
dec arr[int] array = [1, 2, 3, 4, 5]
dec int total = 0
for item in array {
    total += item
}
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("total"), Some(Value::Integer(15)));
}

#[test]
fn for_range_builds_array() {
    let ev = eval_program(
        r#"
get arr_push from std::array
dec arr[int] x = []
for i in 0..3 {
    x = arr_push(x, i)?
}
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(int_array(vec![0, 1, 2])));
}