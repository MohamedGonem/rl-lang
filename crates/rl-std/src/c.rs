//! `std::c` - compile or load a C shared library and call into it. Handle
//! module: loaded libraries are stored behind integer handles in the runtime's
//! handle table, accessed via the `CStore` trait (implemented by each runtime).
//!
//! This module pulls in `libloading`/`libffi`, so it is only built under the
//! `impls` feature (gated at the `mod` site in `lib.rs`, like `process` /
//! `terminal`).

use libffi::middle::{Arg, Cif, CodePtr, Type, arg};
use libloading::{Library, Symbol};
use rl_ast::statements::{HandleKind, TypeAnnotation};
use rl_std_core::Runtime;
use rl_std_macros::native_fn;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::process::Command;

/// A single native C-interop resource, stored behind an `int` handle.
/// (Moved here from the per-runtime copies so both share one definition.)
pub enum CHandle {
    Library(Library),
}

/// Per-runtime access to the `c` handle table. Implemented by `VmRuntime` /
/// `EvalRuntime` in the runtime crates.
pub trait CStore: Runtime {
    fn c_insert(cx: &mut Self::Cx, h: CHandle) -> u64;
    fn c_get(cx: &Self::Cx, id: u64) -> Option<&CHandle>;
    fn c_get_mut(cx: &mut Self::Cx, id: u64) -> Option<&mut CHandle>;
    fn c_remove(cx: &mut Self::Cx, id: u64) -> Option<CHandle>;
}

/// Inserts a handle and returns its rl handle value.
fn insert_handle<R: CStore>(cx: &mut R::Cx, h: CHandle) -> R::Value {
    let id = R::c_insert(cx, h);
    R::make_handle(HandleKind::C, id)
}

// ---- shared argument extraction (reproducing the old `extract_*` helpers) --

/// Reproduces the old `extract_string`: rejects non-strings with
/// `"<name>: expected string type, got <ty>"`.
fn extract_string<R: CStore>(v: &R::Value, name: &str) -> Result<String, String> {
    match R::as_str(v) {
        Some(s) => Ok(s.to_owned()),
        None => Err(format!(
            "{}: expected string type, got {}",
            name,
            R::type_name(v)
        )),
    }
}

/// Reproduces the old `extract_handle`: unwraps a `C` handle into its id, with
/// the same wrong-kind / not-a-handle messages.
fn extract_handle<R: CStore>(v: &R::Value, name: &str) -> Result<u64, String> {
    match R::as_handle(v, HandleKind::C) {
        Some(id) => Ok(id),
        None => match R::as_handle(v, HandleKind::Net)
            .map(|_| HandleKind::Net)
            .or_else(|| R::as_handle(v, HandleKind::Http).map(|_| HandleKind::Http))
            .or_else(|| R::as_handle(v, HandleKind::Audio).map(|_| HandleKind::Audio))
            .or_else(|| R::as_handle(v, HandleKind::Gui).map(|_| HandleKind::Gui))
        {
            Some(kind) => Err(format!(
                "{}: expected a {:?} handle, got a {:?} handle",
                name,
                HandleKind::C,
                kind
            )),
            None => Err(format!(
                "{}: expected a handle, got {}",
                name,
                R::type_name(v)
            )),
        },
    }
}

// ---- compile cache ---------------------------------------------------------

/// Where `compile` caches built shared libraries by content hash, and what
/// `clear_cache` empties. Kept here (not duplicated in each function) so
/// there's exactly one place both agree on the path - and deliberately the
/// *same* path both runtimes' `std::c::compile` uses, so switching between
/// interpreter and VM doesn't force a recompile.
fn cache_dir() -> PathBuf {
    std::env::temp_dir().join("rl_std_c_cache")
}

#[cfg(target_os = "windows")]
const SHARED_LIB_EXT: &str = "dll";
#[cfg(target_os = "macos")]
const SHARED_LIB_EXT: &str = "dylib";
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
const SHARED_LIB_EXT: &str = "so";

