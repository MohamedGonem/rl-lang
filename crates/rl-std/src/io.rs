//! `std::io` - input/output: reading from stdin, reading/writing files, printing.
//!
//! `print` and `println` write to [`Runtime::output_buffer`] when set (the REPL
//! captures per-input output there), otherwise directly to stdout. They are
//! variadic and untyped: they stringify each argument via `R::display` (the old
//! `Value::to_string`).
//!
//! `read`/`read_int`/`read_float` read a line from stdin. They are variadic and
//! untyped, accepting 0 or 1 optional prompt argument (any scalar, stringified
//! via `R::display`); with more than one argument they return an `err(..)`.
//! `read_int`/`read_float` then parse the line and return a language
//! `result[int]` / `result[float]`.
//!
//! The file functions (`read_file`, `read_lines`, `read_bytes`, `write_file`,
//! `append_file`, `delete_file`) return a language `result[T]` value.
//!
//! Handle-based functions (`open`, `close`, `read_handle`, `write_handle`,
//! `seek`, `flush`, `read_all`, `readline`) provide streaming and random-access
//! I/O. Handles are stored in the runtime's handle table via the [`IoStore`]
//! trait, using `HandleKind::File`.
//!
//! `eprint` raises a propagating runtime error rather than writing to stderr, so
//! errors surface through rl's normal error reporting pipeline. Ported once from
//! the former per-runtime `stdlib/io/*.rs` copies; logic and error strings are
//! preserved exactly.

#[cfg(feature = "impls")]
use std::io::{BufRead, Read, Seek, Write};
use rl_ast::statements::HandleKind;
use rl_std_core::Runtime;
use rl_std_macros::native_fn;

// ---- handle store ---------------------------------------------------------

/// A single native I/O resource stored behind an integer handle.
pub enum IoFileHandle {
    Read(std::io::BufReader<std::fs::File>),
    Write(std::fs::File),
    ReadWrite(std::io::BufReader<std::fs::File>, std::fs::File),
}

/// Per-runtime access to the `io` handle table. Implemented by `VmRuntime`.
pub trait IoStore: Runtime {
    fn io_insert(cx: &mut Self::Cx, h: IoFileHandle) -> u64;
    fn io_get(cx: &Self::Cx, id: u64) -> Option<&IoFileHandle>;
    fn io_get_mut(cx: &mut Self::Cx, id: u64) -> Option<&mut IoFileHandle>;
    fn io_remove(cx: &mut Self::Cx, id: u64) -> Option<IoFileHandle>;
}

/// Inserts a handle and returns its rl handle value.
#[cfg(feature = "impls")]
fn insert_handle<R: IoStore>(cx: &mut R::Cx, h: IoFileHandle) -> R::Value {
    let id = R::io_insert(cx, h);
    R::make_handle(HandleKind::File, id)
}

