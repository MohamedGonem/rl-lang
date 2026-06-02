use crate::interpreter::values::Value;

pub mod print;
pub mod println;

const KEYWORDS: &[&str] = &["print", "println"];

pub fn is_in_display(name: &str) -> bool {
    if KEYWORDS.contains(&name) {
        return true;
    }
    false
}

pub fn match_std_display(name: &str, args: Vec<Value>) -> Value {
    match name {
        "print" => print::std_print(args),
        "println" => println::std_println(args),
        &_ => unreachable!(),
    }
}