/// Builds the compiler invocation for turning `src` into `out` on this platform.
/// macOS wants `-dynamiclib`; everything else (incl. mingw-w64 `cc`/`gcc` on
/// Windows) accepts `-shared -fPIC`. Native MSVC (`cl.exe`) is not supported.
fn compiler_command(compiler: &str, src: &PathBuf, out: &PathBuf) -> Command {
    let mut cmd = Command::new(compiler);
    #[cfg(target_os = "macos")]
    {
        cmd.args(["-dynamiclib", "-O2", "-o"]).arg(out).arg(src);
    }
    #[cfg(not(target_os = "macos"))]
    {
        cmd.args(["-shared", "-fPIC", "-O2", "-o"])
            .arg(out)
            .arg(src);
    }
    cmd
}

fn find_compiler() -> String {
    std::env::var("CC").unwrap_or_else(|_| "cc".to_string())
}

// ---- compile / load --------------------------------------------------------

#[native_fn(module = "c", bound = "CStore", sig(string -> result[handle(C)]))]
pub fn compile<R: CStore>(cx: &mut R::Cx, source: R::Value) -> R::Value {
    let source = match extract_string::<R>(&source, "compile") {
        Ok(s) => s,
        Err(e) => return R::err(R::from_string(format!("compile: {}", e))),
    };

    let mut hasher = DefaultHasher::new();
    source.hash(&mut hasher);
    let hash = hasher.finish();

    let dir = cache_dir();
    if let Err(e) = std::fs::create_dir_all(&dir) {
        return R::err(R::from_string(format!(
            "compile: failed to create cache dir {}: {}",
            dir.display(),
            e
        )));
    }

    let src_path = dir.join(format!("{:016x}.c", hash));
    let out_path = dir.join(format!("{:016x}.{}", hash, SHARED_LIB_EXT));

    // Cache hit: skip both writing the source and recompiling.
    if !out_path.exists() {
        if let Err(e) = std::fs::write(&src_path, &source) {
            return R::err(R::from_string(format!(
                "compile: failed to write {}: {}",
                src_path.display(),
                e
            )));
        }

        let compiler = find_compiler();
        let output = compiler_command(&compiler, &src_path, &out_path).output();

        match output {
            Ok(o) if o.status.success() => {}
            Ok(o) => {
                let stderr = String::from_utf8_lossy(&o.stderr);
                return R::err(R::from_string(format!(
                    "compile: `{}` failed:\n{}",
                    compiler,
                    stderr.trim_end()
                )));
            }
            Err(e) => {
                return R::err(R::from_string(format!(
                    "compile: couldn't run `{}` (set $CC to point at a C compiler if it's not on $PATH): {}",
                    compiler, e
                )));
            }
        }
    }

    // SAFETY: loading the shared library we just compiled from a fixed cache
    // path. The library's static initializers (if any) run here.
    let lib = match unsafe { Library::new(&out_path) } {
        Ok(l) => l,
        Err(e) => {
            return R::err(R::from_string(format!(
                "compile: failed to load compiled library {}: {}",
                out_path.display(),
                e
            )));
        }
    };

    R::ok(insert_handle::<R>(cx, CHandle::Library(lib)))
}

#[native_fn(module = "c", bound = "CStore", sig(string -> result[handle(C)]))]
pub fn load<R: CStore>(cx: &mut R::Cx, path: R::Value) -> R::Value {
    let path = match extract_string::<R>(&path, "load") {
        Ok(s) => s,
        Err(e) => return R::err(R::from_string(format!("load: {}", e))),
    };

    // SAFETY: loading a library the caller pointed us at by path. Its static
    // initializers (if any) run here, same as `compile`'s load step and same
    // as `dlopen` in C itself - this is inherently trusting whatever code is
    // at that path, not something a Rust-side check can make "safe".
    let lib = match unsafe { Library::new(&path) } {
        Ok(l) => l,
        Err(e) => {
            return R::err(R::from_string(format!(
                "load: failed to load \"{}\": {}",
                path, e
            )));
        }
    };

    R::ok(insert_handle::<R>(cx, CHandle::Library(lib)))
}

// ---- has_symbol / close / clear_cache --------------------------------------

