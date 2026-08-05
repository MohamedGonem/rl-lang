use crate::{
    stdlib::common::{extract_string, verr, vok, vs},
    stdlib::macros::vi,
    values::VmValue,
    vm_logic::Vm,
};
use rl_lexer::tokenizer::Tokenizer;
use rl_utils::source::SourceFile;
use std::rc::Rc;

pub fn func(_: &mut Vm, value: VmValue) -> VmValue {
    let code = match extract_string(value, "lex") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(e)),
    };

    let source = SourceFile::new("<lex>", code);

    let tokens = match Tokenizer::lex(source) {
        Ok(t) => t,
        Err(e) => return verr!(vs!(e.message().to_string())),
    };

    let items: Vec<VmValue> = tokens
        .into_iter()
        .map(|t| {
            let kind = format!("{:?}", t.token);
            let kind = kind.split('(').next().unwrap_or(&kind).to_string();
            VmValue::Tuple(Rc::new(vec![
                vs!(kind),
                vs!(t.lexeme.to_string()),
                vi!(t.line as i64),
            ]))
        })
        .collect();

    vok!(VmValue::Arr(Rc::new(items)))
}
