use crate::writer::CWriter;
use crate::types::type_to_c;
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
        self.emit_header(statements);

        let has_main = statements.iter().any(|s| {
            matches!(&s.kind,
                StatementKind::ResolvedFunctionDeclaration { name, .. }
                if name == "main" || name == "__entry__"
            )
        });

        // Hoist function declarations before main
        for stmt in statements {
            if matches!(&stmt.kind, StatementKind::ResolvedFunctionDeclaration { .. }) {
                self.compile_statement(stmt)?;
            }
        }

        if !has_main {
            self.is_script_mode = true;
            self.writer.write("int main(int argc, char **argv) {\n");
            self.writer.indent();
        }

        for stmt in statements {
            if !matches!(&stmt.kind,
                StatementKind::ResolvedFunctionDeclaration { .. }
                | StatementKind::RecordDeclaration { .. }
                | StatementKind::TagDeclaration { .. }
                | StatementKind::ResolvedImplBlock { .. }
                | StatementKind::ImplBlock { .. }
            ) {
                self.compile_statement(stmt)?;
            }
        }

        if !has_main {
            self.writer.writeln("return 0;");
            self.writer.dedent();
            self.writer.writeln("}");
        }

        Ok(self.writer.source().to_string())
    }

    fn emit_header(&mut self, statements: &[Statement]) {
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

        // Emit record typedefs from checker's resolved record info
        for (name, fields) in &self.checker.records {
            self.writer.write("typedef struct { ");
            for (field_name, field_type) in fields {
                let c_type = type_to_c(field_type);
                self.writer.write(&format!("{} {}; ", c_type, field_name));
            }
            self.writer.write(&format!("}} rl_Record_{};\n", name));
        }
        if !self.checker.records.is_empty() {
            self.writer.blank_line();
        }

        // Emit tag (enum) defines from checker's resolved tag info
        for (name, variants) in &self.checker.tags {
            for (i, variant) in variants.iter().enumerate() {
                let macro_name = format!(
                    "RL_TAG_{}_{}",
                    name.to_uppercase(),
                    variant.to_uppercase()
                );
                self.writer
                    .writeln(&format!("#define {} ((int64_t){})", macro_name, i));
            }
        }
        if !self.checker.tags.is_empty() {
            self.writer.blank_line();
        }

        // Collect tuple types used in the program
        let mut tuple_types: Vec<Vec<TypeAnnotation>> = Vec::new();
        self.collect_tuple_types(statements, &mut tuple_types);
        tuple_types.sort_by_key(|t| t.len());
        tuple_types.dedup_by_key(|t| t.len());
        for fields in &tuple_types {
            self.writer.write("typedef struct { ");
            for (i, field_type) in fields.iter().enumerate() {
                let c_type = type_to_c(field_type);
                self.writer.write(&format!("{} field_{}; ", c_type, i));
            }
            self.writer
                .writeln(&format!("}} rl_tuple_{};", fields.len()));
        }
        if !tuple_types.is_empty() {
            self.writer.blank_line();
        }
    }

    fn collect_tuple_types(&self, statements: &[Statement], types: &mut Vec<Vec<TypeAnnotation>>) {
        for stmt in statements {
            match &stmt.kind {
                StatementKind::ResolvedVariableDeclaration {
                    type_annotation, ..
                } => {
                    self.collect_types_from_type(type_annotation, types);
                }
                StatementKind::ResolvedConstantDeclaration {
                    type_annotation, ..
                } => {
                    self.collect_types_from_type(type_annotation, types);
                }
                StatementKind::ResolvedFunctionDeclaration {
                    params,
                    return_type,
                    body,
                    ..
                } => {
                    for param in params {
                        self.collect_types_from_type(&param.param_type, types);
                    }
                    self.collect_types_from_type(return_type, types);
                    self.collect_tuple_types(body, types);
                }
                _ => {}
            }
        }
    }

    fn collect_types_from_type(&self, ta: &TypeAnnotation, types: &mut Vec<Vec<TypeAnnotation>>) {
        match ta {
            TypeAnnotation::Tuple(elems) | TypeAnnotation::CTuple(elems) => {
                types.push(elems.as_ref().clone());
                for elem in elems.iter() {
                    self.collect_types_from_type(elem, types);
                }
            }
            TypeAnnotation::Array(inner) | TypeAnnotation::CArray(inner) => {
                self.collect_types_from_type(inner, types);
            }
            TypeAnnotation::Result(inner) | TypeAnnotation::CResult(inner) => {
                self.collect_types_from_type(inner, types);
            }
            _ => {}
        }
    }
}
