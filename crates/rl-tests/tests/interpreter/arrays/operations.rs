use {rl_ast::statements::TypeAnnotation, rl_interpreter::values::Value};

use crate::common::eval_program;

use crate::interpreter::arrays::common::int_array;


#[test]
fn arr_count() {
    let ev = eval_program(
        r#"
get arr_count from std::array
dec arr[int] array = [10, 20, 30]
dec int x = arr_count(array)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(3)));
}

#[test]
fn arr_is_empty_true() {
    let ev = eval_program(
        r#"
get arr_is_empty from std::array
dec arr[int] array = []
dec bool x = arr_is_empty(array)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn arr_is_empty_false() {
    let ev = eval_program(
        r#"
get arr_is_empty from std::array
dec arr[int] array = [1]
dec bool x = arr_is_empty(array)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(false)));
}

#[test]
fn arr_contains_true() {
    let ev = eval_program(
        r#"
get arr_contains from std::array
dec arr[int] array = [1, 2, 3]
dec bool x = arr_contains(array, 2)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn arr_contains_false() {
    let ev = eval_program(
        r#"
get arr_contains from std::array
dec arr[int] array = [1, 2, 3]
dec bool x = arr_contains(array, 99)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Bool(false)));
}

#[test]
fn arr_reverse() {
    let ev = eval_program(
        r#"
get arr_reverse from std::array
dec arr[int] x = [1, 2, 3]
x = arr_reverse(x)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(int_array(vec![3, 2, 1])));
}

#[test]
fn arr_sum() {
    let ev = eval_program(
        r#"
get arr_sum from std::array
dec arr[int] array = [1, 2, 3, 4]
dec int x = arr_sum(array)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(10)));
}

#[test]
fn arr_max() {
    let ev = eval_program(
        r#"
get arr_max from std::array
dec arr[int] array = [3, 1, 4, 1, 5, 9]
dec int x = arr_max(array)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(9)));
}

#[test]
fn arr_min() {
    let ev = eval_program(
        r#"
get arr_min from std::array
dec arr[int] array = [3, 1, 4, 1, 5, 9]
dec int x = arr_min(array)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(1)));
}

#[test]
fn arr_first() {
    let ev = eval_program(
        r#"
get arr_first from std::array
dec arr[int] array = [7, 8, 9]
dec int x = arr_first(array)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(7)));
}

#[test]
fn arr_last() {
    let ev = eval_program(
        r#"
get arr_last from std::array
dec arr[int] array = [7, 8, 9]
dec int x = arr_last(array)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(Value::Integer(9)));
}



#[test]
fn arr_concat() {
    let ev = eval_program(
        r#"
get arr_concat from std::array
dec arr[int] a = [1, 2]
dec arr[int] b = [3, 4]
dec arr[int] x = arr_concat(a, b)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(int_array(vec![1, 2, 3, 4])));
}

#[test]
fn arr_slice() {
    let ev = eval_program(
        r#"
get arr_slice from std::array
dec arr[int] array = [0, 1, 2, 3, 4]
dec arr[int] x = arr_slice(array, 1, 4)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(int_array(vec![1, 2, 3])));
}


#[test]
fn arr_map_doubles() {
    let ev = eval_program(
        r#"
get arr_map from std::array
dec arr[int] x = [1, 2, 3]
dec arr[int] y = arr_map(x, fn(int n) -> int { return n * 2 })?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("y"), Some(int_array(vec![2, 4, 6])));
}

#[test]
fn arr_filter_evens() {
    let ev = eval_program(
        r#"
get arr_filter, arr_count from std::array
dec arr[int] x = [1, 2, 3, 4, 5, 6]
dec arr[int] y = arr_filter(x, fn(int n) -> bool { return n > 3 })?
dec int c = arr_count(y)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("c"), Some(Value::Integer(3)));
}

#[test]
fn arr_reduce_sum() {
    let ev = eval_program(
        r#"
get arr_reduce from std::array
dec arr[int] x = [1, 2, 3, 4, 5]
dec int total = arr_reduce(x, fn(int acc, int n) -> int { return acc + n }, 0)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("total"), Some(Value::Integer(15)));
}

#[test]
fn arr_reduce_product() {
    let ev = eval_program(
        r#"
get arr_reduce from std::array
dec arr[int] x = [1, 2, 3, 4]
dec int product = arr_reduce(x, fn(int acc, int n) -> int { return acc * n }, 1)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("product"), Some(Value::Integer(24)));
}

#[test]
fn arr_sort_ascending() {
    let ev = eval_program(
        r#"
get arr_sort from std::array
dec arr[int] x = [3, 1, 4, 1, 5, 9, 2, 6]
x = arr_sort(x)?
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("x"),
        Some(int_array(vec![1, 1, 2, 3, 4, 5, 6, 9]))
    );
}

#[test]
fn arr_unique_deduplicates() {
    let ev = eval_program(
        r#"
get arr_unique, arr_count from std::array
dec arr[int] x = [1, 2, 2, 3, 3, 3]
dec arr[int] u = arr_unique(x)?
dec int c = arr_count(u)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("c"), Some(Value::Integer(3)));
}

#[test]
fn arr_flatten() {
    let ev = eval_program(
        r#"
get arr_flatten from std::array
dec arr[arr[int]] x = [[1, 2], [3, 4], [5]]
dec arr[int] y = arr_flatten(x)?
"#,
    )
    .unwrap();
    assert_eq!(
        ev.get_value_raw("y"),
        Some(Value::Values {
            items_type: TypeAnnotation::Array(Box::new(TypeAnnotation::Int)),
            items: vec![
                Value::Integer(1),
                Value::Integer(2),
                Value::Integer(3),
                Value::Integer(4),
                Value::Integer(5),
            ],
        })
    );
}

#[test]
fn arr_find_returns_value() {
    let ev = eval_program(
        r#"
get arr_find from std::array
dec arr[int] x = [10, 20, 30]
dec int found = arr_find(x, fn(int n) -> bool { return n > 15 })?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("found"), Some(Value::Integer(20)));
}

#[test]
fn arr_any_true() {
    let ev = eval_program(
        r#"
get arr_any from std::array
dec arr[int] x = [1, 2, 3]
dec bool b = arr_any(x, fn(int n) -> bool { return n > 2 })?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("b"), Some(Value::Bool(true)));
}

#[test]
fn arr_any_false() {
    let ev = eval_program(
        r#"
get arr_any from std::array
dec arr[int] x = [1, 2, 3]
dec bool b = arr_any(x, fn(int n) -> bool { return n > 10 })?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("b"), Some(Value::Bool(false)));
}

#[test]
fn arr_all_true() {
    let ev = eval_program(
        r#"
get arr_all from std::array
dec arr[int] x = [2, 4, 6]
dec bool b = arr_all(x, fn(int n) -> bool { return n > 0 })?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("b"), Some(Value::Bool(true)));
}

#[test]
fn arr_all_false() {
    let ev = eval_program(
        r#"
get arr_all from std::array
dec arr[int] x = [2, 4, 6]
dec bool b = arr_all(x, fn(int n) -> bool { return n > 3 })?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("b"), Some(Value::Bool(false)));
}

#[test]
fn arr_fill() {
    let ev = eval_program(
        r#"
get arr_fill from std::array
dec arr[int] x = arr_fill(0, 5)
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(int_array(vec![0, 0, 0, 0, 0])));
}

#[test]
fn arr_range() {
    let ev = eval_program(
        r#"
get arr_range from std::array
dec arr[int] x = arr_range(0, 5, 1)?
"#,
    )
    .unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(int_array(vec![0, 1, 2, 3, 4])));
}
