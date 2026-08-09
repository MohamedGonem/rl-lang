use {rl_ast::statements::TypeAnnotation, rl_interpreter::values::Value};

pub fn int_array(items: Vec<i64>) -> Value {
    Value::Values {
        items_type: TypeAnnotation::Int,
        items: items.into_iter().map(Value::Integer).collect(),
    }
}

pub fn string_array(items: Vec<&str>) -> Value {
    Value::Values {
        items_type: TypeAnnotation::String,
        items: items
            .into_iter()
            .map(|s| Value::String(s.to_string()))
            .collect(),
    }
}
