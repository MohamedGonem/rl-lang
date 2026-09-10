use crate::entry::{FnEntry, StdEntry};

mod append_file;
mod close;
mod decode_utf8;
mod delete_file;
mod eprint;
mod eprintln;
mod encode_utf8;
mod flush;
mod isatty;
mod open;
mod print;
mod println;
mod read;
mod read_all;
mod read_all_stdin;
mod read_bytes;
mod read_file;
mod read_float;
mod read_handle;
mod read_int;
mod read_lines;
mod readline;
mod seek;
mod write_file;
mod write_handle;

pub static IO: StdEntry = StdEntry {
    name: "io",
    description: "functions for input and output",
    functions: FUNCTIONS,
    since: Some("v0.1.5"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &append_file::APPEND_FILE,
    &close::CLOSE,
    &decode_utf8::DECODE_UTF8,
    &delete_file::DELETE_FILE,
    &encode_utf8::ENCODE_UTF8,
    &eprint::EPRINT,
    &eprintln::EPRINTLN,
    &flush::FLUSH,
    &isatty::ISATTY,
    &open::OPEN,
    &print::PRINT,
    &println::PRINTLN,
    &read::READ,
    &read_all::READ_ALL,
    &read_all_stdin::READ_ALL_STDIN,
    &read_bytes::READ_BYTES,
    &read_file::READ_FILE,
    &read_float::READ_FLOAT,
    &read_float::READ_FLOAT_PROMPT,
    &read_handle::READ_HANDLE,
    &read_int::READ_INT,
    &read_int::READ_INT_PROMPT,
    &read_lines::READ_LINES,
    &read::READ_PROMPT,
    &readline::READLINE,
    &seek::SEEK,
    &write_file::WRITE_FILE,
    &write_handle::WRITE_HANDLE,
];
