use crate::{interpreter::values::Value, utils::errors::Error};

pub fn std_tan(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        return Value::Null;
    }
    let target: f64 = match args[0] {
        Value::Integer(i) => i as f64,
        Value::Float(fl) => fl,
        _ => {
            Error::init(
                "only integers and floats are supported".to_string(),
                None,
                None,
            )
            .print_error();
            unreachable!()
        }
    };

    Value::Float(target.tan())
}
