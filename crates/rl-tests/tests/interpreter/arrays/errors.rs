use crate::common::eval_program;

#[test]
fn array_type_mismatch_is_error() {
    assert!(eval_program(r#"dec arr[int] x = ["not an int"]"#).is_err());
}


#[test]
fn array_out_of_bounds_is_error() {
    assert!(
        eval_program(
            r#"
dec arr[int] array = [1, 2]
dec int x = array[5]
"#
        )
        .is_err()
    );
}