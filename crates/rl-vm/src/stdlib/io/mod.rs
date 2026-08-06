//! `std::io` - input/output: reading from stdin, reading/writing files, printing.
//!
//! `print` and `println` write to [`Vm::output_buffer`] when set (the REPL
//! captures per-input output there), otherwise directly to stdout.
//!
//! `eprint` raises a runtime error rather than writing to stderr, so errors
//! surface through rl's normal error reporting pipeline.

mod append_file;
mod delete_file;
mod eprint;
mod input;
mod read_bytes;
mod read_file;
mod read_lines;
mod write_file;

use crate::native::Module;
use crate::values::VmValue;
use crate::vm_logic::{Vm, VmError};

pub fn module() -> Module {
    Module::new("io")
        .with_raw_function("read", input::std_read)
        .with_raw_function("read_int", input::std_read_int)
        .with_raw_function("read_float", input::std_read_float)
        .with_function("read_file", read_file::std_read_file)
        .with_function("read_lines", read_lines::std_read_lines)
        .with_function("delete_file", delete_file::std_delete_file)
        .with_function("write_file", write_file::std_write_file)
        .with_function("append_file", append_file::std_append_file)
        .with_raw_function("print", std_print)
        .with_raw_function("println", std_println)
        .with_function("eprint", eprint::std_eprint)
        .with_function("read_bytes", read_bytes::std_read_bytes)
}

fn std_print(vm: &mut Vm, args: Vec<VmValue>) -> Result<VmValue, VmError> {
    let text = args.iter().map(|v| v.to_string()).collect::<String>();
    if let Some(buffer) = &mut vm.output_buffer {
        buffer.push_str(&text);
    } else {
        print!("{}", text);
    }
    Ok(VmValue::Null)
}

fn std_println(vm: &mut Vm, args: Vec<VmValue>) -> Result<VmValue, VmError> {
    let text = args.iter().map(|v| v.to_string()).collect::<String>();
    if let Some(buffer) = &mut vm.output_buffer {
        buffer.push_str(&text);
        buffer.push('\n');
    } else {
        println!("{}", text);
    }
    Ok(VmValue::Null)
}