/// Extracts a `File` handle id from a value, or returns a type error.
#[cfg(feature = "impls")]
fn extract_handle<R: IoStore>(v: &R::Value, name: &str) -> Result<u64, String> {
    match R::as_handle(v, HandleKind::File) {
        Some(id) => Ok(id),
        None => match R::as_handle(v, HandleKind::C)
            .map(|_| HandleKind::C)
            .or_else(|| R::as_handle(v, HandleKind::Http).map(|_| HandleKind::Http))
            .or_else(|| R::as_handle(v, HandleKind::Audio).map(|_| HandleKind::Audio))
            .or_else(|| R::as_handle(v, HandleKind::Gui).map(|_| HandleKind::Gui))
            .or_else(|| R::as_handle(v, HandleKind::Net).map(|_| HandleKind::Net))
        {
            Some(kind) => Err(format!(
                "{}: expected a {:?} handle, got a {:?} handle",
                name,
                HandleKind::File,
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

// ---- printing (variadic, untyped) -----------------------------------------

#[native_fn(module = "io", untyped)]
pub fn print<R: Runtime>(cx: &mut R::Cx, args: Vec<R::Value>) -> R::Value {
    let text = args.iter().map(|v| R::display(v)).collect::<String>();
    if let Some(buffer) = R::output_buffer(cx) {
        buffer.push_str(&text);
    } else {
        print!("{}", text);
    }
    R::null()
}

#[native_fn(module = "io", untyped)]
pub fn println<R: Runtime>(cx: &mut R::Cx, args: Vec<R::Value>) -> R::Value {
    let text = args.iter().map(|v| R::display(v)).collect::<String>();
    if let Some(buffer) = R::output_buffer(cx) {
        buffer.push_str(&text);
        buffer.push('\n');
    } else {
        println!("{}", text);
    }
    R::null()
}

// ---- stdin reading --------------------------------------------------------

#[cfg(feature = "impls")]
fn read_line<R: Runtime>() -> R::Value {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => R::ok(R::from_string(input.trim().to_string())),
        Err(e) => R::err(R::from_string(format!("read: failed to read line: {}", e))),
    }
}

#[cfg(feature = "impls")]
fn input<R: Runtime>(prompt: Option<&R::Value>) -> R::Value {
    match prompt {
        None => read_line::<R>(),
        Some(p) => {
            print!("{}", R::display(p));
            std::io::stdout().flush().ok();
            read_line::<R>()
        }
    }
}

#[native_fn(module = "io",
    sig(-> result[string]),
    sig(int -> result[string]),
    sig(float -> result[string]),
    sig(string -> result[string]),
    sig(bool -> result[string]),
    sig(char -> result[string]))]
pub fn read<R: Runtime>(args: Vec<R::Value>) -> R::Value {
    match args.len() {
        0 => input::<R>(None),
        1 => input::<R>(args.first()),
        n => R::err(R::from_string(format!(
            "read: expects 0 or 1 argument(s), got {}",
            n
        ))),
    }
}

#[native_fn(module = "io",
    sig(-> result[int]),
    sig(int -> result[int]),
    sig(float -> result[int]),
    sig(string -> result[int]),
    sig(bool -> result[int]),
    sig(char -> result[int]))]
pub fn read_int<R: Runtime>(args: Vec<R::Value>) -> R::Value {
    let value = match args.len() {
        0 => input::<R>(None),
        1 => input::<R>(args.first()),
        n => {
            return R::err(R::from_string(format!(
                "read_int: expects 0 or 1 argument(s), got {}",
                n
            )));
        }
    };

    if let Some(inner) = R::as_ok_inner(&value) {
        match R::as_str(&inner) {
            Some(s) => match s.parse::<i64>() {
                Ok(i) => R::ok(R::from_i64(i)),
                Err(_) => R::err(R::from_string(format!(
                    "read_int: \"{}\" is not a valid integer",
                    s
                ))),
            },
            None => R::err(R::from_string(format!(
                "read_int: found unsupported type from input, got {}",
                R::type_name(&inner)
            ))),
        }
    } else if R::as_err_inner(&value).is_some() {
        value
    } else {
        R::err(R::from_string(format!(
            "read_int: found unsupported type from input, got {}",
            R::type_name(&value)
        )))
    }
}

#[native_fn(module = "io",
    sig(-> result[float]),
    sig(int -> result[float]),
    sig(float -> result[float]),
    sig(string -> result[float]),
    sig(bool -> result[float]),
    sig(char -> result[float]))]
pub fn read_float<R: Runtime>(args: Vec<R::Value>) -> R::Value {
    let value = match args.len() {
        0 => input::<R>(None),
        1 => input::<R>(args.first()),
        n => {
            return R::err(R::from_string(format!(
                "read_float: expects 0 or 1 argument(s), got {}",
                n
            )));
        }
    };

    if let Some(inner) = R::as_ok_inner(&value) {
        match R::as_str(&inner) {
            Some(s) => match s.parse::<f64>() {
                Ok(f) => R::ok(R::from_f64(f)),
                Err(_) => R::err(R::from_string(format!(
                    "read_float: \"{}\" is not a valid float",
                    s
                ))),
            },
            None => R::err(R::from_string(format!(
                "read_float: found unsupported type from input, got {}",
                R::type_name(&inner)
            ))),
        }
    } else if R::as_err_inner(&value).is_some() {
        value
    } else {
        R::err(R::from_string(format!(
            "read_float: found unsupported type from input, got {}",
            R::type_name(&value)
        )))
    }
}

// ---- file reading (language `result[T]`) ----------------------------------

#[native_fn(module = "io")]
pub fn read_file(file: String) -> Result<String, String> {
    match std::fs::read_to_string(&file) {
        Ok(d) => Ok(d),
        Err(e) => Err(format!("read_file: failed to read \"{}\": {}", file, e)),
    }
}

