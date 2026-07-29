use libffi::middle::{Arg, Cif, CodePtr, Type, arg};
use libloading::Symbol;
use rl_ast::statements::HandleKind;
use std::rc::Rc;

use crate::{
    Vm,
    stdlib::{
        c::CHandle,
        common::{extract_handle, extract_string},
        macros::{vb, vby, verr, vf, vi, vnl, vok, vs},
    },
    values::VmValue,
};

/// The element type of an `"arr:TYPE:N"` arg.
#[derive(Clone, Copy)]
enum ArrElem {
    I32,
    I64,
    F32,
    F64,
    U8,
    I16,
}

impl ArrElem {
    fn parse(name: &str) -> Option<Self> {
        Some(match name {
            "i32" => ArrElem::I32,
            "i64" => ArrElem::I64,
            "f32" => ArrElem::F32,
            "f64" => ArrElem::F64,
            "u8" => ArrElem::U8,
            "i16" => ArrElem::I16,
            _ => return None,
        })
    }

    fn name(self) -> &'static str {
        match self {
            ArrElem::I32 => "i32",
            ArrElem::I64 => "i64",
            ArrElem::F32 => "f32",
            ArrElem::F64 => "f64",
            ArrElem::U8 => "u8",
            ArrElem::I16 => "i16",
        }
    }
}

/// One argument's value, converted from its rl-lang `VmValue` into the exact
/// Rust representation its declared C type requires. Kept as an owned value
/// (rather than immediately building an `Arg`) so it has somewhere to live
/// while the `Arg`s that borrow from it are assembled and passed to `Cif::call`.
enum CArg {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Bool(u8),
    U8(u8),
    I16(i16),
    Str { buf: Vec<u8>, ptr: *mut u8 },
    ArrI32 { buf: Vec<i32>, ptr: *mut u8 },
    ArrI64 { buf: Vec<i64>, ptr: *mut u8 },
    ArrF32 { buf: Vec<f32>, ptr: *mut u8 },
    ArrF64 { buf: Vec<f64>, ptr: *mut u8 },
    ArrU8 { buf: Vec<u8>, ptr: *mut u8 },
    ArrI16 { buf: Vec<i16>, ptr: *mut u8 },
}

