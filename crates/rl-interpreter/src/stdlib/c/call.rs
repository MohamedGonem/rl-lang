use libffi::middle::{Arg, Cif, CodePtr, Type, arg};
use libloading::Symbol;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        c::CHandle,
        common::{extract_number, extract_string, verr, vf, vi, vnl, vok, vs},
    },
    values::Value,
};

enum CArg {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

pub fn func(
    eval: &mut Evaluator,
    handle: Value,
    fn_name: Value,
    args: Value,
    arg_types: Value,
    ret_type: Value,
) -> Value {
    let handle_id = match extract_number(handle, "call") {
        Ok(n) => n as i64,
        Err(e) => return verr!(vs!(format!("call: {}", e))),
    };
    let fn_name = match extract_string(fn_name, "call") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("call: {}", e))),
    };
    let ret_type = match extract_string(ret_type, "call") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("call: {}", e))),
    };

    let arg_type_names: Vec<String> = match arg_types {
        Value::Values { items, .. } => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                match item {
                    Value::String(s) => out.push(s),
                    other => {
                        return verr!(vs!(format!(
                            "call: arg_types must be an array of string, found {}",
                            other.type_name()
                        )));
                    }
                }
            }
            out
        }
        other => {
            return verr!(vs!(format!(
                "call: expected an array of string for arg_types, found {}",
                other.type_name()
            )));
        }
    };

    let arg_values: Vec<Value> = match args {
        Value::Values { items, .. } => items,
        other => {
            return verr!(vs!(format!(
                "call: expected an array of args, found {}",
                other.type_name()
            )));
        }
    };

    if arg_values.len() != arg_type_names.len() {
        return verr!(vs!(format!(
            "call: {} arg(s) but {} arg_type(s) given",
            arg_values.len(),
            arg_type_names.len()
        )));
    }

    let CHandle::Library(lib) = match eval.c_handles.get(&handle_id) {
        Some(h) => h,
        None => return verr!(vs!(format!("call: unknown handle {}", handle_id))),
    };

    // SAFETY: the caller-declared signature (all-i64, fixed arity) must match
    // the real C function's signature. Getting this wrong is UB - same
    // contract as any FFI call.
    let result = unsafe {
        match args.len() {
            0 => {
                let sym: Symbol<'_, unsafe extern "C" fn() -> i64> =
                    match lib.get(fn_name.as_bytes()) {
                        Ok(s) => s,
                        Err(e) => return sym_err(&fn_name, e),
                    };
                sym()
            }
            1 => {
                let sym: Symbol<'_, unsafe extern "C" fn(i64) -> i64> =
                    match lib.get(fn_name.as_bytes()) {
                        Ok(s) => s,
                        Err(e) => return sym_err(&fn_name, e),
                    };
                sym(args[0])
            }
            2 => {
                let sym: Symbol<'_, unsafe extern "C" fn(i64, i64) -> i64> =
                    match lib.get(fn_name.as_bytes()) {
                        Ok(s) => s,
                        Err(e) => return sym_err(&fn_name, e),
                    };
                sym(args[0], args[1])
            }
            3 => {
                let sym: Symbol<'_, unsafe extern "C" fn(i64, i64, i64) -> i64> =
                    match lib.get(fn_name.as_bytes()) {
                        Ok(s) => s,
                        Err(e) => return sym_err(&fn_name, e),
                    };
                sym(args[0], args[1], args[2])
            }
            4 => {
                let sym: Symbol<'_, unsafe extern "C" fn(i64, i64, i64, i64) -> i64> =
                    match lib.get(fn_name.as_bytes()) {
                        Ok(s) => s,
                        Err(e) => return sym_err(&fn_name, e),
                    };
                sym(args[0], args[1], args[2], args[3])
            }
            5 => {
                let sym: Symbol<'_, unsafe extern "C" fn(i64, i64, i64, i64, i64) -> i64> =
                    match lib.get(fn_name.as_bytes()) {
                        Ok(s) => s,
                        Err(e) => return sym_err(&fn_name, e),
                    };
                sym(args[0], args[1], args[2], args[3], args[4])
            }
            6 => {
                let sym: Symbol<'_, unsafe extern "C" fn(i64, i64, i64, i64, i64, i64) -> i64> =
                    match lib.get(fn_name.as_bytes()) {
                        Ok(s) => s,
                        Err(e) => return sym_err(&fn_name, e),
                    };
                sym(args[0], args[1], args[2], args[3], args[4], args[5])
            }
            _ => unreachable!("checked above"),
        }
    };

    vok!(vi!(result))
}

fn sym_err(fn_name: &str, e: libloading::Error) -> Value {
    verr!(vs!(format!(
        "call: symbol \"{}\" not found: {}",
        fn_name, e
    )))
}

fn parse_type(name: &str, context: &str) -> Result<Type, Value> {
    match name {
        "i32" => Ok(Type::i32()),
        "i64" => Ok(Type::i64()),
        "f32" => Ok(Type::f32()),
        "f64" => Ok(Type::f64()),
        other => Err(verr!(vs!(format!(
            "call: unsupported {} type \"{}\" (supported: i32, i64, f32, f64)",
            context, other
        )))),
    }
}

fn value_to_carg(value: Value, type_name: &str, index: usize) -> Result<CArg, Value> {
    match (type_name, &value) {
        ("i32", Value::Integer(i)) => Ok(CArg::I32(*i as i32)),
        ("i32", Value::Byte(b)) => Ok(CArg::I32(*b as i32)),
        ("i64", Value::Integer(i)) => Ok(CArg::I64(*i)),
        ("i64", Value::Byte(b)) => Ok(CArg::I64(*b as i64)),
        ("f32", Value::Float(f)) => Ok(CArg::F32(*f as f32)),
        ("f64", Value::Float(f)) => Ok(CArg::F64(*f)),
        (t, other) => Err(verr!(vs!(format!(
            "call: arg {} declared as \"{}\" but got {}",
            index,
            t,
            other.type_name()
        )))),
    }
}