#[native_fn(module = "io")]
pub fn read_lines(file: String) -> Result<Vec<String>, String> {
    match std::fs::read_to_string(&file) {
        Ok(d) => Ok(d.lines().map(String::from).collect()),
        Err(e) => Err(format!("read_lines: failed to read \"{}\": {}", file, e)),
    }
}

#[native_fn(module = "io")]
pub fn read_bytes(file: String) -> Result<Vec<u8>, String> {
    match std::fs::read(&file) {
        Ok(d) => Ok(d),
        Err(e) => Err(format!("read_bytes: failed to read \"{}\": {}", file, e)),
    }
}

// ---- file writing (language `result[null]`) -------------------------------

#[native_fn(module = "io")]
pub fn write_file(file: String, content: String) -> Result<(), String> {
    match std::fs::write(&file, content) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("write_file: failed to write \"{}\": {}", file, e)),
    }
}

#[native_fn(module = "io")]
pub fn append_file(file: String, content: String) -> Result<(), String> {
    let mut file_data = match std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&file)
    {
        Ok(fd) => fd,
        Err(e) => {
            return Err(format!("append_file: failed to open \"{}\": {}", file, e));
        }
    };

    match file_data.write_all(content.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("append_file: failed to append \"{}\": {}", file, e)),
    }
}

#[native_fn(module = "io")]
pub fn delete_file(file: String) -> Result<(), String> {
    match std::fs::remove_file(&file) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("delete_file: failed to read \"{}\": {}", file, e)),
    }
}

// ---- handle-based I/O ----------------------------------------------------

#[native_fn(module = "io", bound = "IoStore",
    sig(string, string -> result[handle(File)]))]
pub fn open<R: IoStore>(cx: &mut R::Cx, file: R::Value, mode: R::Value) -> R::Value {
    let file_str = match R::as_str(&file) {
        Some(s) => s.to_owned(),
        None => {
            return R::err(R::from_string(format!(
                "open: expected string for file, got {}",
                R::type_name(&file)
            )))
        }
    };
    let mode_str = match R::as_str(&mode) {
        Some(s) => s.to_owned(),
        None => {
            return R::err(R::from_string(format!(
                "open: expected string for mode, got {}",
                R::type_name(&mode)
            )))
        }
    };

    let mut opts = std::fs::OpenOptions::new();
    match mode_str.as_str() {
        "r" => { opts.read(true); }
        "w" => { opts.write(true).create(true).truncate(true); }
        "a" => { opts.append(true).create(true); }
        "r+" => { opts.read(true).write(true).create(true); }
        "w+" => { opts.read(true).write(true).create(true).truncate(true); }
        "a+" => { opts.read(true).append(true).create(true); }
        _ => {
            return R::err(R::from_string(format!(
                "open: invalid mode \"{}\" (expected r, w, a, r+, w+, a+)",
                mode_str
            )));
        }
    }

    let file_handle = match opts.open(&file_str) {
        Ok(f) => f,
        Err(e) => {
            return R::err(R::from_string(format!(
                "open: failed to open \"{}\": {}", file_str, e
            )))
        }
    };

    let readable = mode_str.starts_with('r') || mode_str.contains('+');
    let writable = mode_str != "r" || mode_str.contains('+');

    let io_handle = match (readable, writable) {
        (true, true) => {
            let writer = match file_handle.try_clone() {
                Ok(f) => f,
                Err(e) => return R::err(R::from_string(format!("open: {}", e))),
            };
            let reader = std::io::BufReader::new(file_handle);
            IoFileHandle::ReadWrite(reader, writer)
        }
        (true, false) => IoFileHandle::Read(std::io::BufReader::new(file_handle)),
        (false, true) => IoFileHandle::Write(file_handle),
        (false, false) => {
            return R::err(R::from_string("open: mode must allow read or write".to_string()));
        }
    };

    R::ok(insert_handle::<R>(cx, io_handle))
}

#[native_fn(module = "io", bound = "IoStore",
    sig(handle(File) -> result[null]))]
pub fn close<R: IoStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "close") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };
    match R::io_remove(cx, id) {
        Some(_) => R::ok(R::null()),
        None => R::err(R::from_string("close: invalid handle".to_string())),
    }
}

#[native_fn(module = "io", bound = "IoStore",
    sig(handle(File), int -> result[string]))]
