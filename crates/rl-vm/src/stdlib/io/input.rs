use crate::{
    stdlib::macros::{verr, vf, vi, vok, vs},
    values::VmValue,
    vm_logic::{Vm, VmError},
};
use std::io::{self, Write};

fn input(prompt: Option<VmValue>) -> VmValue {
    match prompt {
        None => read_line(),
        Some(p) => {
            let prompt = match p {
                VmValue::Int(i) => i.to_string(),
                VmValue::Float(f) => f.to_string(),
                VmValue::Str(s) => s.to_string(),
                VmValue::Char(c) => c.to_string(),
                VmValue::Bool(b) => b.to_string(),
                VmValue::Null => "null".to_string(),
                _ => "".to_string(),
            };
            print!("{}", prompt);
            io::stdout().flush().ok();
            read_line()
        }
    }
}

fn read_line() -> VmValue {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => vok!(vs!(input.trim().to_string())),
        Err(e) => verr!(vs!(format!("read: failed to read line: {}", e))),
    }
}

pub fn std_read(_: &mut Vm, args: Vec<VmValue>) -> Result<VmValue, VmError> {
    let len = args.len();
    let mut args = args.into_iter();

    match len {
        0 => Ok(input(None)),
        1 => Ok(input(args.next())),
        n => Ok(verr!(vs!(format!(
            "read: expects 0 or 1 argument(s), got {}",
            n
        )))),
    }
}

pub fn std_read_int(_: &mut Vm, args: Vec<VmValue>) -> Result<VmValue, VmError> {
    let len = args.len();
    let mut args = args.into_iter();

    let value = match len {
        0 => input(None),
        1 => input(args.next()),
        n => {
            return Ok(verr!(vs!(format!(
                "read_int: expects 0 or 1 argument(s), got {}",
                n
            ))));
        }
    };

    match value {
        VmValue::Ok(inner) => match *inner {
            VmValue::Str(s) => match s.parse::<i64>() {
                Ok(i) => Ok(vok!(vi!(i))),
                Err(_) => Ok(verr!(vs!(format!(
                    "read_int: \"{}\" is not a valid integer",
                    s
                )))),
            },
            other => Ok(verr!(vs!(format!(
                "read_int: found unsupported type from input, got {}",
                other.type_name()
            )))),
        },
        // propagate a failed read as-is (e.g. stdin read error)
        err @ VmValue::Err(_) => Ok(err),
        other => Ok(verr!(vs!(format!(
            "read_int: found unsupported type from input, got {}",
            other.type_name()
        )))),
    }
}

pub fn std_read_float(_: &mut Vm, args: Vec<VmValue>) -> Result<VmValue, VmError> {
    let len = args.len();
    let mut args = args.into_iter();

    let value = match len {
        0 => input(None),
        1 => input(args.next()),
        n => {
            return Ok(verr!(vs!(format!(
                "read_float: expects 0 or 1 argument(s), got {}",
                n
            ))));
        }
    };

    match value {
        VmValue::Ok(inner) => match *inner {
            VmValue::Str(s) => match s.parse::<f64>() {
                Ok(f) => Ok(vok!(vf!(f))),
                Err(_) => Ok(verr!(vs!(format!(
                    "read_float: \"{}\" is not a valid float",
                    s
                )))),
            },
            other => Ok(verr!(vs!(format!(
                "read_float: found unsupported type from input, got {}",
                other.type_name()
            )))),
        },
        // propagate a failed read as-is (e.g. stdin read error)
        err @ VmValue::Err(_) => Ok(err),
        other => Ok(verr!(vs!(format!(
            "read_float: found unsupported type from input, got {}",
            other.type_name()
        )))),
    }
}
