use libffi::middle::{Arg, Cif, CodePtr, Type, arg};
use libloading::Symbol;
use rl_ast::statements::TypeAnnotation;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        c::CHandle,
        common::{extract_number, extract_string, verr, vf, vi, vnl, vok, vs},
    },
    values::Value,
};

/// One argument's value, converted from its rl-lang `Value` into the exact
/// Rust type its declared C type requires. Kept as an owned value (rather
/// than immediately building an `Arg`) so it has somewhere to live while the
/// `Arg`s that borrow from it are assembled and passed to `Cif::call`.
enum CArg {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Str { buf: Vec<u8>, ptr: *mut u8 },
}

enum ArgKind {
    Num(&'static str),
    Str(usize),
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
        Value::Tuple(items) => items,
        other => {
            return verr!(vs!(format!(
                "call: expected a tuple of args (e.g. (\"hello\", 16)), found {}",
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

    let mut arg_kinds: Vec<ArgKind> = Vec::with_capacity(arg_type_names.len());
    for type_name in &arg_type_names {
        match parse_arg_kind(type_name) {
            Ok(k) => arg_kinds.push(k),
            Err(e) => return e,
        }
    }

    let ret_ffi_type = match parse_ret_type(&ret_type) {
        Ok(t) => t,
        Err(e) => return e,
    };

    let mut c_args: Vec<CArg> = Vec::with_capacity(arg_values.len());
    for (i, (value, kind)) in arg_values.into_iter().zip(&arg_kinds).enumerate() {
        match value_to_carg(value, kind, i) {
            Ok(c_arg) => c_args.push(c_arg),
            Err(e) => return e,
        }
    }

    let arg_ffi_types: Vec<Type> = arg_kinds
        .iter()
        .map(|k| match k {
            ArgKind::Num(name) => numeric_ffi_type(name),
            ArgKind::Str(_) => Type::pointer(),
        })
        .collect();

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
            CArg::Str { ptr, .. } => arg(ptr),
        })
        .collect();

    let cif = Cif::new(arg_ffi_types, ret_ffi_type);

    // SAFETY: `cif`'s arg/return types come directly from `arg_types`/`ret_type`
    // as declared by the caller. If those don't match the real C function's
    // signature, this is UB - same contract as any FFI call. For `str` args,
    // the buffer is sized to the declared capacity and C is trusted to
    // respect it (there's no way to enforce that from the caller's side -
    // same as passing any buffer+length pair to C in C itself). Getting the
    // symbol (`lib.get`) is also unsafe per `libloading`'s contract.
    let ret_value: Value = unsafe {
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
                vnl!()
            }
            "i32" => vi!(cif.call::<i32>(code_ptr, &ffi_args) as i64),
            "i64" => vi!(cif.call::<i64>(code_ptr, &ffi_args)),
            "f32" => vf!(cif.call::<f32>(code_ptr, &ffi_args) as f64),
            "f64" => vf!(cif.call::<f64>(code_ptr, &ffi_args)),
            _ => unreachable!("ret_type validated by parse_ret_type above"),
        }
    };

    // Read back any `str` buffers C may have written into, in the order
    // their args appeared`.
    let mutated_strs: Vec<Value> = c_args
        .iter()
        .filter_map(|c| match c {
            CArg::Str { buf, .. } => {
                let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
                Some(Value::String(
                    String::from_utf8_lossy(&buf[..end]).into_owned(),
                ))
            }
            _ => None,
        })
        .collect();

    if mutated_strs.is_empty() {
        vok!(ret_value)
    } else {
        vok!(Value::Tuple(vec![
            ret_value,
            Value::Values {
                items_type: TypeAnnotation::String,
                items: mutated_strs,
            },
        ]))
    }
}

fn parse_arg_kind(name: &str) -> Result<ArgKind, Value> {
    match name {
        "i32" | "i64" | "f32" | "f64" => Ok(ArgKind::Num(match name {
            "i32" => "i32",
            "i64" => "i64",
            "f32" => "f32",
            _ => "f64",
        })),
        _ if name.starts_with("str:") => {
            let cap: usize = name["str:".len()..].parse().map_err(|_| {
                verr!(vs!(format!(
                    "call: invalid str capacity in \"{}\" (expected e.g. \"str:64\")",
                    name
                )))
            })?;
            if cap == 0 {
                return Err(verr!(vs!(format!(
                    "call: str capacity in \"{}\" must be at least 1",
                    name
                ))));
            }
            Ok(ArgKind::Str(cap))
        }
        "str" => Err(verr!(vs!(
            "call: \"str\" needs an explicit capacity, e.g. \"str:64\" - \
             there's no safe default to write past"
                .to_string()
        ))),
        other => Err(verr!(vs!(format!(
            "call: unsupported arg type \"{}\" (supported: i32, i64, f32, f64, str:N)",
            other
        )))),
    }
}

fn numeric_ffi_type(name: &str) -> Type {
    match name {
        "i32" => Type::i32(),
        "i64" => Type::i64(),
        "f32" => Type::f32(),
        _ => Type::f64(),
    }
}

fn parse_ret_type(name: &str) -> Result<Type, Value> {
    match name {
        "void" => Ok(Type::void()),
        "i32" | "i64" | "f32" | "f64" => Ok(numeric_ffi_type(name)),
        other => Err(verr!(vs!(format!(
            "call: unsupported return type \"{}\" (supported: i32, i64, f32, f64, void - \
             no \"str\" return: an unmanaged C string's ownership can't be known safely)",
            other
        )))),
    }
}

fn value_to_carg(value: Value, kind: &ArgKind, index: usize) -> Result<CArg, Value> {
    match (kind, value) {
        (ArgKind::Num("i32"), Value::Integer(i)) => Ok(CArg::I32(i as i32)),
        (ArgKind::Num("i32"), Value::Byte(b)) => Ok(CArg::I32(b as i32)),
        (ArgKind::Num("i64"), Value::Integer(i)) => Ok(CArg::I64(i)),
        (ArgKind::Num("i64"), Value::Byte(b)) => Ok(CArg::I64(b as i64)),
        (ArgKind::Num("f32"), Value::Float(f)) => Ok(CArg::F32(f as f32)),
        (ArgKind::Num("f64"), Value::Float(f)) => Ok(CArg::F64(f)),
        (ArgKind::Num(t), other) => Err(verr!(vs!(format!(
            "call: arg {} declared as \"{}\" but got {}",
            index,
            t,
            other.type_name()
        )))),
        (ArgKind::Str(cap), Value::String(s)) => {
            let cap = *cap;
            if s.len() >= cap {
                return Err(verr!(vs!(format!(
                    "call: arg {} is {} byte(s) but its str buffer capacity is only {} \
                     (need room for a null terminator - raise the \"str:N\")",
                    index,
                    s.len(),
                    cap
                ))));
            }
            let mut buf = vec![0u8; cap];
            buf[..s.len()].copy_from_slice(s.as_bytes());
            let ptr = buf.as_mut_ptr();
            Ok(CArg::Str { buf, ptr })
        }
        (ArgKind::Str(_), other) => Err(verr!(vs!(format!(
            "call: arg {} declared as str but got {}",
            index,
            other.type_name()
        )))),
    }
}