pub fn read_handle<R: IoStore>(cx: &mut R::Cx, handle: R::Value, n: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "read") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };
    let n = match R::as_i64(&n) {
        Some(v) => v,
        None => {
            return R::err(R::from_string(format!(
                "read: expected int for n, got {}",
                R::type_name(&n)
            )))
        }
    };

    let io_file = match R::io_get_mut(cx, id) {
        Some(f) => f,
        None => return R::err(R::from_string("read: invalid handle".to_string())),
    };

    let mut buf = vec![0u8; n.max(0) as usize];
    let bytes_read = match io_file {
        IoFileHandle::Read(r) => r.read(&mut buf),
        IoFileHandle::ReadWrite(r, _) => r.read(&mut buf),
        IoFileHandle::Write(_) => {
            return R::err(R::from_string("read: handle is not open for reading".to_string()))
        }
    };

    match bytes_read {
        Ok(bytes) => {
            buf.truncate(bytes);
            R::ok(R::from_string(String::from_utf8_lossy(&buf).into_owned()))
        }
        Err(e) => R::err(R::from_string(format!("read: {}", e))),
    }
}

#[native_fn(module = "io", bound = "IoStore",
    sig(handle(File), string -> result[int]))]
pub fn write_handle<R: IoStore>(cx: &mut R::Cx, handle: R::Value, data: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "write") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };
    let bytes = match R::as_str(&data) {
        Some(s) => s.as_bytes().to_vec(),
        None => {
            return R::err(R::from_string(format!(
                "write: expected string for data, got {}",
                R::type_name(&data)
            )))
        }
    };

    let io_file = match R::io_get_mut(cx, id) {
        Some(f) => f,
        None => return R::err(R::from_string("write: invalid handle".to_string())),
    };

    let result = match io_file {
        IoFileHandle::Write(w) => w.write_all(&bytes),
        IoFileHandle::ReadWrite(_, w) => w.write_all(&bytes),
        IoFileHandle::Read(_) => {
            return R::err(R::from_string("write: handle is not open for writing".to_string()))
        }
    };

    match result {
        Ok(()) => R::ok(R::from_i64(bytes.len() as i64)),
        Err(e) => R::err(R::from_string(format!("write: {}", e))),
    }
}

#[native_fn(module = "io", bound = "IoStore",
    sig(handle(File), int, int -> result[int]))]
pub fn seek<R: IoStore>(cx: &mut R::Cx, handle: R::Value, offset: R::Value, whence: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "seek") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };
    let offset = match R::as_i64(&offset) {
        Some(v) => v,
        None => {
            return R::err(R::from_string(format!(
                "seek: expected int for offset, got {}",
                R::type_name(&offset)
            )))
        }
    };
    let whence = match R::as_i64(&whence) {
        Some(v) => v,
        None => {
            return R::err(R::from_string(format!(
                "seek: expected int for whence, got {}",
                R::type_name(&whence)
            )))
        }
    };

    let seek_from = match whence {
        0 => std::io::SeekFrom::Start(offset.max(0) as u64),
        1 => std::io::SeekFrom::Current(offset),
        2 => std::io::SeekFrom::End(offset),
        _ => {
            return R::err(R::from_string(format!(
                "seek: invalid whence {} (expected 0, 1, or 2)",
                whence
            )))
        }
    };

    let io_file = match R::io_get_mut(cx, id) {
        Some(f) => f,
        None => return R::err(R::from_string("seek: invalid handle".to_string())),
    };

    let result = match io_file {
        IoFileHandle::Read(r) => r.seek(seek_from),
        IoFileHandle::ReadWrite(r, _) => r.seek(seek_from),
        IoFileHandle::Write(w) => w.seek(seek_from),
    };

    match result {
        Ok(pos) => R::ok(R::from_i64(pos as i64)),
        Err(e) => R::err(R::from_string(format!("seek: {}", e))),
    }
}

#[native_fn(module = "io", bound = "IoStore",
    sig(handle(File) -> result[null]))]
pub fn flush<R: IoStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "flush") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let io_file = match R::io_get_mut(cx, id) {
        Some(f) => f,
        None => return R::err(R::from_string("flush: invalid handle".to_string())),
    };

    let result = match io_file {
        IoFileHandle::Write(w) => w.flush(),
        IoFileHandle::ReadWrite(_, w) => w.flush(),
        IoFileHandle::Read(_) => {
            return R::err(R::from_string("flush: handle is not open for writing".to_string()))
        }
    };

    match result {
        Ok(()) => R::ok(R::null()),
        Err(e) => R::err(R::from_string(format!("flush: {}", e))),
    }
}

