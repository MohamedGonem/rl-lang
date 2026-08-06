use {rl_ast::statements::TypeAnnotation, rl_interpreter::values::Value};

use crate::common::eval_program;

use crate::interpreter::arrays::common::{int_array, string_array};

#[test]
fn dec_int_array() {
    let ev = eval_program("dec arr[int] x = [1, 2, 3]").unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(int_array(vec![1, 2, 3])));
}

#[test]
fn dec_string_array() {
    let ev = eval_program(r#"dec arr[string] x = ["a", "b"]"#).unwrap();
    assert_eq!(ev.get_value_raw("x"), Some(string_array(vec!["a", "b"])));
}

#[test]
fn empty_array() {
    let ev = eval_program("dec arr[int] x = []").unwrap();
    assert_eq!(
        ev.get_value_raw("x"),
        Some(Value::Values {
            items_type: TypeAnnotation::Int,
            items: vec![],
        })
    );
}
