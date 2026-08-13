use rl_ast::Ast;
use rl_ast::statements::Statement;
use rl_checker::TypeChecker;
use rl_lexer::tokentypes::Token;
use rl_resolver::Resolver;
use rl_utils::source::SourceFile;

pub fn lex(source: &str) -> Vec<Token> {
    let text = SourceFile::new("test", source.to_string());
    rl_lexer::tokenizer::Tokenizer::lex(text).expect("lex failed")
}

pub fn parse(source: &str) -> (Ast, Vec<Statement>) {
    let text = SourceFile::new("test", source.to_string());
    rl_parser::parser_logic::Parser::parse(lex(source), text).expect("parse failed")
}

/// Parses `source`, asserting that lexing or parsing fails, and returns the
/// first error message so tests can assert on its contents.
pub fn parse_assert_err(source: &str) -> String {
    let text = SourceFile::new("test", source.to_string());
    let result = rl_lexer::tokenizer::Tokenizer::lex(text.clone())
        .and_then(|tokens| {
            rl_parser::parser_logic::Parser::parse(tokens, text)
                .map(|_| panic!("expected `{source}` to fail to parse, but it succeeded"))
        })
        .unwrap_err();
    result.message().to_string()
}

pub fn compile_and_run(source: &str) -> Result<rl_vm::VmValue, rl_vm::VmError> {
    let file = SourceFile::new("test", source.to_string());
    let tokens = rl_lexer::tokenizer::Tokenizer::lex(file.clone()).expect("lex failed");
    let (ast, stmts) =
        rl_parser::parser_logic::Parser::parse(tokens, file.clone()).expect("parse failed");
    let mut resolver = Resolver::new();
    let stmts = resolver.resolve_program(ast, stmts);
    let chunk = rl_vm::Compiler::new(&resolver.ast_arena)
        .compile(&stmts)
        .expect("compile failed");
    rl_vm::Vm::new().run_and_return(&chunk)
}

/// Compiles `source`, asserting that the VM compile step fails, and returns
/// the first error message so tests can assert on its contents.
pub fn compile_error(source: &str) -> String {
    let file = SourceFile::new("test", source.to_string());
    let tokens = rl_lexer::tokenizer::Tokenizer::lex(file.clone()).expect("lex failed");
    let (ast, stmts) =
        rl_parser::parser_logic::Parser::parse(tokens, file.clone()).expect("parse failed");
    let mut resolver = Resolver::new();
    let stmts = resolver.resolve_program(ast, stmts);
    rl_vm::Compiler::new(&resolver.ast_arena)
        .compile(&stmts)
        .map(|_| {
            panic!("expected `{source}` to fail compilation, but it succeeded");
        })
        .unwrap_err()
        .message()
        .to_string()
}

pub fn checker_messages(source: &str) -> Vec<String> {
    let (ast, stmts) = parse(source);
    let mut checker = TypeChecker::new().with_ast_arena(ast);
    checker
        .check(&stmts)
        .iter()
        .map(|e| e.message().to_string())
        .collect()
}

/// Runs the type checker over `source` and returns the populated
/// [`TypeChecker`] so tests can inspect scopes, units, and errors.
pub fn check(source: &str) -> TypeChecker {
    let file = SourceFile::new("test", source.to_string());
    let tokens = rl_lexer::tokenizer::Tokenizer::lex(file.clone()).expect("lex failed");
    let (ast, stmts) = rl_parser::parser_logic::Parser::parse(tokens, file).expect("parse failed");
    let mut checker = TypeChecker::new().with_ast_arena(ast);
    checker.check(&stmts);
    checker
}
