use crate::{
    stdlib::common::{extract_string, verr, vok, vs},
    values::VmValue,
    vm_logic::Vm,
};
use rl_checker::TypeChecker;
use rl_lexer::tokenizer::Tokenizer;
use rl_parser::parser_logic::Parser;
use rl_utils::source::SourceFile;
use std::rc::Rc;

pub fn func(_: &mut Vm, value: VmValue) -> VmValue {
    let code = match extract_string(value, "check") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(e)),
    };

    let source = SourceFile::new("<check>", code);

    let tokens = match Tokenizer::lex(source.clone()) {
        Ok(t) => t,
        Err(e) => return verr!(vs!(e.message().to_string())),
    };

    let (ast, statements) = match Parser::parse(tokens, source.clone()) {
        Ok(s) => s,
        Err(e) => return verr!(vs!(e.message().to_string())),
    };

    let mut checker = TypeChecker::new()
        .with_source_file(source)
        .with_ast_arena(ast);

    let errors = checker.check(&statements);

    if errors.is_empty() {
        return vok!(VmValue::Null);
    }
    verr!(VmValue::Arr(Rc::new(
        errors
            .iter()
            .map(|e| vs!(e.message().to_string()))
            .collect()
    )))
}
