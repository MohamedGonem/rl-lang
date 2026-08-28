use rl_ast::statements::TypeAnnotation as T;

pub fn type_to_c(ta: &T) -> &'static str {
    match ta {
        T::Int | T::CInt => "int64_t",
        T::UInt | T::CUInt => "uint64_t",
        T::SInt | T::CSInt => "int32_t",
        T::SUInt | T::CSUInt => "uint32_t",
        T::Float | T::CFloat => "double",
        T::SFloat | T::CSFloat => "float",
        T::Bool | T::CBool => "bool",
        T::Char | T::CChar => "char",
        T::String | T::CString => "rl_string",
        T::Byte | T::CByte => "uint8_t",
        T::SByte | T::CSByte => "int8_t",
        T::BByte | T::CBByte => "uint16_t",
        T::BSByte | T::CBSByte => "int16_t",
        T::Null => "void",
        // Compound types
        T::Array(_) | T::CArray(_) => "rl_array",
        T::Map(_, _) | T::CMap(_, _) => "rl_map",
        T::Set(_) | T::CSet(_) => "rl_set",
        T::Result(_) | T::CResult(_) => "rl_result",
        T::Tuple(_) | T::CTuple(_) => "rl_tuple",
        // placeholders
        T::Record(_) | T::CRecord(_) => "rl_value",
        T::Enum(_) | T::CEnum(_) => "rl_value",
        T::Fn => "rl_closure",
        T::Handle(_) => "rl_handle",
        T::HandleInfer => "rl_handle",
        T::Infer => "rl_value",
        T::Generic(_) => "rl_value",
        T::Callback(_, _) => "rl_closure",
        T::Error | T::CError => "rl_error",
    }
}
