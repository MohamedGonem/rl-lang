use libloading::Symbol;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        c::CHandle,
        common::{extract_number, extract_string, verr, vi, vok, vs},
    },
    values::Value,
};

const MAX_ARGS: usize = 6;

pub fn func(eval: &mut Evaluator, handle: Value, fn_name: Value, args: Value) -> Value {
    let handle_id = match extract_number(handle, "call") {
        Ok(n) => n as i64,
        Err(e) => return verr!(vs!(format!("call: {}", e))),
    };
    let fn_name = match extract_string(fn_name, "call") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("call: {}", e))),
    };
    let args: Vec<i64> = match args {
        Value::Values { items, .. } => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                match item {
                    Value::Integer(i) => out.push(i),
                    Value::Byte(b) => out.push(b as i64),
                    other => {
                        return verr!(vs!(format!(
                            "call: all args must be int, found {}",
                            other.type_name()
                        )));
                    }
                }
            }
            out
        }
        other => {
            return verr!(vs!(format!(
                "call: expected an array of int args, found {}",
                other.type_name()
            )));
        }
    };
    if args.len() > MAX_ARGS {
        return verr!(vs!(format!(
            "call: at most {} args are supported in this version, got {}",
            MAX_ARGS,
            args.len()
        )));
    }
}
