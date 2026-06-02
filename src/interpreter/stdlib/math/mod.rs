pub mod cos;
pub mod modulo;
pub mod power;
pub mod sin;
pub mod tan;

use crate::interpreter::values::Value;

const KEYWORDS: &[&str] = &["sin", "cos", "tan", "pow", "mod"];

pub fn is_in_math(name: &str) -> bool {
    if KEYWORDS.contains(&name) {
        return true;
    }
    false
}

pub fn match_std_math(name: &str, args: Vec<Value>) -> Value {
    match name {
        "cos" => cos::std_cos(args),
        "sin" => sin::std_sin(args),
        "tan" => tan::std_tan(args),
        "pow" => power::std_pow(args),
        "mod" => modulo::std_mod(args),
        &_ => unreachable!(),
    }
}
