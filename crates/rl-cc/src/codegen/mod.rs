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
    pub var_types: HashMap<String, TypeAnnotation>,
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
            var_types: HashMap::new(),
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

        // Hoist function declarations and impl methods before main
        for stmt in statements {
            match &stmt.kind {
                StatementKind::ResolvedFunctionDeclaration { .. } => {
                    self.compile_statement(stmt)?;
                }
                StatementKind::ResolvedImplBlock { .. } => {
                    self.compile_statement(stmt)?;
                }
                _ => {}
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

        self.writer.writeln("#define _GNU_SOURCE");
        self.writer.writeln("#define _POSIX_C_SOURCE 200809L");
        self.writer.writeln("#include <stdint.h>");
        self.writer.writeln("#include <stdbool.h>");
        self.writer.writeln("#include <stdio.h>");
        self.writer.writeln("#include <stdlib.h>");
        self.writer.writeln("#include <string.h>");
        self.writer.writeln("#include \"rl_runtime.h\"");
        self.writer.blank_line();

        // Emit record typedefs and print functions
        for (name, fields) in &self.checker.records {
            self.writer.write("typedef struct { ");
            for (field_name, field_type) in fields {
                let c_type = type_to_c(field_type);
                self.writer.write(&format!("{} {}; ", c_type, field_name));
            }
            self.writer.write(&format!("}} rl_Record_{};\n", name));
            // Generate print function
            self.writer.write(&format!("void rl_print_rl_Record_{}(rl_Record_{} v) {{ ", name, name));
            self.writer.write("printf(\"Record(");
            for (i, (field_name, _)) in fields.iter().enumerate() {
                if i > 0 { self.writer.write(", "); }
                self.writer.write(&format!("{}: ", field_name));
                match fields[i].1 {
                    TypeAnnotation::Int | TypeAnnotation::CInt => self.writer.write("%ld"),
                    TypeAnnotation::Float | TypeAnnotation::CFloat => self.writer.write("%g"),
                    TypeAnnotation::Bool | TypeAnnotation::CBool => self.writer.write("%s"),
                    TypeAnnotation::String | TypeAnnotation::CString => self.writer.write("%.*s"),
                    TypeAnnotation::Char | TypeAnnotation::CChar => self.writer.write("%c"),
                    _ => self.writer.write("?"),
                }
            }
            self.writer.write(")\", ");
            for (i, (field_name, field_type)) in fields.iter().enumerate() {
                if i > 0 { self.writer.write(", "); }
                match field_type {
                    TypeAnnotation::Int | TypeAnnotation::CInt => self.writer.write(&format!("(long)v.{}", field_name)),
                    TypeAnnotation::Float | TypeAnnotation::CFloat => self.writer.write(&format!("v.{}", field_name)),
                    TypeAnnotation::Bool | TypeAnnotation::CBool => self.writer.write(&format!("v.{} ? \"true\" : \"false\"", field_name)),
                    TypeAnnotation::String | TypeAnnotation::CString => self.writer.write(&format!("(int)v.{}.len, v.{}.data", field_name, field_name)),
                    TypeAnnotation::Char | TypeAnnotation::CChar => self.writer.write(&format!("v.{}", field_name)),
                    _ => self.writer.write("\"?\""),
                }
            }
            self.writer.writeln(");");
            self.writer.writeln("}");
            self.writer.write(&format!("void rl_println_rl_Record_{}(rl_Record_{} v) {{ rl_print_rl_Record_{}(v); printf(\"\\n\"); }}\n", name, name, name));
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
            // Generate print function for tuple
            let arity = fields.len();
            self.writer.write(&format!("void rl_print_rl_tuple_{}(rl_tuple_{} v) {{ ", arity, arity));
            self.writer.write("printf(\"(");
            for (i, field_type) in fields.iter().enumerate() {
                if i > 0 { self.writer.write(", "); }
                match field_type {
                    TypeAnnotation::Int | TypeAnnotation::CInt => self.writer.write("%ld"),
                    TypeAnnotation::Float | TypeAnnotation::CFloat => self.writer.write("%g"),
                    TypeAnnotation::Bool | TypeAnnotation::CBool => self.writer.write("%s"),
                    TypeAnnotation::String | TypeAnnotation::CString => self.writer.write("%.*s"),
                    TypeAnnotation::Char | TypeAnnotation::CChar => self.writer.write("%c"),
                    _ => self.writer.write("?"),
                }
            }
            self.writer.write(")\", ");
            for (i, field_type) in fields.iter().enumerate() {
                if i > 0 { self.writer.write(", "); }
                match field_type {
                    TypeAnnotation::Int | TypeAnnotation::CInt => self.writer.write(&format!("(long)v.field_{}", i)),
                    TypeAnnotation::Float | TypeAnnotation::CFloat => self.writer.write(&format!("v.field_{}", i)),
                    TypeAnnotation::Bool | TypeAnnotation::CBool => self.writer.write(&format!("v.field_{} ? \"true\" : \"false\"", i)),
                    TypeAnnotation::String | TypeAnnotation::CString => self.writer.write(&format!("(int)v.field_{}.len, v.field_{}.data", i, i)),
                    TypeAnnotation::Char | TypeAnnotation::CChar => self.writer.write(&format!("v.field_{}", i)),
                    _ => self.writer.write("\"?\""),
                }
            }
            self.writer.writeln(");");
            self.writer.writeln("}");
            self.writer.write(&format!("void rl_println_rl_tuple_{}(rl_tuple_{} v) {{ rl_print_rl_tuple_{}(v); printf(\"\\n\"); }}\n", arity, arity, arity));
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
