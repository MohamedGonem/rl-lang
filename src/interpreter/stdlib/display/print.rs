use crate::interpreter::values::Value;

pub fn std_print(args: Vec<Value>) -> Value {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", arg);
    }
    Value::Null
}
