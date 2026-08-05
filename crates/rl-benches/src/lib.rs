//! Shared fixtures for the `rl-benches` targets.
//!
//! Each `benches/*.rs` file compiles as its own binary, so the source
//! snippets and pipeline-stage helpers live here once and get imported
//! with `use rl_benches::*;` instead of being duplicated per file.

use criterion::black_box;
use rl_ast::statements::Statement;
use rl_interpreter::evaluator::Evaluator;
use rl_lexer::tokenizer::Tokenizer;
use rl_parser::parser_logic::Parser;
use rl_resolver::Resolver;
use rl_utils::source::SourceFile;
use rl_vm::{Chunk, Compiler, Vm};

// ===== source snippets =====

// --- declarations ---
pub const SRC_DEC_INT: &str = "dec int x = 0";
pub const SRC_CONST_INT: &str = "CONST int x = 0";
pub const SRC_DEC_FLOAT: &str = "dec float x = 0.0";
pub const SRC_CONST_FLOAT: &str = "CONST float x = 0.0";
pub const SRC_DEC_STRING: &str = r#"dec string x = "hi""#;
pub const SRC_CONST_STRING: &str = r#"CONST string x = "hi""#;
pub const SRC_DEC_BOOL: &str = "dec bool x = true";
pub const SRC_CONST_BOOL: &str = "CONST bool x = false";
pub const SRC_DEC_CHAR: &str = "dec char x = 'x'";
pub const SRC_CONST_CHAR: &str = "CONST char x = 'x'";
pub const SRC_DEC_ARRAY: &str = "dec arr[int] x = [1, 2, 3]";
pub const SRC_CONST_ARRAY: &str = "CONST arr[int] x = [1, 2, 3]";
pub const SRC_DEC_FN: &str = "dec fn x = fn() {}";
pub const SRC_CONST_FN: &str = "CONST fn x = fn() {}";
pub const SRC_DEC_TUPLE: &str = r#"dec (int, string) x = (42, "hello")"#;
pub const SRC_DEC_ERROR: &str = r#"dec error e = error("oops")"#;
pub const SRC_DEC_RESULT_OK: &str = "dec result[int] r = ok(42)";
pub const SRC_DEC_RESULT_ERR: &str = r#"dec result[int] r = err("oops")"#;

// --- control flow ---
pub const SRC_WHILE: &str = "while true { 0 }";
pub const SRC_IF_SIMPLE: &str = "if true { 0 }";
pub const SRC_IF_ELSE: &str = "if true { 1 } else { 0 }";
pub const SRC_IF_ELSE_IF: &str = "if true { 1 } else if false { 2 }";
pub const SRC_IF_ELSE_IF_ELSE: &str = "if true { 1 } else if false { 2 } else { 0 }";
pub const SRC_FOR_C: &str = "for [dec int i = 0, i < 10, i += 1] { 0 }";
pub const SRC_FOR_RANGE: &str = "for i in 0..10 { 0 }";
pub const SRC_FOR_EACH: &str = "for i in [1, 2, 3, 4, 5] { 0 }";

// --- functions ---
pub const SRC_FN_SIMPLE: &str = "fn add(int a, int b) -> int { return a + b }";
pub const SRC_FN_FN_PARAM: &str = "fn apply(fn f, int x) -> int { return f(x) }";
pub const SRC_FN_RECURSIVE: &str =
    "fn fact(int n) -> int { if n <= 1 { return 1 } return n * fact(n - 1) }";
pub const SRC_FN_RESULT_RETURN: &str = "fn safe_div(int a, int b) -> result[int] { if b == 0 { return err(\"div by zero\") } return ok(a / b) }";
pub const SRC_LAMBDA: &str = "dec fn f = fn(int n) -> int { return n * 2 }";

// --- imports ---
pub const SRC_IMPORT_SIMPLE: &str = "get println from std::io";
pub const SRC_IMPORT_MULTI: &str = "get abs, min, max from std::math";
pub const SRC_IMPORT_RESULT: &str = "get is_ok, is_err, result_unwrap from std::res";