#[native_fn(module = "c", bound = "CStore", sig(handle(C), string -> result[bool]))]
pub fn has_symbol<R: CStore>(cx: &mut R::Cx, handle: R::Value, fn_name: R::Value) -> R::Value {
    let handle_id = match extract_handle::<R>(&handle, "has_symbol") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(format!("has_symbol: {}", e))),
    };
    let fn_name = match extract_string::<R>(&fn_name, "has_symbol") {
        Ok(s) => s,
        Err(e) => return R::err(R::from_string(format!("has_symbol: {}", e))),
    };

    let CHandle::Library(lib) = match R::c_get(cx, handle_id) {
        Some(h) => h,
        None => {
            return R::err(R::from_string(format!(
                "has_symbol: unknown handle {}",
                handle_id
            )));
        }
    };

    // SAFETY: same as `call`'s symbol lookup - resolving a symbol address is
    // unsafe per `libloading`'s contract, but we never call through it here,
    // only check whether the lookup itself succeeds.
    let found = unsafe { lib.get::<*const ()>(fn_name.as_bytes()) }.is_ok();
    R::ok(R::from_bool(found))
}

#[native_fn(module = "c", bound = "CStore", sig(handle(C) -> result[null]))]
pub fn close<R: CStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let handle_id = match extract_handle::<R>(&handle, "close") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(format!("close: {}", e))),
    };

    match R::c_remove(cx, handle_id) {
        Some(_) => R::ok(R::null()),
        None => R::err(R::from_string(format!("close: unknown handle {}", handle_id))),
    }
}

