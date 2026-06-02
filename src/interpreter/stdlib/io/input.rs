use std::io;

use crate::{interpreter::values::Value, utils::errors::Error};

pub fn std_input(args: Vec<Value>) -> Value {
    if args.is_empty() {
        input()
    } else if args.len() == 1 {
        input_with_prompt(args[0].clone())
    } else {
        Error::init("incorrect usage".to_string(), None, None).print_error();
        unreachable!()
    }
}

fn input() -> Value {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    Value::String(input.to_string())
}

fn input_with_prompt(prompt: Value) -> Value {
    let prompt = match prompt {
        Value::Integer(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => s,
        Value::Char(c) => c.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
    };

    println!("{}", prompt);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    Value::String(input.to_string())
}
