use libloading::Symbol;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        c::CHandle,
        common::{extract_number, extract_string, verr, vi, vok, vs},
    },
    values::Value,
};

const MAX_ARGS: usize = 6;

pub fn func(eval: &mut Evaluator, handle: Value, fn_name: Value, args: Value) -> Value {
    let handle_id = match extract_number(handle, "call") {
        Ok(n) => n as i64,
        Err(e) => return verr!(vs!(format!("call: {}", e))),
    };
    let fn_name = match extract_string(fn_name, "call") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("call: {}", e))),
    };
    let args: Vec<i64> = match args {
        Value::Values { items, .. } => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                match item {
                    Value::Integer(i) => out.push(i),
                    Value::Byte(b) => out.push(b as i64),
                    other => {
                        return verr!(vs!(format!(
                            "call: all args must be int, found {}",
                            other.type_name()
                        )));
                    }
                }
            }
            out
        }
        other => {
            return verr!(vs!(format!(
                "call: expected an array of int args, found {}",
                other.type_name()
            )));
        }
    };
    if args.len() > MAX_ARGS {
        return verr!(vs!(format!(
            "call: at most {} args are supported in this version, got {}",
            MAX_ARGS,
            args.len()
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
