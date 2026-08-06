//! Backend abstraction for the REPL.
//!
//! The REPL UI code (`logic_loop`, `command_handler`, `completion`) only ever
//! touches [`ReplBackend`], so rl can drive the same interactive loop from
//! either execution engine:
//!
//! - [`TreewalkerBackend`] wraps the interpreter's `Evaluator` (enabled with
//!   the `treewalker` feature).
//! - [`VmBackend`] wraps a persistent `rl_vm::Vm` + `rl_resolver::Resolver`
//!   (enabled with the `vm` feature). Global state survives across inputs
//!   because the resolver keeps its global scope and the VM keeps its global
//!   slots; each input is resolved, compiled (seeded with the current global
//!   slot count), and run via `run_and_return`.

use std::path::Path;

#[cfg(feature = "vm")]
use std::collections::HashSet;

use rl_ast::{Ast, statements::Statement};
use rl_utils::source::SourceFile;

use crate::lines_types::OutputLine;
use crate::utils::push_error;

/// What the REPL needs from an execution engine: evaluate inputs, attach
/// files, reset, and report bound names for tab-completion.
pub trait ReplBackend {
    /// Evaluates an already-parsed input (statements from one submission),
    /// appending rendered values, captured `print` output, and errors to
    /// `output`. Returns `true` if every statement evaluated without error.
    fn eval_parsed(
        &mut self,
        source: SourceFile,
        ast: Ast,
        statements: Vec<Statement>,
        output: &mut Vec<OutputLine>,
    ) -> bool;

    /// Evaluates an already-parsed file into the environment (the `:attach`
    /// command). Does not render expression values. Returns `true` on success.
    fn attach_parsed(
        &mut self,
        source: SourceFile,
        ast: Ast,
        statements: Vec<Statement>,
        output: &mut Vec<OutputLine>,
    ) -> bool;

    /// Resets to a fresh environment, dropping all user state.
    fn reset(&mut self);

    /// Names currently bound in the environment, for tab-completion.
    fn candidate_names(&self) -> Vec<String>;
}

/// The interpreter-backed REPL backend.
#[cfg(feature = "treewalker")]
pub struct TreewalkerBackend {
    evaluator: rl_interpreter::evaluator::Evaluator,
}

#[cfg(feature = "treewalker")]
impl TreewalkerBackend {
    pub fn new() -> Self {
        Self {
            evaluator: rl_interpreter::evaluator::Evaluator::default().with_stdlib(),
        }
    }
}

#[cfg(feature = "treewalker")]
impl ReplBackend for TreewalkerBackend {
    fn eval_parsed(
        &mut self,
        source: SourceFile,
        ast: Ast,
        statements: Vec<Statement>,
        output: &mut Vec<OutputLine>,
    ) -> bool {
        use rl_ast::statements::StatementKind;
        use rl_interpreter::values::Value;

        self.evaluator.set_source_file(source);
        self.evaluator.output_buffer = Some(String::new());

        let statements = self.evaluator.resolver.resolve_program(ast, statements);

        let mut success = true;
        for statement in &statements {
            if let StatementKind::Expression(expr) = &statement.kind {
                match self.evaluator.evaluate(*expr) {
                    Ok(val) => {
                        if !matches!(val, Value::Null) {
                            let val_str = format!("{}", val);
                            let spans = crate::syntax_highlighting::highlight(&val_str);
                            output.push(OutputLine::Styled(
                                spans
                                    .into_iter()
                                    .map(|sp| (sp.content.into_owned(), sp.style))
                                    .collect(),
                            ));
                        }
                    }
                    Err(e) => {
                        output.push(OutputLine::Error(format!("error: {}", e.message())));
                        success = false;
                        break;
                    }
                }
            } else if let Err(e) = self.evaluator.evaluate_statement(statement) {
                output.push(OutputLine::Error(format!("error: {}", e.message())));
                success = false;
                break;
            }
        }

        if let Some(captured) = self.evaluator.output_buffer.take() {
            for line in captured.split('\n') {
                if !line.is_empty() {
                    output.push(OutputLine::Result(line.to_string()));
                }
            }
        }

        success
    }

    fn attach_parsed(
        &mut self,
        source: SourceFile,
        ast: Ast,
        statements: Vec<Statement>,
        output: &mut Vec<OutputLine>,
    ) -> bool {
        let dir = Path::new(source.name.as_ref())
            .parent()
            .unwrap_or(Path::new(""))
            .to_path_buf();
        self.evaluator.set_source_file(source);
        self.evaluator.resolver.current_dir = dir;
        let statements = self.evaluator.resolver.resolve_program(ast, statements);
        let mut ok = true;
        for stmt in &statements {
            if let Err(e) = self.evaluator.evaluate_statement(stmt) {
                push_error(output, &e);
                ok = false;
                break;
            }
        }
        ok
    }

    fn reset(&mut self) {
        self.evaluator = rl_interpreter::evaluator::Evaluator::default().with_stdlib();
    }

    fn candidate_names(&self) -> Vec<String> {
        let mut names: Vec<String> = Vec::new();
        names.extend(self.evaluator.fn_names.keys().cloned());
        names.extend(self.evaluator.records.keys().cloned());
        names.extend(self.evaluator.tags.keys().cloned());
        names
    }
}

