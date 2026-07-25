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

    let mut c_args: Vec<CArg> = Vec::with_capacity(arg_values.len());
    for (i, (value, type_name)) in arg_values.into_iter().zip(&arg_type_names).enumerate() {
        match value_to_carg(value, type_name, i) {
            Ok(c_arg) => c_args.push(c_arg),
            Err(e) => return e,
        }
    }

    let mut arg_ffi_types: Vec<Type> = Vec::with_capacity(arg_type_names.len());
    for type_name in &arg_type_names {
        match parse_type(type_name, "arg") {
            Ok(t) => arg_ffi_types.push(t),
            Err(e) => return e,
        }
    }

    let ret_ffi_type = if ret_type == "void" {
        Type::void()
    } else {
        match parse_type(&ret_type, "return") {
            Ok(t) => t,
            Err(e) => return e,
        }
    };

    let CHandle::Library(lib) = match eval.c_handles.get(&handle_id) {
        Some(h) => h,
        None => return verr!(vs!(format!("call: unknown handle {}", handle_id))),
    };

    let ffi_args: Vec<Arg> = c_args
        .iter()
        .map(|c| match c {
            CArg::I32(v) => arg(v),
            CArg::I64(v) => arg(v),
            CArg::F32(v) => arg(v),
            CArg::F64(v) => arg(v),
        })
        .collect();

    let cif = Cif::new(arg_ffi_types, ret_ffi_type);

    // SAFETY: `cif`'s arg/return types come directly from `arg_types`/`ret_type`
    // as declared by the caller. If those don't match the real C function's
    // signature, this is UB - same contract as any FFI call. Getting the
    // symbol itself (`lib.get`) is also unsafe per `libloading`'s contract.
    let result = unsafe {
        let sym: Symbol<unsafe extern "C" fn()> = match lib.get(fn_name.as_bytes()) {
            Ok(s) => s,
            Err(e) => {
                return verr!(vs!(format!(
                    "call: symbol \"{}\" not found: {}",
                    fn_name, e
                )));
            }
        };
        let code_ptr = CodePtr::from_fun(*sym);

        match ret_type.as_str() {
            "void" => {
                let _: () = cif.call(code_ptr, &ffi_args);
                vok!(vnl!())
            }
            "i32" => vok!(vi!(cif.call::<i32>(code_ptr, &ffi_args) as i64)),
            "i64" => vok!(vi!(cif.call::<i64>(code_ptr, &ffi_args))),
            "f32" => vok!(vf!(cif.call::<f32>(code_ptr, &ffi_args) as f64)),
            "f64" => vok!(vf!(cif.call::<f64>(code_ptr, &ffi_args))),
            _ => unreachable!("ret_type validated by parse_type above"),
        }
    };

    result
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