// --- programs ---
pub const SRC_PROGRAM_RESULT_CHAIN: &str = "\
get is_ok, is_err, result_unwrap, result_unwrap_or, result_map from std::res
fn safe_div(int a, int b) -> result[int] {
    if b == 0 {
        return err(\"division by zero\")
    }
    return ok(a / b)
}
dec result[int] r1 = safe_div(10, 2)
dec result[int] r2 = safe_div(10, 0)
dec result[int] r3 = result_map(r1, fn(int n) -> int { return n * 2 })
dec int v1 = result_unwrap(r1)
dec int v2 = result_unwrap_or(r2, -1)
dec bool b1 = is_ok(r3)
dec bool b2 = is_err(r2)";

pub const SRC_PROGRAM_ARR_ZIP: &str = "\
get arr_zip, arr_map, arr_filter, arr_reduce from std::array
dec arr[int] xs = [1, 2, 3, 4, 5]
dec arr[int] ys = [10, 20, 30, 40, 50]
dec arr[(int, int)] pairs = arr_zip(xs, ys)?
dec arr[int] sums = arr_map(xs, fn(int n) -> int { return n * 2 })?
dec arr[int] evens = arr_filter(xs, fn(int n) -> bool { return n > 2 })?
dec int total = arr_reduce(xs, fn(int acc, int n) -> int { return acc + n }, 0)?";

pub const SRC_PROGRAM_FIBONACCI: &str = "\
fn fib(int n) -> int {
    if n <= 1 {
        return n
    }
    return fib(n - 1) + fib(n - 2)
}
dec int a = fib(10)
dec int b = fib(15)
dec int c = fib(20)";

pub const SRC_PROGRAM_TUPLE_DESTRUCTURE: &str = r#"
dec (int, string, bool) t = (42, "hello", true)
dec int a = t[0]
dec string b = t[1]
dec bool c = t[2]
dec int x, string y, bool z = (1, "world", false)
"#;

pub const SRC_PROGRAM_CLOSURE: &str = "\
dec int base = 100
dec fn add_base = fn(int n) -> int { return n + base }
dec fn mul_base = fn(int n) -> int { return n * base }
dec int r1 = add_base(5)
dec int r2 = mul_base(3)
dec int r3 = add_base(mul_base(2))";

// --- realistic workloads ---
// Deep recursion: fib(26) = 121393 (naive fib does ~2*fib(n) calls).
pub const SRC_PROGRAM_RECURSION: &str = "\
fn fib(int n) -> int {
    if n <= 1 {
        return n
    }
    return fib(n - 1) + fib(n - 2)
}
fib(26)";