/// The bytecode VM-backed REPL backend.
///
/// A single persistent `Vm` + `Resolver` lives for the whole session. Each
/// input is lexed/parsed by the shared REPL code, then resolved against the
/// persistent resolver (so global slots keep growing), compiled with a
/// `Compiler` seeded at the current global slot count and the accumulated
/// stdlib module (so `get x from std::io` imports survive across inputs),
/// and executed with `run_and_return` so a trailing expression's value is
/// rendered.
#[cfg(feature = "vm")]
pub struct VmBackend {
    vm: rl_vm::Vm,
    resolver: rl_resolver::Resolver,
    stdlib: rl_vm::Module,
    /// `record`/`tag` type names, which the resolver does not put in its
    /// global scope but which tab-completion should still offer.
    types: HashSet<String>,
}

#[cfg(feature = "vm")]
impl VmBackend {
    pub fn new() -> Self {
        Self {
            vm: rl_vm::Vm::new(),
            resolver: rl_resolver::Resolver::new(),
            stdlib: rl_vm::stdlib::root(),
            types: HashSet::new(),
        }
    }

    /// Resolves and compiles `statements`, appending any compile error to
    /// `output`. Returns the compiled chunk on success.
    fn compile(
        &mut self,
        source: SourceFile,
        ast: Ast,
        statements: Vec<Statement>,
        output: &mut Vec<OutputLine>,
    ) -> Option<rl_vm::Chunk> {
        use rl_ast::statements::StatementKind;

        let base = self.resolver.global_slot_count() as u16;
        let statements = self.resolver.resolve_program(ast, statements);

        let mut new_types: Vec<String> = Vec::new();
        for stmt in &statements {
            match &stmt.kind {
                StatementKind::RecordDeclaration { name, .. } => new_types.push(name.clone()),
                StatementKind::TagDeclaration { name, .. } => new_types.push(name.clone()),
                _ => {}
            }
        }

        let mut compiler = rl_vm::Compiler::new(&self.resolver.ast_arena)
            .with_source_file(source.clone())
            .with_stdlib(self.stdlib.clone())
            .with_global_slot_base(base);

        match compiler.compile(&statements) {
            Ok(chunk) => {
                self.stdlib = compiler.stdlib().clone();
                self.types.extend(new_types);
                Some(chunk)
            }
            Err(e) => {
                // The chunk never ran, so the globals declared during
                // resolution were never set in the VM. Roll the global scope
                // back so the resolver's slot count stays in sync with what
                // actually executed.
                self.resolver.truncate_global_scope(base as usize);
                push_error(output, &e);
                None
            }
        }
    }

    /// Runs `chunk`, appending captured `print` output (when `capture_print`)
    /// and a rendered trailing value (when `render_value`) to `output`.
    /// Returns `true` if execution succeeded.
    fn run(
        &mut self,
        chunk: &rl_vm::Chunk,
        render_value: bool,
        capture_print: bool,
        output: &mut Vec<OutputLine>,
    ) -> bool {
        if capture_print {
            self.vm.output_buffer = Some(String::new());
        }

        match self.vm.run_and_return(chunk) {
            Ok(value) => {
                if render_value && !matches!(value, rl_vm::VmValue::Null) {
                    let val_str = value.to_string();
                    let spans = crate::syntax_highlighting::highlight(&val_str);
                    output.push(OutputLine::Styled(
                        spans
                            .into_iter()
                            .map(|sp| (sp.content.into_owned(), sp.style))
                            .collect(),
                    ));
                }
                if let Some(captured) = self.vm.output_buffer.take() {
                    for line in captured.split('\n') {
                        if !line.is_empty() {
                            output.push(OutputLine::Result(line.to_string()));
                        }
                    }
                }
                true
            }
            Err(e) => {
                self.vm.reset_transient();
                let _ = self.vm.output_buffer.take();
                push_error(output, &e);
                false
            }
        }
    }
}

#[cfg(feature = "vm")]
impl ReplBackend for VmBackend {
    fn eval_parsed(
        &mut self,
        source: SourceFile,
        ast: Ast,
        statements: Vec<Statement>,
        output: &mut Vec<OutputLine>,
    ) -> bool {
        self.vm.set_source_file(source.clone());
        match self.compile(source, ast, statements, output) {
            Some(chunk) => self.run(&chunk, true, true, output),
            None => false,
        }
    }

    fn attach_parsed(
        &mut self,
        source: SourceFile,
        ast: Ast,
        statements: Vec<Statement>,
        output: &mut Vec<OutputLine>,
    ) -> bool {
        self.resolver.current_dir = Path::new(source.name.as_ref())
            .parent()
            .unwrap_or(Path::new(""))
            .to_path_buf();
        self.vm.set_source_file(source.clone());
        match self.compile(source, ast, statements, output) {
            Some(chunk) => self.run(&chunk, false, false, output),
            None => false,
        }
    }

    fn reset(&mut self) {
        self.vm = rl_vm::Vm::new();
        self.resolver = rl_resolver::Resolver::new();
        self.stdlib = rl_vm::stdlib::root();
        self.types.clear();
    }

    fn candidate_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.resolver.global_names().to_vec();
        names.extend(self.types.iter().cloned());
        names
    }
}
