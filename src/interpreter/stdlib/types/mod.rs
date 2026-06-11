mod bool_to_int;
mod char_to_int;
mod float_to_int;
mod from_bin;
mod from_hex;
mod int_to_bool;
mod int_to_char;
mod int_to_float;
mod to_bin;
mod to_hex;
mod to_oct;

use crate::interpreter::native::Module;

pub const KEYWORDS: &[&str] = &[
    "bool_to_int",
    "char_to_int",
    "float_to_int",
    "from_bin",
    "from_hex",
    "int_to_bool",
    "int_to_char",
    "int_to_float",
    "to_bin",
    "to_hex",
    "to_oct",
];

pub fn module() -> Module {
    Module::new("types")
}