// String building: 300 concatenations into a growing string, then count.
pub const SRC_PROGRAM_STRING_BUILD: &str = "\
get concat, count from std::str
dec string s = \"\"
for i in 0..300 {
    s = concat(s, \"xy\")
}
count(s, \"x\")";

// Array pipeline: build 1000 elements, map to doubles, filter, reduce.
// sum of evens 1002..=2000 = (1002 + 2000) / 2 * 500 = 750500.
pub const SRC_PROGRAM_ARRAY_PIPELINE: &str = "\
get arr_range, arr_map, arr_filter, arr_reduce from std::array
dec arr[int] xs = arr_range(1, 1001, 1)?
dec arr[int] doubled = arr_map(xs, fn(int n) -> int { return n * 2 })?
dec arr[int] evens = arr_filter(doubled, fn(int n) -> bool { return n > 1000 })?
arr_reduce(evens, fn(int acc, int n) -> int { return acc + n }, 0)?";

// Map + set churn: grow a set and a map in loops, then total their sizes.
pub const SRC_PROGRAM_MAP_SET: &str = "\
get set_add, set_len from std::collections
get map_merge, map_len from std::collections
dec set[int] s = {}
for i in 0..300 {
    s = set_add(s, i)?
}
dec map[int, int] m = {}
for i in 0..300 {
    m = map_merge(m, {i: i * 2})?
}
set_len(s)? + map_len(m)?";

// Closures capturing an outer variable, called in a hot loop.
pub const SRC_PROGRAM_CLOSURE_MUTATION: &str = "\
dec int base = 1
dec fn step = fn(int n) -> int { return n + base }
dec int acc = 0
for i in 0..1000 {
    acc = step(acc)
}
acc";

// Records + impl methods in a hot loop.
// sum over i=0..499 of (2i + 1) = 2*sum(0..499) + 500 = 250000.
pub const SRC_PROGRAM_RECORD: &str = "\
record Point {
    int x,
    int y,
}

impl Point {
    fn sum(self) -> int {
        return self.x + self.y
    }
}

dec int total = 0
dec Point p = Point { x: 0, y: 1 }
for i in 0..500 {
    p = Point { x: i, y: i + 1 }
    total = total + p.sum()
}
total";

/// (name, source) pairs for the small original programs. These end in
/// declarations, so they are smoke-verified (both engines run cleanly)
/// rather than asserted to a final value.
pub const BASE_PROGRAMS: &[(&str, &str)] = &[
    ("result_chain", SRC_PROGRAM_RESULT_CHAIN),
    ("arr_zip", SRC_PROGRAM_ARR_ZIP),
    ("fibonacci", SRC_PROGRAM_FIBONACCI),
    ("tuple_destructure", SRC_PROGRAM_TUPLE_DESTRUCTURE),
    ("closure", SRC_PROGRAM_CLOSURE),
];

/// (name, source, expected final VM value) pairs for the realistic
/// workloads. Each fixture ends in a bare expression whose value is known,
/// so these are strictly verified (asserted, not just smoke-run).
pub const WORKLOAD_PROGRAMS: &[(&str, &str, rl_vm::VmValue)] = &[
    ("recursion", SRC_PROGRAM_RECURSION, rl_vm::VmValue::Int(121393)),
    ("string_build", SRC_PROGRAM_STRING_BUILD, rl_vm::VmValue::Int(300)),
    (
        "array_pipeline",
        SRC_PROGRAM_ARRAY_PIPELINE,
        rl_vm::VmValue::Int(750500),
    ),
    ("map_set", SRC_PROGRAM_MAP_SET, rl_vm::VmValue::Int(600)),
    (
        "closure_mutation",
        SRC_PROGRAM_CLOSURE_MUTATION,
        rl_vm::VmValue::Int(1000),
    ),
    ("record", SRC_PROGRAM_RECORD, rl_vm::VmValue::Int(250000)),
];

// ===== pipeline-stage helpers =====

pub fn src(name: &str, text: &str) -> SourceFile {
    SourceFile::new(name, text.to_string())
}

pub fn lex_only(text: &str) {
    let _ = Tokenizer::lex(src("bench", black_box(text)));
}

pub fn lex_and_parse(text: &str) {
    let sf = src("bench", black_box(text));
    let tokens = match Tokenizer::lex(sf.clone()) {
        Ok(t) => t,
        Err(_) => return,
    };
    let _ = Parser::parse(tokens, sf);
}

/// A parsed and resolved program. The `resolver` owns the single expression
/// arena (`ast_arena`) that the VM compiler needs to compile the statements.
pub struct ResolvedProgram {
    pub resolver: Resolver,
    pub statements: Vec<Statement>,
}

/// Parses and resolves `text`, panicking loudly if any stage fails so a
/// broken fixture can never be silently benchmarked as empty work.
pub fn parse_and_resolve(text: &str) -> ResolvedProgram {
    let sf = src("bench", black_box(text));
    let tokens = Tokenizer::lex(sf.clone()).expect("bench fixture failed to lex");
    let (ast, stmts) = Parser::parse(tokens, sf).expect("bench fixture failed to parse");
    let mut resolver = Resolver::new();
    let statements = resolver.resolve_program(ast, stmts);
    ResolvedProgram {
        resolver,
        statements,
    }
}

/// Compiles an already-resolved program into a VM chunk.
pub fn compile_resolved(program: &ResolvedProgram) -> Chunk {
    Compiler::new(&program.resolver.ast_arena)
        .compile(&program.statements)
        .expect("bench fixture failed to compile")
}

/// Runs a compiled chunk on a fresh VM (the dispatch loop mutates
/// `stack`/`locals`, so each run needs its own VM).
pub fn run_chunk(chunk: &Chunk) {
    let mut vm = Vm::new();
    let _ = vm.run_and_return(chunk).expect("bench fixture failed to run");
}

/// Full resolve stage only (lex + parse + resolve).
pub fn resolve_only(text: &str) {
    let _ = parse_and_resolve(text);
}

/// Full interpreter pipeline (lex + parse + resolve + evaluate).
pub fn interp_evaluate_only(text: &str) {
    let sf = src("bench", black_box(text));
    let tokens = Tokenizer::lex(sf.clone()).expect("bench fixture failed to lex");
    let (ast, stmts) = Parser::parse(tokens, sf.clone()).expect("bench fixture failed to parse");
    let mut ev = Evaluator::default().with_stdlib().with_source_file(sf);
    let stmts = ev.resolver.resolve_program(ast, stmts);
    ev.evaluate_program(&stmts).expect("bench fixture failed to evaluate");
}

// ===== self-validation =====

/// Verifies a `BASE_PROGRAMS` fixture runs cleanly through BOTH the VM and
/// the interpreter. Called once before benchmarking so a broken fixture
/// fails the bench loudly instead of being silently skipped.
pub fn verify_base_program(text: &str) {
    let program = parse_and_resolve(text);
    let chunk = compile_resolved(&program);
    run_chunk(&chunk);
    interp_evaluate_only(text);
}

/// Verifies a `WORKLOAD_PROGRAMS` fixture produces exactly `expected` when
/// run on the VM, and that the interpreter can run it cleanly too.
pub fn verify_workload(text: &str, expected: rl_vm::VmValue) {
    let program = parse_and_resolve(text);
    let chunk = compile_resolved(&program);
    let mut vm = Vm::new();
    let got = vm
        .run_and_return(&chunk)
        .expect("bench fixture failed to run");
    assert_eq!(got, expected, "bench fixture produced the wrong result");
    interp_evaluate_only(text);
}
