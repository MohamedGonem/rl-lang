use crate::interpreter::values::Value;

pub mod input;

const KEYWORDS: &[&str] = &["input"];

pub fn is_in_io(name: &str) -> bool {
    if KEYWORDS.contains(&name) {
        return true;
    }
    false
}

pub fn match_std_io(name: &str, args: Vec<Value>) -> Value {
    match name {
        "input" => input::std_input(args),
        &_ => unreachable!(),
    }
}
