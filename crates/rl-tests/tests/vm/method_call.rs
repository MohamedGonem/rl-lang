use crate::common;
use rl_vm::VmValue;

#[test]
fn string_method_falls_back_to_stdlib() {
    let result = common::compile_and_run(
        r#"
        get repeat from std::str
        "ab".repeat(3)
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Str(std::rc::Rc::from("ababab")));
}

#[test]
fn chained_stdlib_method_calls() {
    let result = common::compile_and_run(
        r#"
        get repeat from std::str
        get to_upper from std::str
        "ab".repeat(3).to_upper()
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Str(std::rc::Rc::from("ABABAB")));
}

#[test]
fn user_function_method_fallback() {
    let result = common::compile_and_run(
        r#"
        fn double(int x) -> int {
            return x * 2
        }

        dec int x = 21
        x.double()
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Int(42));
}

#[test]
fn record_impl_method_takes_precedence_over_stdlib_fallback() {
    let result = common::compile_and_run(
        r#"
        get to_upper from std::str

        record Point {
            int x,
        }

        impl Point {
            fn to_upper(self) -> int {
                return 99
            }
        }

        dec Point p = Point { x: 0 }
        p.to_upper()
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Int(99));
}

#[test]
fn calling_unknown_method_on_a_primitive_is_a_runtime_error() {
    let err = common::compile_and_run(
        r#"
        dec int x = 5
        x.magnitude()
        "#,
    )
    .expect_err("expected a runtime error for an unknown method on a primitive");

    assert!(
        err.message().contains("magnitude"),
        "unexpected error message: {}",
        err.message()
    );
}

#[test]
fn calling_unknown_method_on_a_record_is_a_runtime_error() {
    let err = common::compile_and_run(
        r#"
        record Point {
            int x,
        }

        dec Point p = Point { x: 0 }
        p.unknown()
        "#,
    )
    .expect_err("expected a runtime error for an unknown record method");

    assert!(
        err.message().contains("unknown") && err.message().contains("Point"),
        "unexpected error message: {}",
        err.message()
    );
}

#[test]
fn namespaced_stdlib_method_call() {
    // `x.std::module::fn()` resolves the stdlib path without an `import`
    let result = common::compile_and_run(
        r#"
        dec string s = "Hello"
        s.std::str::to_lower()
        "#,
    )
    .expect("vm run failed");
    assert_eq!(result, VmValue::Str(std::rc::Rc::from("hello")));
}