#[native_fn(module = "c", bound = "CStore", sig( -> result[null]))]
pub fn clear_cache<R: CStore>(_cx: &mut R::Cx) -> R::Value {
    let dir = cache_dir();
    match std::fs::remove_dir_all(&dir) {
        Ok(()) => R::ok(R::null()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => R::ok(R::null()),
        Err(e) => R::err(R::from_string(format!(
            "clear_cache: failed to remove {}: {}",
            dir.display(),
            e
        ))),
    }
}

// ---- call ------------------------------------------------------------------

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

/// One argument's value, converted from its rl-lang value into the exact Rust
/// representation its declared C type requires. Kept as an owned value (rather
/// than immediately building an `Arg`) so it has somewhere to live while the
/// `Arg`s that borrow from it are assembled and passed to `Cif::call`.
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

#[native_fn(module = "c", bound = "CStore", untyped)]
pub fn call<R: CStore>(
    cx: &mut R::Cx,
    handle: R::Value,
    fn_name: R::Value,
    args: R::Value,
    arg_types: R::Value,
    ret_type: R::Value,
) -> R::Value {
    let handle_id = match extract_handle::<R>(&handle, "call") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(format!("call: {}", e))),
    };
    let fn_name = match extract_string::<R>(&fn_name, "call") {
        Ok(s) => s,
        Err(e) => return R::err(R::from_string(format!("call: {}", e))),
    };
    let ret_type = match extract_string::<R>(&ret_type, "call") {
        Ok(s) => s,
        Err(e) => return R::err(R::from_string(format!("call: {}", e))),
    };

    let arg_type_names: Vec<String> = match R::as_array(&arg_types) {
        Some((items, _)) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items.iter() {
                match R::as_str(item) {
                    Some(s) => out.push(s.to_owned()),
                    None => {
                        return R::err(R::from_string(format!(
                            "call: arg_types must be an array of string, found {}",
                            R::type_name(item)
                        )));
                    }
                }
            }
            out
        }
        None => {
            return R::err(R::from_string(format!(
                "call: expected an array of string for arg_types, found {}",
                R::type_name(&arg_types)
            )));
        }
    };

    let arg_values: Vec<R::Value> = match R::as_tuple(&args) {
        Some(items) => items.to_vec(),
        None => {
            return R::err(R::from_string(format!(
                "call: expected a tuple of args (e.g. (\"hello\", 16)), found {}",
                R::type_name(&args)
            )));
        }
    };

    if arg_values.len() != arg_type_names.len() {
        return R::err(R::from_string(format!(
            "call: {} arg(s) but {} arg_type(s) given",
            arg_values.len(),
            arg_type_names.len()
        )));
    }

    let mut arg_kinds: Vec<ArgKind> = Vec::with_capacity(arg_type_names.len());
    for type_name in &arg_type_names {
        match parse_arg_kind::<R>(type_name) {
            Ok(k) => arg_kinds.push(k),
            Err(e) => return e,
        }
    }

    let ret_ffi_type = match parse_ret_type::<R>(&ret_type) {
        Ok(t) => t,
        Err(e) => return e,
    };

    let mut c_args: Vec<CArg> = Vec::with_capacity(arg_values.len());
    for (i, (value, kind)) in arg_values.into_iter().zip(&arg_kinds).enumerate() {
        match value_to_carg::<R>(value, kind, i) {
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

    let CHandle::Library(lib) = match R::c_get(cx, handle_id) {
        Some(h) => h,
        None => {
            return R::err(R::from_string(format!(
                "call: unknown handle {}",
                handle_id
            )));
        }
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
    let ret_value: R::Value = unsafe {
        let sym: Symbol<'_, unsafe extern "C" fn()> = match lib.get(fn_name.as_bytes()) {
            Ok(s) => s,
            Err(e) => {
                return R::err(R::from_string(format!(
                    "call: symbol \"{}\" not found: {}",
                    fn_name, e
                )));
            }
        };
        let code_ptr = CodePtr::from_fun(*sym);

        match ret_type.as_str() {
            "void" => {
                let _: () = cif.call(code_ptr, &ffi_args);
                R::null()
            }
            "i32" => R::from_i64(cif.call::<i32>(code_ptr, &ffi_args) as i64),
            "i64" => R::from_i64(cif.call::<i64>(code_ptr, &ffi_args)),
            "f32" => R::from_f64(cif.call::<f32>(code_ptr, &ffi_args) as f64),
            "f64" => R::from_f64(cif.call::<f64>(code_ptr, &ffi_args)),
            "bool" => R::from_bool(cif.call::<u8>(code_ptr, &ffi_args) != 0),
            "u8" => R::from_u8(cif.call::<u8>(code_ptr, &ffi_args)),
            "i16" => R::from_i64(cif.call::<i16>(code_ptr, &ffi_args) as i64),
            _ => unreachable!("ret_type validated by parse_ret_type above"),
        }
    };

    // Read back any `str`/`arr` buffers C may have written into, in the
    // order their args appeared - this is the only "mutation" that crosses
    // back into rl-lang, and only because the caller explicitly opted an
    // arg into it via `"str:N"`/`"arr:TYPE:N"`.
    let mutated_outs: Vec<R::Value> = c_args
        .into_iter()
        .filter_map(|c| match c {
            CArg::Str { buf, .. } => {
                let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
                Some(R::from_string(
                    String::from_utf8_lossy(&buf[..end]).into_owned(),
                ))
            }
            CArg::ArrI32 { buf, .. } => Some(R::array(
                buf.into_iter().map(|v| R::from_i64(v as i64)).collect(),
                TypeAnnotation::Int,
            )),
            CArg::ArrI64 { buf, .. } => Some(R::array(
                buf.into_iter().map(R::from_i64).collect(),
                TypeAnnotation::Int,
            )),
            CArg::ArrI16 { buf, .. } => Some(R::array(
                buf.into_iter().map(|v| R::from_i64(v as i64)).collect(),
                TypeAnnotation::Int,
            )),
            CArg::ArrU8 { buf, .. } => Some(R::array(
                buf.into_iter().map(R::from_u8).collect(),
                TypeAnnotation::Byte,
            )),
            CArg::ArrF32 { buf, .. } => Some(R::array(
                buf.into_iter().map(|v| R::from_f64(v as f64)).collect(),
                TypeAnnotation::Float,
            )),
            CArg::ArrF64 { buf, .. } => Some(R::array(
                buf.into_iter().map(R::from_f64).collect(),
                TypeAnnotation::Float,
            )),
            _ => None,
        })
        .collect();

    if mutated_outs.is_empty() {
        R::ok(ret_value)
    } else {
        R::ok(R::tuple(vec![ret_value, R::tuple(mutated_outs)]))
    }
}

fn parse_arg_kind<R: CStore>(name: &str) -> Result<ArgKind, R::Value> {
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
                R::err(R::from_string(format!(
                    "call: invalid str capacity in \"{}\" (expected e.g. \"str:64\")",
                    name
                )))
            })?;
            if cap == 0 {
                return Err(R::err(R::from_string(format!(
                    "call: str capacity in \"{}\" must be at least 1",
                    name
                ))));
            }
            Ok(ArgKind::Str(cap))
        }
        "str" => Err(R::err(R::from_string(
            "call: \"str\" needs an explicit capacity, e.g. \"str:64\" - \
             there's no safe default to write past"
                .to_string(),
        ))),
        _ if name.starts_with("arr:") => {
            let rest = &name["arr:".len()..];
            let (elem_name, count_str) = rest.split_once(':').ok_or_else(|| {
                R::err(R::from_string(format!(
                    "call: invalid arr type \"{}\" (expected e.g. \"arr:i32:8\")",
                    name
                )))
            })?;
            let elem = ArrElem::parse(elem_name).ok_or_else(|| {
                R::err(R::from_string(format!(
                    "call: unsupported arr element type \"{}\" in \"{}\" \
                     (supported: i32, i64, f32, f64, u8, i16)",
                    elem_name, name
                )))
            })?;
            let count: usize = count_str.parse().map_err(|_| {
                R::err(R::from_string(format!(
                    "call: invalid arr element count in \"{}\" (expected e.g. \"arr:i32:8\")",
                    name
                )))
            })?;
            if count == 0 {
                return Err(R::err(R::from_string(format!(
                    "call: arr element count in \"{}\" must be at least 1",
                    name
                ))));
            }
            Ok(ArgKind::Arr(elem, count))
        }
        other => Err(R::err(R::from_string(format!(
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

fn parse_ret_type<R: CStore>(name: &str) -> Result<Type, R::Value> {
    match name {
        "void" => Ok(Type::void()),
        "i32" | "i64" | "i16" | "u8" | "f32" | "f64" | "bool" => Ok(numeric_ffi_type(name)),
        other => Err(R::err(R::from_string(format!(
            "call: unsupported return type \"{}\" (supported: i32, i64, i16, u8, f32, f64, bool, \
             void - no \"str\"/\"arr\" return: unmanaged C memory's ownership can't be known safely)",
            other
        )))),
    }
}

/// Builds one `"arr:TYPE:N"` argument's buffer from an rl-lang `arr[T]`,
/// checking every element's type and that the length is exactly `count`.
fn arr_value_to_carg<R: CStore>(
    value: R::Value,
    elem: ArrElem,
    count: usize,
    index: usize,
) -> Result<CArg, R::Value> {
    let items: Vec<R::Value> = match R::as_array(&value) {
        Some((items, _)) => items.to_vec(),
        None => {
            return Err(R::err(R::from_string(format!(
                "call: arg {} declared as arr:{}:{} but got {}",
                index,
                elem.name(),
                count,
                R::type_name(&value)
            ))));
        }
    };
    if items.len() != count {
        return Err(R::err(R::from_string(format!(
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
                let v: i64 = if let Some(n) = R::as_i64(&item) {
                    n
                } else if let Some(b) = R::as_u8(&item) {
                    b as i64
                } else {
                    return Err(R::err(R::from_string(format!(
                        "call: arg {} element {} declared as {} but got {}",
                        index,
                        i,
                        elem.name(),
                        R::type_name(&item)
                    ))));
                };
                if !$range.contains(&v) {
                    return Err(R::err(R::from_string(format!(
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
                if let Some(f) = R::as_f64(&item) {
                    buf.push(f as $ty);
                } else {
                    return Err(R::err(R::from_string(format!(
                        "call: arg {} element {} declared as {} but got {}",
                        index,
                        i,
                        elem.name(),
                        R::type_name(&item)
                    ))));
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

fn value_to_carg<R: CStore>(
    value: R::Value,
    kind: &ArgKind,
    index: usize,
) -> Result<CArg, R::Value> {
    match kind {
        ArgKind::Num("i32") => {
            if let Some(i) = R::as_i64(&value) {
                Ok(CArg::I32(i as i32))
            } else if let Some(b) = R::as_u8(&value) {
                Ok(CArg::I32(b as i32))
            } else {
                Err(R::err(R::from_string(format!(
                    "call: arg {} declared as \"{}\" but got {}",
                    index,
                    "i32",
                    R::type_name(&value)
                ))))
            }
        }
        ArgKind::Num("i64") => {
            if let Some(i) = R::as_i64(&value) {
                Ok(CArg::I64(i))
            } else if let Some(b) = R::as_u8(&value) {
                Ok(CArg::I64(b as i64))
            } else {
                Err(R::err(R::from_string(format!(
                    "call: arg {} declared as \"{}\" but got {}",
                    index,
                    "i64",
                    R::type_name(&value)
                ))))
            }
        }
        ArgKind::Num("f32") => {
            if let Some(f) = R::as_f64(&value) {
                Ok(CArg::F32(f as f32))
            } else {
                Err(R::err(R::from_string(format!(
                    "call: arg {} declared as \"{}\" but got {}",
                    index,
                    "f32",
                    R::type_name(&value)
                ))))
            }
        }
        ArgKind::Num("f64") => {
            if let Some(f) = R::as_f64(&value) {
                Ok(CArg::F64(f))
            } else {
                Err(R::err(R::from_string(format!(
                    "call: arg {} declared as \"{}\" but got {}",
                    index,
                    "f64",
                    R::type_name(&value)
                ))))
            }
        }
        ArgKind::Num("bool") => {
            if let Some(b) = R::as_bool(&value) {
                Ok(CArg::Bool(u8::from(b)))
            } else {
                Err(R::err(R::from_string(format!(
                    "call: arg {} declared as \"{}\" but got {}",
                    index,
                    "bool",
                    R::type_name(&value)
                ))))
            }
        }
        ArgKind::Num("u8") => {
            if let Some(b) = R::as_u8(&value) {
                Ok(CArg::U8(b))
            } else if let Some(i) = R::as_i64(&value) {
                if !(0..=255).contains(&i) {
                    return Err(R::err(R::from_string(format!(
                        "call: arg {} is {} but \"u8\" only holds 0..=255",
                        index, i
                    ))));
                }
                Ok(CArg::U8(i as u8))
            } else {
                Err(R::err(R::from_string(format!(
                    "call: arg {} declared as \"{}\" but got {}",
                    index,
                    "u8",
                    R::type_name(&value)
                ))))
            }
        }
        ArgKind::Num("i16") => {
            if let Some(i) = R::as_i64(&value) {
                if !(i16::MIN as i64..=i16::MAX as i64).contains(&i) {
                    return Err(R::err(R::from_string(format!(
                        "call: arg {} is {} but \"i16\" only holds {}..={}",
                        index,
                        i,
                        i16::MIN,
                        i16::MAX
                    ))));
                }
                Ok(CArg::I16(i as i16))
            } else {
                Err(R::err(R::from_string(format!(
                    "call: arg {} declared as \"{}\" but got {}",
                    index,
                    "i16",
                    R::type_name(&value)
                ))))
            }
        }
        ArgKind::Num(t) => Err(R::err(R::from_string(format!(
            "call: arg {} declared as \"{}\" but got {}",
            index,
            t,
            R::type_name(&value)
        )))),
        ArgKind::Str(cap) => {
            if let Some(s) = R::as_str(&value) {
                let cap = *cap;
                if s.len() >= cap {
                    return Err(R::err(R::from_string(format!(
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
            } else {
                Err(R::err(R::from_string(format!(
                    "call: arg {} declared as str but got {}",
                    index,
                    R::type_name(&value)
                ))))
            }
        }
        ArgKind::Arr(elem, count) => arr_value_to_carg::<R>(value, *elem, *count, index),
    }
}

rl_std_core::native_module!("c";
    bound: CStore;
    funcs: [
        compile, load, call, has_symbol, close, clear_cache,
    ],
);
