use crate::writer::CWriter;
use rl_ast::{Ast, statements::*};
use rl_checker::structs::TypeChecker;
use std::collections::HashMap;

pub mod expressions;
pub mod ops;
pub mod scope;
pub mod statements;

pub struct CCodegen<'a> {
    pub ast: &'a Ast,
    pub checker: &'a TypeChecker,
    pub writer: CWriter,
    pub scopes: Vec<HashMap<String, String>>,
    pub emitted_includes: bool,
    pub is_script_mode: bool,
    pub temp_counter: usize,
}

impl<'a> CCodegen<'a> {
    pub fn new(ast: &'a Ast, checker: &'a TypeChecker) -> Self {
        Self {
            ast,
            checker,
            writer: CWriter::new(),
            scopes: vec![HashMap::new()],
            emitted_includes: false,
            is_script_mode: false,
            temp_counter: 0,
        }
    }

    pub fn emit_program(&mut self, statements: &[Statement]) -> Result<String, rl_utils::errors::Error> {
        self.emit_header();

        let has_main = statements.iter().any(|s| {
            matches!(&s.kind,
                StatementKind::ResolvedFunctionDeclaration { name, .. }
                if name == "main" || name == "__entry__"
            )
        });

        if !has_main {
            self.is_script_mode = true;
            self.writer.writeln("int main(int argc, char **argv) {");
            self.writer.indent();
        }

        for stmt in statements {
            self.compile_statement(stmt)?;
        }

        if !has_main {
            self.writer.writeln("return 0;");
            self.writer.dedent();
            self.writer.writeln("}");
        }

        Ok(self.writer.source().to_string())
    }

    fn emit_header(&mut self) {
        if self.emitted_includes {
            return;
        }
        self.emitted_includes = true;

        self.writer.writeln("#include <stdint.h>");
        self.writer.writeln("#include <stdbool.h>");
        self.writer.writeln("#include <stdio.h>");
        self.writer.writeln("#include <stdlib.h>");
        self.writer.writeln("#include <string.h>");
        self.writer.writeln("#include \"rl_runtime.h\"");
        self.writer.blank_line();
    }
}