#[native_fn(module = "io", bound = "IoStore",
    sig(handle(File) -> result[string]))]
pub fn read_all<R: IoStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "read_all") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let io_file = match R::io_get_mut(cx, id) {
        Some(f) => f,
        None => return R::err(R::from_string("read_all: invalid handle".to_string())),
    };

    let mut buf = Vec::new();
    let result = match io_file {
        IoFileHandle::Read(r) => r.read_to_end(&mut buf),
        IoFileHandle::ReadWrite(r, _) => r.read_to_end(&mut buf),
        IoFileHandle::Write(_) => {
            return R::err(R::from_string("read_all: handle is not open for reading".to_string()))
        }
    };

    match result {
        Ok(_) => R::ok(R::from_string(String::from_utf8_lossy(&buf).into_owned())),
        Err(e) => R::err(R::from_string(format!("read_all: {}", e))),
    }
}

#[native_fn(module = "io", bound = "IoStore",
    sig(handle(File) -> result[string]))]
pub fn readline<R: IoStore>(cx: &mut R::Cx, handle: R::Value) -> R::Value {
    let id = match extract_handle::<R>(&handle, "readline") {
        Ok(id) => id,
        Err(e) => return R::err(R::from_string(e)),
    };

    let io_file = match R::io_get_mut(cx, id) {
        Some(f) => f,
        None => return R::err(R::from_string("readline: invalid handle".to_string())),
    };

    let mut line = String::new();
    let result = match io_file {
        IoFileHandle::Read(r) => r.read_line(&mut line),
        IoFileHandle::ReadWrite(r, _) => r.read_line(&mut line),
        IoFileHandle::Write(_) => {
            return R::err(R::from_string("readline: handle is not open for reading".to_string()))
        }
    };

    match result {
        Ok(0) => R::ok(R::from_string(String::new())),
        Ok(_) => {
            line.truncate(line.trim_end().len());
            R::ok(R::from_string(line))
        }
        Err(e) => R::err(R::from_string(format!("readline: {}", e))),
    }
}

// ---- stdin advanced -------------------------------------------------------

#[native_fn(module = "io")]
pub fn read_all_stdin() -> Result<String, String> {
    let mut input = String::new();
    match std::io::stdin().read_to_string(&mut input) {
        Ok(_) => Ok(input),
        Err(e) => Err(format!("read_all_stdin: {}", e)),
    }
}

// ---- encoding / decoding --------------------------------------------------

#[native_fn(module = "io")]
pub fn decode_utf8(bytes: Vec<u8>) -> Result<String, String> {
    match String::from_utf8(bytes) {
        Ok(s) => Ok(s),
        Err(e) => Err(format!(
            "decode_utf8: invalid UTF-8 at byte {}",
            e.utf8_error()
        )),
    }
}

#[native_fn(module = "io")]
pub fn encode_utf8(string: String) -> Vec<u8> {
    string.into_bytes()
}

// ---- terminal check -------------------------------------------------------

#[native_fn(module = "io")]
pub fn isatty() -> bool {
    unsafe { libc::isatty(libc::STDIN_FILENO) != 0 }
}

// ---- stderr ---------------------------------------------------------------

#[native_fn(module = "io", untyped)]
pub fn eprint<R: Runtime>(_cx: &mut R::Cx, args: Vec<R::Value>) -> R::Value {
    let text = args.iter().map(|v| R::display(v)).collect::<String>();
    eprint!("{}", text);
    R::null()
}

#[native_fn(module = "io", untyped)]
pub fn eprintln<R: Runtime>(_cx: &mut R::Cx, args: Vec<R::Value>) -> R::Value {
    let text = args.iter().map(|v| R::display(v)).collect::<String>();
    eprintln!("{}", text);
    R::null()
}

rl_std_core::native_module!("io";
    bound: IoStore;
    funcs: [
        print, println,
        read, read_int, read_float,
        read_file, read_lines, read_bytes,
        write_file, append_file, delete_file,
        open, close,
        read_handle, write_handle, seek, flush, read_all, readline,
        read_all_stdin,
        decode_utf8, encode_utf8,
        isatty,
        eprint, eprintln,
    ],
);