/// A parsed `arg_types` entry.
enum ArgKind {
    Num(&'static str),
    Str(usize),
    Arr(ArrElem, usize),
}

pub fn std_call(
    vm: &mut Vm,
    handle: VmValue,
    fn_name: VmValue,
    args: VmValue,
    arg_types: VmValue,
    ret_type: VmValue,
) -> VmValue {
    let handle_id = match extract_handle(handle, HandleKind::C, "call") {
        Ok(id) => id,
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
        VmValue::Arr(items) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items.iter() {
                match item {
                    VmValue::Str(s) => out.push(s.to_string()),
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

    let arg_values: Vec<VmValue> = match args {
        VmValue::Tuple(items) => (*items).clone(),
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
            ArgKind::Str(_) | ArgKind::Arr(_, _) => Type::pointer(),
        })
        .collect();

    let CHandle::Library(lib) = match vm.c_handles.get(&handle_id) {
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
            CArg::Bool(v) => arg(v),
            CArg::U8(v) => arg(v),
            CArg::I16(v) => arg(v),
            CArg::Str { ptr, .. } => arg(ptr),
            CArg::ArrI32 { ptr, .. } => arg(ptr),
            CArg::ArrI64 { ptr, .. } => arg(ptr),
            CArg::ArrF32 { ptr, .. } => arg(ptr),
            CArg::ArrF64 { ptr, .. } => arg(ptr),
            CArg::ArrU8 { ptr, .. } => arg(ptr),
            CArg::ArrI16 { ptr, .. } => arg(ptr),
        })
        .collect();

    let cif = Cif::new(arg_ffi_types, ret_ffi_type);

    // SAFETY: `cif`'s arg/return types come directly from `arg_types`/`ret_type`
    // as declared by the caller. If those don't match the real C function's
    // signature, this is UB - same contract as any FFI call. For `str`/`arr`
    // args, the buffer is sized to the declared capacity/count and C is
    // trusted to respect it (there's no way to enforce that from the
    // caller's side - same as passing any buffer+length pair to C in C
    // itself). Getting the symbol (`lib.get`) is also unsafe per
    // `libloading`'s contract.
    let ret_value: VmValue = unsafe {
        let sym: Symbol<'_, unsafe extern "C" fn()> = match lib.get(fn_name.as_bytes()) {
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
            "bool" => vb!(cif.call::<u8>(code_ptr, &ffi_args) != 0),
            "u8" => vby!(cif.call::<u8>(code_ptr, &ffi_args)),
            "i16" => vi!(cif.call::<i16>(code_ptr, &ffi_args) as i64),
            _ => unreachable!("ret_type validated by parse_ret_type above"),
        }
    };

    // Read back any `str`/`arr` buffers C may have written into, in the
    // order their args appeared - this is the only "mutation" that crosses
    // back into rl-lang, and only because the caller explicitly opted an
    // arg into it via `"str:N"`/`"arr:TYPE:N"`.
    let mutated_outs: Vec<VmValue> = c_args
        .into_iter()
        .filter_map(|c| match c {
            CArg::Str { buf, .. } => {
                let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
                Some(vs!(String::from_utf8_lossy(&buf[..end]).into_owned()))
            }
            CArg::ArrI32 { buf, .. } => Some(VmValue::Arr(Rc::new(
                buf.into_iter().map(|v| VmValue::Int(v as i64)).collect(),
            ))),
            CArg::ArrI64 { buf, .. } => Some(VmValue::Arr(Rc::new(
                buf.into_iter().map(VmValue::Int).collect(),
            ))),
            CArg::ArrI16 { buf, .. } => Some(VmValue::Arr(Rc::new(
                buf.into_iter().map(|v| VmValue::Int(v as i64)).collect(),
            ))),
            CArg::ArrU8 { buf, .. } => Some(VmValue::Arr(Rc::new(
                buf.into_iter().map(VmValue::Byte).collect(),
            ))),
            CArg::ArrF32 { buf, .. } => Some(VmValue::Arr(Rc::new(
                buf.into_iter().map(|v| VmValue::Float(v as f64)).collect(),
            ))),
            CArg::ArrF64 { buf, .. } => Some(VmValue::Arr(Rc::new(
                buf.into_iter().map(VmValue::Float).collect(),
            ))),
            _ => None,
        })
        .collect();

    if mutated_outs.is_empty() {
        vok!(ret_value)
    } else {
        vok!(VmValue::Tuple(Rc::new(vec![
            ret_value,
            VmValue::Tuple(Rc::new(mutated_outs)),
        ])))
    }
}

fn parse_arg_kind(name: &str) -> Result<ArgKind, VmValue> {
    match name {
        "i32" | "i64" | "f32" | "f64" | "bool" | "u8" | "i16" => Ok(ArgKind::Num(match name {
            "i32" => "i32",
            "i64" => "i64",
            "f32" => "f32",
            "f64" => "f64",
            "bool" => "bool",
            "u8" => "u8",
            _ => "i16",
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
        _ if name.starts_with("arr:") => {
            let rest = &name["arr:".len()..];
            let (elem_name, count_str) = rest.split_once(':').ok_or_else(|| {
                verr!(vs!(format!(
                    "call: invalid arr type \"{}\" (expected e.g. \"arr:i32:8\")",
                    name
                )))
            })?;
            let elem = ArrElem::parse(elem_name).ok_or_else(|| {
                verr!(vs!(format!(
                    "call: unsupported arr element type \"{}\" in \"{}\" \
                     (supported: i32, i64, f32, f64, u8, i16)",
                    elem_name, name
                )))
            })?;
            let count: usize = count_str.parse().map_err(|_| {
                verr!(vs!(format!(
                    "call: invalid arr element count in \"{}\" (expected e.g. \"arr:i32:8\")",
                    name
                )))
            })?;
            if count == 0 {
                return Err(verr!(vs!(format!(
                    "call: arr element count in \"{}\" must be at least 1",
                    name
                ))));
            }
            Ok(ArgKind::Arr(elem, count))
        }
        other => Err(verr!(vs!(format!(
            "call: unsupported arg type \"{}\" \
             (supported: i32, i64, i16, u8, f32, f64, bool, str:N, arr:TYPE:N)",
            other
        )))),
    }
}

fn numeric_ffi_type(name: &str) -> Type {
    match name {
        "i32" => Type::i32(),
        "i64" => Type::i64(),
        "f32" => Type::f32(),
        "f64" => Type::f64(),
        "i16" => Type::i16(),
        "u8" | "bool" => Type::u8(),
        other => unreachable!(
            "numeric_ffi_type called with unvalidated name \"{}\"",
            other
        ),
    }
}

fn parse_ret_type(name: &str) -> Result<Type, VmValue> {
    match name {
        "void" => Ok(Type::void()),
        "i32" | "i64" | "i16" | "u8" | "f32" | "f64" | "bool" => Ok(numeric_ffi_type(name)),
        other => Err(verr!(vs!(format!(
            "call: unsupported return type \"{}\" (supported: i32, i64, i16, u8, f32, f64, bool, \
             void - no \"str\"/\"arr\" return: unmanaged C memory's ownership can't be known safely)",
            other
        )))),
    }
}

fn arr_value_to_carg(
    value: VmValue,
    elem: ArrElem,
    count: usize,
    index: usize,
) -> Result<CArg, VmValue> {
    let items: Vec<VmValue> = match value {
        VmValue::Arr(items) => (*items).clone(),
        other => {
            return Err(verr!(vs!(format!(
                "call: arg {} declared as arr:{}:{} but got {}",
                index,
                elem.name(),
                count,
                other.type_name()
            ))));
        }
    };
    if items.len() != count {
        return Err(verr!(vs!(format!(
            "call: arg {} is arr:{}:{} but the array has {} element(s), not {}",
            index,
            elem.name(),
            count,
            items.len(),
            count
        ))));
    }

    macro_rules! build_int_buf {
        ($variant:ident, $ty:ty, $range:expr) => {{
            let mut buf: Vec<$ty> = Vec::with_capacity(count);
            for (i, item) in items.into_iter().enumerate() {
                let v: i64 = match item {
                    VmValue::Int(n) => n,
                    VmValue::Byte(b) => b as i64,
                    other => {
                        return Err(verr!(vs!(format!(
                            "call: arg {} element {} declared as {} but got {}",
                            index,
                            i,
                            elem.name(),
                            other.type_name()
                        ))));
                    }
                };
                if !$range.contains(&v) {
                    return Err(verr!(vs!(format!(
                        "call: arg {} element {} is {} but \"{}\" only holds {:?}",
                        index,
                        i,
                        v,
                        elem.name(),
                        $range
                    ))));
                }
                buf.push(v as $ty);
            }
            let ptr = buf.as_mut_ptr() as *mut u8;
            Ok(CArg::$variant { buf, ptr })
        }};
    }

    macro_rules! build_float_buf {
        ($variant:ident, $ty:ty) => {{
            let mut buf: Vec<$ty> = Vec::with_capacity(count);
            for (i, item) in items.into_iter().enumerate() {
                match item {
                    VmValue::Float(f) => buf.push(f as $ty),
                    other => {
                        return Err(verr!(vs!(format!(
                            "call: arg {} element {} declared as {} but got {}",
                            index,
                            i,
                            elem.name(),
                            other.type_name()
                        ))));
                    }
                }
            }
            let ptr = buf.as_mut_ptr() as *mut u8;
            Ok(CArg::$variant { buf, ptr })
        }};
    }

    match elem {
        ArrElem::I32 => build_int_buf!(ArrI32, i32, (i32::MIN as i64..=i32::MAX as i64)),
        ArrElem::I64 => build_int_buf!(ArrI64, i64, (i64::MIN..=i64::MAX)),
        ArrElem::I16 => build_int_buf!(ArrI16, i16, (i16::MIN as i64..=i16::MAX as i64)),
        ArrElem::U8 => build_int_buf!(ArrU8, u8, (0i64..=255)),
        ArrElem::F32 => build_float_buf!(ArrF32, f32),
        ArrElem::F64 => build_float_buf!(ArrF64, f64),
    }
}

fn value_to_carg(value: VmValue, kind: &ArgKind, index: usize) -> Result<CArg, VmValue> {
    match (kind, value) {
        (ArgKind::Num("i32"), VmValue::Int(i)) => Ok(CArg::I32(i as i32)),
        (ArgKind::Num("i32"), VmValue::Byte(b)) => Ok(CArg::I32(b as i32)),
        (ArgKind::Num("i64"), VmValue::Int(i)) => Ok(CArg::I64(i)),
        (ArgKind::Num("i64"), VmValue::Byte(b)) => Ok(CArg::I64(b as i64)),
        (ArgKind::Num("f32"), VmValue::Float(f)) => Ok(CArg::F32(f as f32)),
        (ArgKind::Num("f64"), VmValue::Float(f)) => Ok(CArg::F64(f)),
        (ArgKind::Num("bool"), VmValue::Bool(b)) => Ok(CArg::Bool(u8::from(b))),
        (ArgKind::Num("u8"), VmValue::Byte(b)) => Ok(CArg::U8(b)),
        (ArgKind::Num("u8"), VmValue::Int(i)) => {
            if !(0..=255).contains(&i) {
                return Err(verr!(vs!(format!(
                    "call: arg {} is {} but \"u8\" only holds 0..=255",
                    index, i
                ))));
            }
            Ok(CArg::U8(i as u8))
        }
        (ArgKind::Num("i16"), VmValue::Int(i)) => {
            if !(i16::MIN as i64..=i16::MAX as i64).contains(&i) {
                return Err(verr!(vs!(format!(
                    "call: arg {} is {} but \"i16\" only holds {}..={}",
                    index,
                    i,
                    i16::MIN,
                    i16::MAX
                ))));
            }
            Ok(CArg::I16(i as i16))
        }
        (ArgKind::Num(t), other) => Err(verr!(vs!(format!(
            "call: arg {} declared as \"{}\" but got {}",
            index,
            t,
            other.type_name()
        )))),
        (ArgKind::Str(cap), VmValue::Str(s)) => {
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
        (ArgKind::Arr(elem, count), value) => arr_value_to_carg(value, *elem, *count, index),
    }
}
