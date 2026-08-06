use rl_ast::Ast;
use rl_ast::statements::Statement;
use rl_interpreter::evaluator::Evaluator;
use rl_lexer::tokentypes::Token;
use rl_utils::{errors::Error, source::SourceFile};

pub fn lex(source: &str) -> Vec<Token> {
    let text = SourceFile::new("test", source.to_string());
    rl_lexer::tokenizer::Tokenizer::lex(text).expect("lex failed")
}

pub fn parse(source: &str) -> (Ast, Vec<Statement>) {
    let text = SourceFile::new("test", source.to_string());
    rl_parser::parser_logic::Parser::parse(lex(source), text).expect("parse failed")
}

pub fn eval_program(source: &str) -> Result<Evaluator, Error> {
    let file = SourceFile::new("test", source.to_string());
    let tokens = rl_lexer::tokenizer::Tokenizer::lex(file.clone())?;
    let (ast, stmts) = rl_parser::parser_logic::Parser::parse(tokens, file.clone())?;
    let mut evaluator = Evaluator::default().with_stdlib().with_source_file(file);
    let stmts = evaluator.resolver.resolve_program(ast, stmts);
    evaluator.evaluate_program(&stmts)?;
    Ok(evaluator)
}

pub fn compile_and_run(source: &str) -> Result<rl_vm::VmValue, rl_vm::VmError> {
    let file = SourceFile::new("test", source.to_string());
    let tokens = rl_lexer::tokenizer::Tokenizer::lex(file.clone()).expect("lex failed");
    let (ast, stmts) =
        rl_parser::parser_logic::Parser::parse(tokens, file.clone()).expect("parse failed");
    let mut evaluator = Evaluator::default().with_stdlib().with_source_file(file);
    let stmts = evaluator.resolver.resolve_program(ast, stmts);
    let chunk = rl_vm::Compiler::new(&evaluator.resolver.ast_arena)
        .compile(&stmts)
        .expect("compile failed");
    rl_vm::Vm::new().run_and_return(&chunk)
}
