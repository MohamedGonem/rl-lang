use crate::writer::CWriter;
use crate::types::type_to_c;
use rl_ast::{Ast, statements::*};
use rl_checker::structs::TypeChecker;
use std::collections::{HashMap, HashSet};

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
    pub lambda_counter: usize,
    pub static_funcs: Vec<String>,
    pub closure_params: Vec<String>,
    pub closure_return_types: HashMap<String, TypeAnnotation>,
    pub tuple_names: Vec<(Vec<TypeAnnotation>, String)>,
    pub nullable_vars: HashSet<String>,
    pub std_c_imports: HashSet<String>,
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
            lambda_counter: 0,
            static_funcs: Vec::new(),
            closure_params: Vec::new(),
            closure_return_types: HashMap::new(),
            tuple_names: Vec::new(),
            nullable_vars: HashSet::new(),
            std_c_imports: HashSet::new(),
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
            self.writer.writeln("rl_store_args(argc, argv);");
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

        // Combine: insert static lambda functions at file scope
        let mut output = self.writer.source().to_string();
        if !self.static_funcs.is_empty() {
            let static_funcs_str: String = self.static_funcs.iter().cloned().collect();
            // Insert right after the #include "rl_runtime.h" line
            if let Some(pos) = output.find("#include \"rl_runtime.h\"") {
                let insert_pos = pos + "#include \"rl_runtime.h\"".len();
                // Skip past the newline after the include
                let insert_pos = if output.as_bytes().get(insert_pos) == Some(&b'\n') {
                    insert_pos + 1
                } else {
                    insert_pos
                };
                output.insert_str(insert_pos, &format!("\n{}\n", static_funcs_str));
            } else {
                output.push_str(&static_funcs_str);
            }
        }

        Ok(output)
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
            self.writer.writeln("printf(\"Record(\");");
            for (i, (field_name, field_type)) in fields.iter().enumerate() {
                if i > 0 {
                    self.writer.writeln("printf(\", \");");
                }
                self.writer.write_indent();
                self.writer.write(&format!("printf(\"{}: \");\n", field_name));
                self.emit_field_print(field_type, &format!("v.{}", field_name));
            }
            self.writer.writeln("printf(\")\");");
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
            // Generate string table and print function for this enum
            let count = variants.len();
            self.writer.write(&format!("static const char* _enum_{}_names[] = {{", name));
            for (i, variant) in variants.iter().enumerate() {
                if i > 0 { self.writer.write(", "); }
                self.writer.write(&format!("\"{}\"", variant));
            }
            self.writer.writeln("};");
            self.writer.write(&format!("void rl_print_Enum_{}(int64_t v) {{ ", name));
            self.writer.writeln(&format!("if (v >= 0 && v < (int64_t){}) printf(\"%s.%s\", \"{}\", _enum_{}_names[v]);", count, name, name));
            self.writer.writeln(&format!("else printf(\"{}(%ld)\", (long)v);", name));
            self.writer.writeln("}");
            self.writer.write(&format!("void rl_println_Enum_{}(int64_t v) {{ rl_print_Enum_{}(v); printf(\"\\n\"); }}\n", name, name));
        }
        if !self.checker.tags.is_empty() {
            self.writer.blank_line();
        }

        // Collect tuple types used in the program
        let mut tuple_types: Vec<Vec<TypeAnnotation>> = Vec::new();
        self.collect_tuple_types(statements, &mut tuple_types);
        // Dedup by full field-type layout, not just arity
        let mut unique_tuples: Vec<Vec<TypeAnnotation>> = Vec::new();
        for tt in &tuple_types {
            if !unique_tuples.contains(tt) {
                unique_tuples.push(tt.clone());
            }
        }
        // Assign names: one per arity gets rl_tuple_N, multiple get rl_tuple_N_K
        let mut arity_count: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for fields in &unique_tuples {
            *arity_count.entry(fields.len()).or_insert(0) += 1;
        }
        let mut arity_index: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        let mut new_tuple_names: Vec<(Vec<TypeAnnotation>, String)> = Vec::new();
        for fields in &unique_tuples {
            let arity = fields.len();
            let count = arity_count[&arity];
            let idx = arity_index.entry(arity).or_insert(0);
            let name = if count == 1 {
                format!("rl_tuple_{}", arity)
            } else {
                format!("rl_tuple_{}_{}", arity, *idx)
            };
            *idx += 1;
            new_tuple_names.push((fields.clone(), name));
        }
        self.tuple_names = new_tuple_names;
        for (fields, name) in self.tuple_names.clone() {
            self.writer.write("typedef struct { ");
            for (i, field_type) in fields.iter().enumerate() {
                let c_type = type_to_c(field_type);
                self.writer.write(&format!("{} field_{}; ", c_type, i));
            }
            self.writer
                .writeln(&format!("}} {};", name));
            // Generate print function for tuple
            self.writer.write(&format!("void rl_print_{}({} v) {{ ", name, name));
            self.writer.writeln("printf(\"(\");");
            for (i, field_type) in fields.iter().enumerate() {
                if i > 0 {
                    self.writer.writeln("printf(\", \");");
                }
                self.emit_field_print(field_type, &format!("v.field_{}", i));
            }
            self.writer.writeln("printf(\")\");");
            self.writer.writeln("}");
            self.writer.write(&format!("void rl_println_{}({} v) {{ rl_print_{}(v); printf(\"\\n\"); }}\n", name, name, name));
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

    pub fn emit_value_wrapping(&mut self, ta: &TypeAnnotation, expr_id: rl_ast::ExprId) -> Result<(), rl_utils::errors::Error> {
        match ta {
            TypeAnnotation::Int | TypeAnnotation::CInt
            | TypeAnnotation::UInt | TypeAnnotation::CUInt
            | TypeAnnotation::SInt | TypeAnnotation::CSInt
            | TypeAnnotation::SUInt | TypeAnnotation::CSUInt
            | TypeAnnotation::Byte | TypeAnnotation::CByte
            | TypeAnnotation::SByte | TypeAnnotation::CSByte
            | TypeAnnotation::BByte | TypeAnnotation::CBByte
            | TypeAnnotation::BSByte | TypeAnnotation::CBSByte => {
                self.writer.write("(rl_value){ .tag = RL_VTAG_I64, .data.i64 = ");
                self.compile_expr(expr_id)?;
                self.writer.write(" }");
            }
            TypeAnnotation::Float | TypeAnnotation::CFloat
            | TypeAnnotation::SFloat | TypeAnnotation::CSFloat => {
                self.writer.write("(rl_value){ .tag = RL_VTAG_F64, .data.f64 = ");
                self.compile_expr(expr_id)?;
                self.writer.write(" }");
            }
            TypeAnnotation::Bool | TypeAnnotation::CBool => {
                self.writer.write("(rl_value){ .tag = RL_VTAG_BOOL, .data.boolean = ");
                self.compile_expr(expr_id)?;
                self.writer.write(" }");
            }
            TypeAnnotation::String | TypeAnnotation::CString => {
                self.writer.write("(rl_value){ .tag = RL_VTAG_STR, .data.str = ");
                self.compile_expr(expr_id)?;
                self.writer.write(" }");
            }
            TypeAnnotation::Char | TypeAnnotation::CChar => {
                self.writer.write("(rl_value){ .tag = RL_VTAG_I64, .data.i64 = (int64_t)(unsigned char)");
                self.compile_expr(expr_id)?;
                self.writer.write(" }");
            }
            TypeAnnotation::Array(_) | TypeAnnotation::CArray(_) => {
                self.writer.write("(rl_value){ .tag = RL_VTAG_ARR, .data.arr = ");
                self.compile_expr(expr_id)?;
                self.writer.write(" }");
            }
            TypeAnnotation::Map(_, _) | TypeAnnotation::CMap(_, _) => {
                self.writer.write("(rl_value){ .tag = RL_VTAG_MAP, .data.map = &");
                self.compile_expr(expr_id)?;
                self.writer.write(" }");
            }
            TypeAnnotation::Set(_) | TypeAnnotation::CSet(_) => {
                self.writer.write("(rl_value){ .tag = RL_VTAG_SET, .data.set = &");
                self.compile_expr(expr_id)?;
                self.writer.write(" }");
            }
            TypeAnnotation::Fn | TypeAnnotation::Callback(_, _) => {
                self.writer.write("(rl_value){ .tag = RL_VTAG_CLOSURE, .data.closure = &");
                self.compile_expr(expr_id)?;
                self.writer.write(" }");
            }
            _ => {
                self.writer.write("(rl_value){ .tag = RL_VTAG_I64, .data.i64 = (int64_t)");
                self.compile_expr(expr_id)?;
                self.writer.write(" }");
            }
        }
        Ok(())
    }

    fn emit_field_print(&mut self, field_type: &TypeAnnotation, accessor: &str) {
        match field_type {
            TypeAnnotation::Int | TypeAnnotation::CInt
            | TypeAnnotation::UInt | TypeAnnotation::CUInt
            | TypeAnnotation::SInt | TypeAnnotation::CSInt
            | TypeAnnotation::SUInt | TypeAnnotation::CSUInt
            | TypeAnnotation::Byte | TypeAnnotation::CByte
            | TypeAnnotation::SByte | TypeAnnotation::CSByte
            | TypeAnnotation::BByte | TypeAnnotation::CBByte
            | TypeAnnotation::BSByte | TypeAnnotation::CBSByte => {
                self.writer.write_indent();
                self.writer.writeln(&format!("printf(\"%ld\", (long){});", accessor));
            }
            TypeAnnotation::Float | TypeAnnotation::CFloat
            | TypeAnnotation::SFloat | TypeAnnotation::CSFloat => {
                self.writer.write_indent();
                self.writer.writeln(&format!("printf(\"%g\", (double){});", accessor));
            }
            TypeAnnotation::Bool | TypeAnnotation::CBool => {
                self.writer.write_indent();
                self.writer.writeln(&format!("printf(\"%s\", {} ? \"true\" : \"false\");", accessor));
            }
            TypeAnnotation::String | TypeAnnotation::CString => {
                self.writer.write_indent();
                self.writer.writeln(&format!("printf(\"%.*s\", (int){}.len, {}.data);", accessor, accessor));
            }
            TypeAnnotation::Char | TypeAnnotation::CChar => {
                self.writer.write_indent();
                self.writer.writeln(&format!("printf(\"%c\", (int){});", accessor));
            }
            TypeAnnotation::Array(_) | TypeAnnotation::CArray(_) => {
                self.writer.write_indent();
                self.writer.writeln(&format!("rl_print_rl_array({});", accessor));
            }
            TypeAnnotation::Map(_, _) | TypeAnnotation::CMap(_, _) => {
                self.writer.write_indent();
                self.writer.writeln(&format!("rl_print_rl_map({});", accessor));
            }
            TypeAnnotation::Set(_) | TypeAnnotation::CSet(_) => {
                self.writer.write_indent();
                self.writer.writeln(&format!("rl_print_rl_set({});", accessor));
            }
            TypeAnnotation::Result(_) | TypeAnnotation::CResult(_) => {
                self.writer.write_indent();
                self.writer.writeln(&format!("rl_print_result({});", accessor));
            }
            TypeAnnotation::Fn | TypeAnnotation::Callback(_, _) => {
                self.writer.write_indent();
                self.writer.writeln(&format!("rl_print_closure({});", accessor));
            }
            TypeAnnotation::Record(rname) | TypeAnnotation::CRecord(rname) => {
                self.writer.write_indent();
                self.writer.writeln(&format!("rl_print_rl_Record_{}({});", rname, accessor));
            }
            TypeAnnotation::Tuple(elems) | TypeAnnotation::CTuple(elems) => {
                let tuple_name = self.lookup_tuple_name(elems).to_string();
                self.writer.write_indent();
                self.writer.writeln(&format!("rl_print_{}({});", tuple_name, accessor));
            }
            TypeAnnotation::Enum(ename) | TypeAnnotation::CEnum(ename) => {
                self.writer.write_indent();
                self.writer.writeln(&format!("rl_print_Enum_{}({});", ename, accessor));
            }
            _ => {
                self.writer.write_indent();
                self.writer.writeln("printf(\"?\");");
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

    pub fn lookup_tuple_name(&self, field_types: &[TypeAnnotation]) -> &str {
        for (fields, name) in &self.tuple_names {
            if fields == field_types {
                return name;
            }
        }
        "rl_tuple_2"
    }

    pub fn ensure_tuple_type(&mut self, field_types: Vec<TypeAnnotation>) -> String {
        for (fields, name) in &self.tuple_names {
            if *fields == field_types {
                return name.clone();
            }
        }
        let arity = field_types.len();
        let count = self.tuple_names.iter().filter(|(f, _)| f.len() == arity).count();
        let name = if count == 0 {
            format!("rl_tuple_{}", arity)
        } else {
            format!("rl_tuple_{}_{}", arity, count)
        };
        self.writer.write("typedef struct { ");
        for (i, field_type) in field_types.iter().enumerate() {
            let c_type = type_to_c(field_type);
            self.writer.write(&format!("{} field_{}; ", c_type, i));
        }
        self.writer.writeln(&format!("}} {};", name));
        self.writer.write(&format!("void rl_print_{}({} v) {{ ", name, name));
        self.writer.writeln("printf(\"(\");");
        for (i, field_type) in field_types.iter().enumerate() {
            if i > 0 {
                self.writer.writeln("printf(\", \");");
            }
            self.emit_field_print(field_type, &format!("v.field_{}", i));
        }
        self.writer.writeln("printf(\")\");");
        self.writer.writeln("}");
        self.writer.write(&format!("void rl_println_{}({} v) {{ rl_print_{}(v); printf(\"\\n\"); }}\n", name, name, name));
        self.tuple_names.push((field_types, name.clone()));
        name
    }
}
