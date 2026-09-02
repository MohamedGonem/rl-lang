use crate::codegen::CCodegen;
use crate::codegen::ops::token_to_c_op;
use crate::name_mangle::{escape_c_char, escape_c_string, mangle};
use crate::types::type_to_c;
use crate::writer::CWriter;
use rl_ast::{ExprId, nodes::ExpressionKind};
use rl_ast::statements::TypeAnnotation;
use rl_lexer::tokentypes::TokenType;
use rl_utils::errors::Error;

impl<'a> CCodegen<'a> {
    pub fn compile_expr(&mut self, id: ExprId) -> Result<(), Error> {
        let kind = self.ast.exprs.get(id).kind.clone();
        match &kind {
            ExpressionKind::Integer(v) => {
                self.writer.write(&format!("(int64_t){}", v));
            }
            ExpressionKind::SInt(v) => {
                self.writer.write(&format!("(int32_t){}", v));
            }
            ExpressionKind::UInt(v) => {
                self.writer.write(&format!("(uint64_t){}", v));
            }
            ExpressionKind::SUInt(v) => {
                self.writer.write(&format!("(uint32_t){}", v));
            }
            ExpressionKind::Float(v) => {
                self.writer.write(&format!("(double){}", v));
            }
            ExpressionKind::SFloat(v) => {
                self.writer.write(&format!("(float){}", v));
            }
            ExpressionKind::Bool(v) => {
                self.writer.write(if *v { "true" } else { "false" });
            }
            ExpressionKind::Character(v) => {
                self.writer.write(&escape_c_char(*v));
            }
            ExpressionKind::String(v) => {
                let escaped = escape_c_string(v);
                self.writer
                    .write(&format!("rl_str_literal(\"{}\", {})", escaped, escaped.len()));
            }
            ExpressionKind::Null => {
                self.writer.write("rl_ok_null()");
            }
            ExpressionKind::Byte(v) => {
                self.writer.write(&format!("(uint8_t){}", v));
            }
            ExpressionKind::BByte(v) => {
                self.writer.write(&format!("(uint16_t){}", v));
            }
            ExpressionKind::SByte(v) => {
                self.writer.write(&format!("(int8_t){}", v));
            }
            ExpressionKind::BSByte(v) => {
                self.writer.write(&format!("(int16_t){}", v));
            }
            ExpressionKind::Grouping(inner) => {
                self.compile_expr(*inner)?;
            }
            ExpressionKind::Binary {
                left,
                operator,
                right,
            } => {
                self.compile_expr(*left)?;
                self.writer.write(&format!(" {} ", token_to_c_op(operator)));
                self.compile_expr(*right)?;
            }
            ExpressionKind::Unary { operator, operand } => {
                match operator {
                    TokenType::Minus => self.writer.write("-"),
                    TokenType::Bang => self.writer.write("!"),
                    _ => {}
                }
                self.compile_expr(*operand)?;
            }
            ExpressionKind::ResolvedIdentifier { name, .. } => {
                let c_name = self.lookup(name);
                if self.nullable_vars.contains(name) {
                    match self.var_types.get(name) {
                        Some(TypeAnnotation::Int) | Some(TypeAnnotation::CInt) => {
                            self.writer.write(&format!("rl_unwrap_i64({})", c_name));
                        }
                        Some(TypeAnnotation::Float) | Some(TypeAnnotation::CFloat) => {
                            self.writer.write(&format!("rl_unwrap_f64({})", c_name));
                        }
                        Some(TypeAnnotation::Bool) | Some(TypeAnnotation::CBool) => {
                            self.writer.write(&format!("rl_unwrap_bool({})", c_name));
                        }
                        Some(TypeAnnotation::String) | Some(TypeAnnotation::CString) => {
                            self.writer.write(&format!("rl_unwrap_str({})", c_name));
                        }
                        Some(TypeAnnotation::Array(_)) | Some(TypeAnnotation::CArray(_)) => {
                            self.writer.write(&format!("rl_unwrap_arr({})", c_name));
                        }
                        _ => {
                            self.writer.write(&format!("rl_unwrap_i64({})", c_name));
                        }
                    }
                } else {
                    self.writer.write(&c_name);
                }
            }
            ExpressionKind::Identifier(name) => {
                let c_name = self.lookup(name);
                self.writer.write(&c_name);
            }
            ExpressionKind::Call { path, args } => {
                self.compile_func_call(path, args)?;
            }
            ExpressionKind::CallExpr { callee, args } => {
                // Check if this is a closure call
                let callee_expr = self.ast.exprs.get(*callee);
                let (is_closure, callee_name) = if let ExpressionKind::ResolvedIdentifier { name, .. } = &callee_expr.kind {
                    let is_fn = matches!(self.var_types.get(name), Some(TypeAnnotation::Fn) | Some(TypeAnnotation::Callback(_, _)));
                    (is_fn, Some(name.clone()))
                } else {
                    (false, None)
                };

                if is_closure {
                    // Look up the closure's return type for unwrapping
                    let return_type = callee_name.as_ref().and_then(|n| self.closure_return_types.get(n));

                    // Build inline: rl_unwrap_XX(rl_closure_call(name, (rl_result[]){ args }, argc))
                    let need_unwrap = !matches!(return_type, None | Some(TypeAnnotation::Result(_)));
                    if need_unwrap {
                        match return_type {
                            Some(TypeAnnotation::Int) | Some(TypeAnnotation::CInt) => self.writer.write("rl_unwrap_i64("),
                            Some(TypeAnnotation::Float) | Some(TypeAnnotation::CFloat) => self.writer.write("rl_unwrap_f64("),
                            Some(TypeAnnotation::Bool) | Some(TypeAnnotation::CBool) => self.writer.write("rl_unwrap_bool("),
                            Some(TypeAnnotation::String) | Some(TypeAnnotation::CString) => self.writer.write("rl_unwrap_str("),
                            Some(TypeAnnotation::Array(_)) | Some(TypeAnnotation::CArray(_)) => self.writer.write("rl_unwrap_arr("),
                            _ => {}
                        }
                    }

                    self.writer.write("rl_closure_call(");
                    self.compile_expr(*callee)?;
                    self.writer.write(", (rl_result[]){ ");
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 { self.writer.write(", "); }
                        self.write_arg_as_result(*arg)?;
                    }
                    self.writer.write(&format!(" }}, {})", args.len()));

                    if need_unwrap {
                        self.writer.write(")");
                    }
                } else {
                    self.compile_expr(*callee)?;
                    self.writer.write("(");
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 { self.writer.write(", "); }
                        self.compile_expr(*arg)?;
                    }
                    self.writer.write(")");
                }
            }
            ExpressionKind::MethodCall {
                caller,
                method,
                args,
            } => {
                self.compile_method_call(*caller, method, args)?;
            }
            ExpressionKind::OkLiteral(inner) => {
                self.writer.write("rl_ok(");
                self.compile_expr(*inner)?;
                self.writer.write(")");
            }
            ExpressionKind::ErrLiteral(inner) => {
                self.writer.write("rl_err(");
                self.compile_expr(*inner)?;
                self.writer.write(")");
            }
            ExpressionKind::ErrorLiteral(inner) => {
                self.writer.write("rl_error(");
                self.compile_expr(*inner)?;
                self.writer.write(")");
            }
            ExpressionKind::Propagate(inner) => {
                let temp = self.temp_var();
                self.writer.write_indent();
                self.writer.write(&format!("rl_result {} = ", temp));
                self.compile_expr(*inner)?;
                self.writer.write(";\n");
                self.writer.write_indent();
                self.writer.write(&format!("if (!{}.is_ok) {{\n", temp));
                self.writer.indent();
                self.writer.write_indent();
                self.writer.write(&format!("return {};\n", temp));
                self.writer.dedent();
                self.writer.write_indent();
                self.writer.write("}\n");
                self.writer.write(&format!("{}.data.i64", temp));
            }
            ExpressionKind::ArrayLiteral(elems) => {
                self.writer.write("rl_arr_from_vals(&(int64_t[]){");
                for (i, elem) in elems.iter().enumerate() {
                    if i > 0 {
                        self.writer.write(", ");
                    }
                    self.compile_expr(*elem)?;
                }
                self.writer.write(&format!("}}, {}, (int32_t)sizeof(int64_t))", elems.len()));
            }
            ExpressionKind::MapLiteral(entries) => {
                let temp = self.temp_var();
                self.writer.write_indent();
                self.writer
                    .write(&format!("rl_map {} = rl_map_new();\n", temp));
                for (key_id, val_id) in entries {
                    let key_expr = self.ast.exprs.get(*key_id);
                    if let ExpressionKind::String(key_str) = &key_expr.kind {
                        let val_expr = self.ast.exprs.get(*val_id);
                        let val_type = match &val_expr.kind {
                            ExpressionKind::Integer(_) => TypeAnnotation::Int,
                            ExpressionKind::Float(_) => TypeAnnotation::Float,
                            ExpressionKind::Bool(_) => TypeAnnotation::Bool,
                            ExpressionKind::String(_) => TypeAnnotation::String,
                            ExpressionKind::ArrayLiteral(e) if !e.is_empty() => TypeAnnotation::Array(Box::new(TypeAnnotation::Infer)),
                            _ => TypeAnnotation::Int,
                        };
                        self.writer.write_indent();
                        self.writer.write(&format!(
                            "rl_map_set(&{}, \"{}\", ",
                            temp, key_str
                        ));
                        self.emit_value_wrapping(&val_type, *val_id)?;
                        self.writer.write(");\n");
                    }
                }
                self.writer.write(&temp);
            }
            ExpressionKind::SetLiteral(items) => {
                let temp = self.temp_var();
                self.writer.write_indent();
                self.writer
                    .write(&format!("rl_set {} = rl_set_new();\n", temp));
                for item_id in items {
                    let val_expr = self.ast.exprs.get(*item_id);
                    let val_type = match &val_expr.kind {
                        ExpressionKind::Integer(_) => TypeAnnotation::Int,
                        ExpressionKind::Float(_) => TypeAnnotation::Float,
                        ExpressionKind::Bool(_) => TypeAnnotation::Bool,
                        ExpressionKind::String(_) => TypeAnnotation::String,
                        _ => TypeAnnotation::Int,
                    };
                    self.writer.write_indent();
                    self.writer.write(&format!("rl_set_add(&{}, ", temp));
                    self.emit_value_wrapping(&val_type, *item_id)?;
                    self.writer.write(");\n");
                }
                self.writer.write(&temp);
            }
            ExpressionKind::TupleLiteral(elems) => {
                let field_types: Vec<TypeAnnotation> = elems.iter().map(|e| {
                    let expr = self.ast.exprs.get(*e);
                    match &expr.kind {
                        ExpressionKind::Integer(_) => TypeAnnotation::Int,
                        ExpressionKind::Float(_) => TypeAnnotation::Float,
                        ExpressionKind::Bool(_) => TypeAnnotation::Bool,
                        ExpressionKind::String(_) => TypeAnnotation::String,
                        ExpressionKind::Character(_) => TypeAnnotation::Char,
                        ExpressionKind::ResolvedIdentifier { name, .. } => {
                            self.var_types.get(name).cloned().unwrap_or(TypeAnnotation::Int)
                        }
                        _ => TypeAnnotation::Int,
                    }
                }).collect();
                let tuple_name = self.ensure_tuple_type(field_types);
                self.writer.write(&format!("({}){{ ", tuple_name));
                for (i, elem) in elems.iter().enumerate() {
                    if i > 0 {
                        self.writer.write(", ");
                    }
                    self.writer.write(&format!(".field_{} = ", i));
                    self.compile_expr(*elem)?;
                }
                self.writer.write(" }");
            }
            ExpressionKind::StructLiteral { name, fields } => {
                let c_name = format!("rl_Record_{}", name);
                self.writer.write(&format!("({}){{ ", c_name));
                for (i, (field_name, field_val)) in fields.iter().enumerate() {
                    if i > 0 {
                        self.writer.write(", ");
                    }
                    self.writer.write(&format!(".{} = ", field_name));
                    self.compile_expr(*field_val)?;
                }
                self.writer.write(" }");
            }
            ExpressionKind::FieldAccess { target, field } => {
                self.compile_expr(*target)?;
                self.writer.write(&format!(".{}", field));
            }
            ExpressionKind::FieldAssign {
                target,
                field,
                value,
            } => {
                self.compile_expr(*target)?;
                self.writer.write(&format!(".{} = ", field));
                self.compile_expr(*value)?;
            }
            ExpressionKind::EnumVariant { enum_name, variant } => {
                let c_name = format!(
                    "RL_TAG_{}_{}",
                    enum_name.to_uppercase(),
                    variant.to_uppercase()
                );
                self.writer.write(&c_name);
            }
            ExpressionKind::ResolvedAssign { name, value, .. } => {
                let c_name = self.lookup(name);
                if self.nullable_vars.contains(name) {
                    let value_expr = self.ast.exprs.get(*value);
                    if let ExpressionKind::Null = &value_expr.kind {
                        self.writer.write(&format!("{} = rl_ok_null()", c_name));
                    } else {
                        match self.var_types.get(name) {
                            Some(TypeAnnotation::Int) | Some(TypeAnnotation::CInt) => {
                                self.writer.write(&format!("{} = rl_ok_i64(", c_name));
                                self.compile_expr(*value)?;
                                self.writer.write(")");
                            }
                            Some(TypeAnnotation::Float) | Some(TypeAnnotation::CFloat) => {
                                self.writer.write(&format!("{} = rl_ok_f64(", c_name));
                                self.compile_expr(*value)?;
                                self.writer.write(")");
                            }
                            Some(TypeAnnotation::Bool) | Some(TypeAnnotation::CBool) => {
                                self.writer.write(&format!("{} = rl_ok_bool(", c_name));
                                self.compile_expr(*value)?;
                                self.writer.write(")");
                            }
                            Some(TypeAnnotation::String) | Some(TypeAnnotation::CString) => {
                                self.writer.write(&format!("{} = rl_ok_str(", c_name));
                                self.compile_expr(*value)?;
                                self.writer.write(")");
                            }
                            _ => {
                                self.writer.write(&format!("{} = rl_ok(", c_name));
                                self.compile_expr(*value)?;
                                self.writer.write(")");
                            }
                        }
                    }
                } else {
                    self.writer.write(&format!("{} = ", c_name));
                    self.compile_expr(*value)?;
                }
            }
            ExpressionKind::Index { target, index } => {
                let target_expr = self.ast.exprs.get(*target);
                if let ExpressionKind::ResolvedIdentifier { name, .. } = &target_expr.kind
                    && let Some(ta) = self.var_types.get(name)
                        && let TypeAnnotation::Array(inner) = ta {
                            let c_type = type_to_c(inner);
                            let c_name = self.lookup(name);
                            self.writer.write(&format!("(({}*){}.data)[", c_type, c_name));
                            self.compile_expr(*index)?;
                            self.writer.write("]");
                            return Ok(());
                        }
                self.compile_expr(*target)?;
                self.writer.write("[");
                self.compile_expr(*index)?;
                self.writer.write("]");
            }
            ExpressionKind::IndexAssign {
                target,
                index,
                value,
            } => {
                let target_expr = self.ast.exprs.get(*target);
                if let ExpressionKind::ResolvedIdentifier { name, .. } = &target_expr.kind
                    && let Some(ta) = self.var_types.get(name)
                        && let TypeAnnotation::Array(inner) = ta {
                            let c_type = type_to_c(inner);
                            let c_name = self.lookup(name);
                            self.writer.write(&format!("(({}*){}.data)[", c_type, c_name));
                            self.compile_expr(*index)?;
                            self.writer.write("] = ");
                            self.compile_expr(*value)?;
                            return Ok(());
                        }
                self.compile_expr(*target)?;
                self.writer.write("[");
                self.compile_expr(*index)?;
                self.writer.write("] = ");
                self.compile_expr(*value)?;
            }
            ExpressionKind::Cast { value, target_type } => {
                let c_type = type_to_c(target_type);
                self.writer.write(&format!("({})", c_type));
                self.compile_expr(*value)?;
            }
            ExpressionKind::ResolvedLambda { params, return_type, body, .. } => {
                self.compile_lambda(params, return_type, body)?;
            }
            _ => {
                self.writer.write("/* unhandled expr */");
            }
        }
        Ok(())
    }

    pub fn compile_func_call(&mut self, path: &[String], args: &[ExprId]) -> Result<(), Error> {
        let func_name = path.last().map(|s| s.as_str()).unwrap_or("");
        let _is_stdlib = path.first().map(|s| s.as_str()) == Some("std");

        match func_name {
            "println" | "print" => {
                let is_ln = func_name == "println";
                if args.is_empty() {
                    if is_ln {
                        self.writer.write("rl_println(\"\")");
                    } else {
                        self.writer.write("rl_print(\"\")");
                    }
                } else {
                    let arg = &args[0];
                    let expr = self.ast.exprs.get(*arg);
                    if let ExpressionKind::ResolvedIdentifier { name, .. } = &expr.kind {
                        if let Some(ta) = self.var_types.get(name) {
                            match ta {
                                TypeAnnotation::Tuple(elems) => {
                                    let c_name = self.lookup(name);
                                    let tuple_name = self.lookup_tuple_name(elems).to_string();
                                    let print_fn = if is_ln { "rl_println" } else { "rl_print" };
                                    self.writer.write(&format!("{}_{}({})", print_fn, tuple_name, c_name));
                                }
                                TypeAnnotation::Record(rname) => {
                                    let c_name = self.lookup(name);
                                    let print_fn = if is_ln { "rl_println" } else { "rl_print" };
                                    self.writer.write(&format!("{}_rl_Record_{}({})", print_fn, rname, c_name));
                                }
                                TypeAnnotation::Enum(ename) | TypeAnnotation::CEnum(ename) => {
                                    let c_name = self.lookup(name);
                                    let print_fn = if is_ln { "rl_println" } else { "rl_print" };
                                    self.writer.write(&format!("{}_Enum_{}({})", print_fn, ename, c_name));
                                }
                                _ if self.nullable_vars.contains(name) => {
                                    let c_fn = if is_ln { "rl_println_raw" } else { "rl_print_raw" };
                                    let c_name = self.lookup(name);
                                    self.writer.write(&format!("{}({})", c_fn, c_name));
                                }
                                _ => {
                                    let c_fn = if is_ln { "rl_println" } else { "rl_print" };
                                    self.writer.write(&format!("{}(", c_fn));
                                    self.compile_expr(*arg)?;
                                    self.writer.write(")");
                                }
                            }
                        } else {
                            let c_fn = if is_ln { "rl_println" } else { "rl_print" };
                            self.writer.write(&format!("{}(", c_fn));
                            self.compile_expr(*arg)?;
                            self.writer.write(")");
                        }
                    } else {
                        let c_fn = if is_ln { "rl_println" } else { "rl_print" };
                        self.writer.write(&format!("{}(", c_fn));
                        self.compile_expr(*arg)?;
                        self.writer.write(")");
                    }
                }
                return Ok(());
            }
            "ok" => {
                self.writer.write("rl_ok(");
                if !args.is_empty() {
                    self.compile_expr(args[0])?;
                }
                self.writer.write(")");
                return Ok(());
            }
            "err" => {
                self.writer.write("rl_err(");
                if !args.is_empty() {
                    self.compile_expr(args[0])?;
                }
                self.writer.write(")");
                return Ok(());
            }
            "error" => {
                self.writer.write("rl_error(");
                if !args.is_empty() {
                    self.compile_expr(args[0])?;
                }
                self.writer.write(")");
                return Ok(());
            }
            "map_len" => {
                self.writer.write("rl_ok((int64_t)rl_map_len(");
                if !args.is_empty() {
                    self.compile_expr(args[0])?;
                }
                self.writer.write("))");
                return Ok(());
            }
            "set_len" => {
                self.writer.write("rl_ok((int64_t)rl_set_len(");
                if !args.is_empty() {
                    self.compile_expr(args[0])?;
                }
                self.writer.write("))");
                return Ok(());
            }
            "len" => {
                self.writer.write("rl_str_len(");
                if !args.is_empty() {
                    self.compile_expr(args[0])?;
                }
                self.writer.write(")");
                return Ok(());
            }
            // ---- math (single arg, raw return) ----
            "sin" | "cos" | "tan" | "asin" | "acos" | "atan"
            | "exp" => {
                self.writer.write(&format!("{}(", func_name));
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            // ---- math (single arg, ok()-wrapped) ----
            "sqrt" | "log2" | "log10" | "ceil" | "floor" | "round" => {
                self.writer.write(&format!("rl_ok({}(", func_name));
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "abs" => {
                self.writer.write("rl_math_abs(");
                if !args.is_empty() {
                    self.writer.write("rl_ok(");
                    self.compile_expr(args[0])?;
                    self.writer.write(")");
                }
                self.writer.write(")");
                return Ok(());
            }
            "hypot" => {
                self.writer.write("hypot(");
                if args.len() >= 2 {
                    self.compile_expr(args[0])?;
                    self.writer.write(", ");
                    self.compile_expr(args[1])?;
                }
                self.writer.write(")");
                return Ok(());
            }
            "atan2" => {
                self.writer.write("atan2(");
                if args.len() >= 2 {
                    self.compile_expr(args[0])?;
                    self.writer.write(", ");
                    self.compile_expr(args[1])?;
                }
                self.writer.write(")");
                return Ok(());
            }
            "pow" => {
                self.writer.write("rl_math_pow(");
                if args.len() >= 2 {
                    self.writer.write("rl_ok(");
                    self.compile_expr(args[0])?;
                    self.writer.write("), rl_ok(");
                    self.compile_expr(args[1])?;
                    self.writer.write(")");
                }
                self.writer.write(")");
                return Ok(());
            }
            "log" => {
                self.writer.write("rl_ok((log(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(") / log(");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write("))");
                return Ok(());
            }
            "radians" => {
                self.writer.write("(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(" * (M_PI / 180.0))");
                return Ok(());
            }
            "degrees" => {
                self.writer.write("(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(" * (180.0 / M_PI))");
                return Ok(());
            }
            "sign" => {
                self.writer.write("((");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(" > 0) - (");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(" < 0))");
                return Ok(());
            }
            "lerp" => {
                self.writer.write("(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(" + (");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(" - ");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(") * ");
                if args.len() >= 3 { self.compile_expr(args[2])?; }
                self.writer.write(")");
                return Ok(());
            }
            "map_range" => {
                // (v - in_min) / (in_max - in_min) * (out_max - out_min) + out_min
                self.writer.write("(((");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(" - ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(") / (");
                if args.len() >= 3 { self.compile_expr(args[2])?; }
                self.writer.write(" - ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")) * (");
                if args.len() >= 5 { self.compile_expr(args[4])?; }
                self.writer.write(" - ");
                if args.len() >= 4 { self.compile_expr(args[3])?; }
                self.writer.write(") + ");
                if args.len() >= 4 { self.compile_expr(args[3])?; }
                self.writer.write(")");
                return Ok(());
            }
            "mod" => {
                self.writer.write("rl_ok(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(" % ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "max" => {
                self.writer.write("rl_ok(((");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(") > (");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(") ? (");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(") : (");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")))");
                return Ok(());
            }
            "min" => {
                self.writer.write("rl_ok(((");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(") < (");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(") ? (");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(") : (");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")))");
                return Ok(());
            }
            "clamp" => {
                self.writer.write("rl_ok(((");
                if args.len() >= 3 { self.compile_expr(args[0])?; }
                self.writer.write(") < (");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(") ? (");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(") : ((");
                if args.len() >= 3 { self.compile_expr(args[0])?; }
                self.writer.write(") > (");
                if args.len() >= 3 { self.compile_expr(args[2])?; }
                self.writer.write(") ? (");
                if args.len() >= 3 { self.compile_expr(args[2])?; }
                self.writer.write(") : (");
                if args.len() >= 3 { self.compile_expr(args[0])?; }
                self.writer.write("))))");
                return Ok(());
            }
            // ---- math (runtime needed) ----
            "factorial" | "gcd" | "lcm" | "is_prime" | "fibonacci" => {
                self.writer.write(&format!("rl_math_{}(", func_name));
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { self.writer.write(", "); }
                    self.compile_expr(*arg)?;
                }
                self.writer.write(")");
                return Ok(());
            }
            // ---- bitwise (all ok()-wrapped) ----
            "bit_and" => {
                self.writer.write("rl_ok(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(" & ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "bit_or" => {
                self.writer.write("rl_ok(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(" | ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "bit_xor" => {
                self.writer.write("rl_ok(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(" ^ ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "bit_not" => {
                self.writer.write("rl_ok(~(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "bit_shift_left" => {
                self.writer.write("rl_ok(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(" << ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "bit_shift_right" => {
                self.writer.write("rl_ok(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(" >> ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "count_bits" => {
                self.writer.write("rl_ok(__builtin_popcountll(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "leading_zeros" => {
                self.writer.write("rl_ok(__builtin_clzll(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "trailing_zeros" => {
                self.writer.write("rl_ok(__builtin_ctzll(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            // ---- result ----
            "is_ok" => {
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(".is_ok");
                return Ok(());
            }
            "is_err" => {
                self.writer.write("!");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(".is_ok");
                return Ok(());
            }
            "result_unwrap" => {
                let unwrap_fn = if !args.is_empty() { self.unwrap_fn_for_result(args[0]) } else { "rl_result_unwrap_i64" };
                self.writer.write(&format!("{}(", unwrap_fn));
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "result_unwrap_err" => {
                let unwrap_fn = if !args.is_empty() { self.unwrap_fn_for_result(args[0]) } else { "rl_result_unwrap_i64" };
                self.writer.write(&format!("{}(", unwrap_fn));
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "result_unwrap_or" => {
                let unwrap_fn = if !args.is_empty() { self.unwrap_fn_for_result(args[0]) } else { "rl_result_unwrap_i64" };
                self.writer.write("(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(".is_ok ? ");
                self.writer.write(&format!("{}(", unwrap_fn));
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(") : ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            // ---- checks ----
            "string_is_empty" => {
                self.writer.write("(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(".len == 0)");
                return Ok(());
            }
            "arr_is_empty" | "set_is_empty" | "map_is_empty" => {
                self.writer.write("(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(".len == 0)");
                return Ok(());
            }
            "arr_count" => {
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(".len");
                return Ok(());
            }
            // ---- process ----
            "exit" => {
                self.writer.write("exit(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "pid" => {
                self.writer.write("getpid()");
                return Ok(());
            }
            "sleep" => {
                self.writer.write("usleep(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(" * 1000)");
                return Ok(());
            }
            "env" => {
                self.writer.write("getenv(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            // ---- time ----
            "time_now" => {
                self.writer.write("time(NULL)");
                return Ok(());
            }
            "time_now_ms" => {
                self.writer.write("rl_time_now_ms()");
                return Ok(());
            }
            "time_add" => {
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(" + ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                return Ok(());
            }
            "time_diff" => {
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(" - ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                return Ok(());
            }
            // ---- path ----
            "path_exists" => {
                self.writer.write("((bool)(access(");
                if !args.is_empty() {
                    self.compile_expr(args[0])?;
                    self.writer.write(".data");
                }
                self.writer.write(", F_OK) == 0))");
                return Ok(());
            }
            // ---- fs ----
            "mkdir" => {
                self.writer.write("rl_fs_mkdir(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "rmdir" => {
                self.writer.write("rmdir(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "move_file" | "rename_file" => {
                self.writer.write("rename(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "temp_dir" => {
                self.writer.write("rl_str_literal(\"/tmp\", 4)");
                return Ok(());
            }
            // ---- math constants (25) ----
            "PI" => { self.writer.write("M_PI"); return Ok(()); }
            "E" => { self.writer.write("M_E"); return Ok(()); }
            "TAU" => { self.writer.write("(2.0 * M_PI)"); return Ok(()); }
            "PHI" => { self.writer.write("((1.0 + sqrt(5.0)) / 2.0)"); return Ok(()); }
            "INF" => { self.writer.write("INFINITY"); return Ok(()); }
            "NAN" => { self.writer.write("NAN"); return Ok(()); }
            "FRAC_1_PI" => { self.writer.write("(1.0 / M_PI)"); return Ok(()); }
            "FRAC_1_SQRT_2" => { self.writer.write("(1.0 / sqrt(2.0))"); return Ok(()); }
            "FRAC_2_PI" => { self.writer.write("(2.0 / M_PI)"); return Ok(()); }
            "FRAC_2_SQRT_PI" => { self.writer.write("(2.0 / sqrt(M_PI))"); return Ok(()); }
            "FRAC_PI_2" => { self.writer.write("(M_PI / 2.0)"); return Ok(()); }
            "FRAC_PI_3" => { self.writer.write("(M_PI / 3.0)"); return Ok(()); }
            "FRAC_PI_4" => { self.writer.write("(M_PI / 4.0)"); return Ok(()); }
            "FRAC_PI_6" => { self.writer.write("(M_PI / 6.0)"); return Ok(()); }
            "FRAC_PI_8" => { self.writer.write("(M_PI / 8.0)"); return Ok(()); }
            "SQRT_2" => { self.writer.write("sqrt(2.0)"); return Ok(()); }
            "LN_2" => { self.writer.write("M_LN2"); return Ok(()); }
            "LN_10" => { self.writer.write("M_LN10"); return Ok(()); }
            "LOG2_E" => { self.writer.write("M_LOG2E"); return Ok(()); }
            "LOG2_10" => { self.writer.write("(M_LN10 / M_LN2)"); return Ok(()); }
            "LOG10_2" => { self.writer.write("(M_LN2 / M_LN10)"); return Ok(()); }
            "LOG10_E" => { self.writer.write("M_LOG10E"); return Ok(()); }
            "EULER_GAMMA" => { self.writer.write("0.5772156649015329"); return Ok(()); }
            "is_inf" => {
                self.writer.write("isinf(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "is_nan" => {
                self.writer.write("isnan(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            // ---- string (runtime) ----
            "to_upper" => {
                self.writer.write("rl_str_to_upper(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "to_lower" => {
                self.writer.write("rl_str_to_lower(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "trim" => {
                self.writer.write("rl_str_trim(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "trim_start" => {
                self.writer.write("rl_str_trim_start(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "trim_end" => {
                self.writer.write("rl_str_trim_end(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "contains" => {
                self.writer.write("rl_str_contains(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "starts_with" => {
                self.writer.write("rl_str_starts_with(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "ends_with" => {
                self.writer.write("rl_str_ends_with(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "replace" => {
                self.writer.write("rl_str_replace(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(", ");
                if args.len() >= 3 { self.compile_expr(args[2])?; }
                self.writer.write(")");
                return Ok(());
            }
            "repeat" => {
                self.writer.write("rl_str_repeat(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "index_of" => {
                self.writer.write("rl_str_index_of(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "count" => {
                self.writer.write("rl_str_count(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "pad_left" => {
                self.writer.write("rl_str_pad_left(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(", ");
                if args.len() >= 3 { self.compile_expr(args[2])?; }
                self.writer.write(")");
                return Ok(());
            }
            "pad_right" => {
                self.writer.write("rl_str_pad_right(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(", ");
                if args.len() >= 3 { self.compile_expr(args[2])?; }
                self.writer.write(")");
                return Ok(());
            }
            "slice" => {
                self.writer.write("rl_ok(rl_str_slice(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(", ");
                if args.len() >= 3 { self.compile_expr(args[2])?; }
                self.writer.write("))");
                return Ok(());
            }
            "reverse" => {
                self.writer.write("rl_str_reverse(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "bytes" => {
                self.writer.write("rl_str_bytes(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "chars" => {
                self.writer.write("rl_str_chars(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "char_at" => {
                self.writer.write("rl_ok(rl_str_char_at(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write("))");
                return Ok(());
            }
            "join" => {
                self.writer.write("rl_ok(rl_str_join(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write("))");
                return Ok(());
            }
            "split" => {
                self.writer.write("rl_str_split(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "concat" => {
                self.writer.write("rl_str_concat_variadic((rl_result[]){ ");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { self.writer.write(", "); }
                    self.write_arg_as_result(*arg)?;
                }
                self.writer.write(&format!(" }}, {})", args.len()));
                return Ok(());
            }
            "format" => {
                self.writer.write("rl_str_format(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                if args.len() > 1 {
                    self.writer.write(", (rl_result[]){ ");
                    for (i, arg) in args[1..].iter().enumerate() {
                        if i > 0 { self.writer.write(", "); }
                        self.write_arg_as_result(*arg)?;
                    }
                    self.writer.write(&format!(" }}, {})", args.len() - 1));
                } else {
                    self.writer.write(", NULL, 0");
                }
                self.writer.write(")");
                return Ok(());
            }
            // ---- is_empty (works for both string and collections) ----
            "is_empty" => {
                self.writer.write("(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(".len == 0)");
                return Ok(());
            }
            // ---- debug ----
            "panic" => {
                self.writer.write("rl_panic(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "unreachable" => {
                self.writer.write("rl_unreachable()");
                return Ok(());
            }
            "todo" => {
                self.writer.write("rl_todo()");
                return Ok(());
            }
            "assert" => {
                self.writer.write("if (!");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(") { rl_assert_fail_msg(rl_str_literal(\"assert\", 6), rl_str_literal(\"\", 0)); }");
                return Ok(());
            }
            "assert_eq" => {
                self.writer.write("if ((");
                if args.len() >= 2 { self.compile_expr(args[0])?; }
                self.writer.write(") != (");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")) { rl_assert_fail(rl_str_literal(\"assert_eq\", 8), ");
                if args.len() >= 2 { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write("); }");
                return Ok(());
            }
            "assert_ne" => {
                self.writer.write("if ((");
                if args.len() >= 2 { self.compile_expr(args[0])?; }
                self.writer.write(") == (");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")) { rl_assert_fail(rl_str_literal(\"assert_ne\", 8), ");
                if args.len() >= 2 { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write("); }");
                return Ok(());
            }
            "assert_lt" => {
                self.writer.write("if ((");
                if args.len() >= 2 { self.compile_expr(args[0])?; }
                self.writer.write(") >= (");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")) { rl_assert_fail(rl_str_literal(\"assert_lt\", 8), ");
                if args.len() >= 2 { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write("); }");
                return Ok(());
            }
            "assert_le" => {
                self.writer.write("if ((");
                if args.len() >= 2 { self.compile_expr(args[0])?; }
                self.writer.write(") > (");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")) { rl_assert_fail(rl_str_literal(\"assert_le\", 8), ");
                if args.len() >= 2 { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write("); }");
                return Ok(());
            }
            "assert_gt" => {
                self.writer.write("if ((");
                if args.len() >= 2 { self.compile_expr(args[0])?; }
                self.writer.write(") <= (");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")) { rl_assert_fail(rl_str_literal(\"assert_gt\", 8), ");
                if args.len() >= 2 { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write("); }");
                return Ok(());
            }
            "assert_ge" => {
                self.writer.write("if ((");
                if args.len() >= 2 { self.compile_expr(args[0])?; }
                self.writer.write(") < (");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")) { rl_assert_fail(rl_str_literal(\"assert_ge\", 8), ");
                if args.len() >= 2 { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write("); }");
                return Ok(());
            }
            "assert_approx_eq" => {
                self.writer.write("if (fabs((double)(");
                if args.len() >= 2 { self.compile_expr(args[0])?; }
                self.writer.write(") - (double)(");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")) > 1e-9) { rl_assert_fail(rl_str_literal(\"assert_approx_eq\", 15), ");
                if args.len() >= 2 { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write("); }");
                return Ok(());
            }
            "type_of" => {
                self.writer.write("rl_type_of(_Generic((");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("), int64_t: 1, double: 2, bool: 3, rl_string: 4, default: 0))");
                return Ok(());
            }
            "dbg" => {
                self.writer.write("_Generic((");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("), int64_t: rl_dbg_int64, double: rl_dbg_float64, bool: rl_dbg_bool, rl_string: rl_dbg_str, default: rl_dbg_int64)(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            // ---- path ----
            "path_extension" => {
                self.writer.write("rl_path_extension(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "path_filename" => {
                self.writer.write("rl_path_filename(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "path_parent" => {
                self.writer.write("rl_path_parent(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "path_stem" => {
                self.writer.write("rl_path_stem(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "path_pop" => {
                self.writer.write("rl_path_pop(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "path_join" | "path_push" => {
                self.writer.write("rl_path_join(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "path_set_extension" => {
                self.writer.write("rl_path_set_extension(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "path_is_dir" => {
                self.writer.write("rl_path_is_dir(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "path_is_file" => {
                self.writer.write("rl_path_is_file(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            // ---- fs ----
            "file_size" => {
                self.writer.write("rl_ok(rl_fs_file_size(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "file_modified" => {
                self.writer.write("rl_ok(rl_fs_file_modified(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "copy_file" => {
                self.writer.write("rl_ok(rl_fs_copy_file(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write("))");
                return Ok(());
            }
            "mkdir_all" => {
                self.writer.write("rl_ok(rl_fs_mkdir_all(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "rmdir_all" => {
                self.writer.write("rl_ok(rl_fs_rmdir_all(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "list_dir" => {
                self.writer.write("rl_ok(rl_fs_list_dir(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            // ---- process ----
            "cwd" => {
                self.writer.write("rl_ok(rl_process_cwd())");
                return Ok(());
            }
            "set_cwd" => {
                self.writer.write("rl_ok(rl_process_set_cwd(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "exec" => {
                self.writer.write("rl_ok(rl_process_exec(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "exec_code" => {
                self.writer.write("rl_ok(rl_process_exec_code(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "exec_lines" => {
                self.writer.write("rl_ok(rl_process_exec_lines(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "with_exec" => {
                self.writer.write("rl_ok(rl_process_with_exec(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write("))");
                return Ok(());
            }
            "with_exec_code" => {
                self.writer.write("rl_ok(rl_process_with_exec_code(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write("))");
                return Ok(());
            }
            "with_exec_lines" => {
                self.writer.write("rl_ok(rl_process_with_exec_lines(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write("))");
                return Ok(());
            }
            "args" => {
                self.writer.write("rl_process_args()");
                return Ok(());
            }
            // ---- time (extended) ----
            "format_time" => {
                self.writer.write("rl_ok(rl_time_format_time(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write("))");
                return Ok(());
            }
            "format_date_str" => {
                self.writer.write("rl_ok(rl_time_format_date_str(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "format_time_str" => {
                self.writer.write("rl_ok(rl_time_format_time_str(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "time_parts" => {
                self.writer.write("rl_ok(rl_time_parts(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            // ---- io (extended) ----
            "read_file" => {
                self.writer.write("rl_io_read_file(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "read_lines" => {
                self.writer.write("rl_io_read_lines(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "read_bytes" => {
                self.writer.write("rl_io_read_bytes(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "read" => {
                self.writer.write("rl_ok(rl_io_read())");
                return Ok(());
            }
            "read_int" => {
                self.writer.write("rl_ok(rl_io_read_int())");
                return Ok(());
            }
            "read_float" => {
                self.writer.write("rl_ok(rl_io_read_float())");
                return Ok(());
            }
            "write_file" => {
                self.writer.write("rl_io_write_file(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "append_file" => {
                self.writer.write("rl_io_append_file(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "delete_file" => {
                self.writer.write("rl_ok(rl_io_delete_file(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "eprint" => {
                self.writer.write("rl_io_eprint(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "eprintln" => {
                self.writer.write("rl_io_eprintln(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            // ---- types (all ok()-wrapped) ----
            "to_string" => {
                self.writer.write("rl_ok(rl_types_to_string(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "to_bin" => {
                self.writer.write("rl_ok(rl_types_to_bin(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "to_hex" => {
                self.writer.write("rl_ok(rl_types_to_hex(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "to_oct" => {
                self.writer.write("rl_ok(rl_types_to_oct(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "to_int" => {
                self.writer.write("rl_ok((int64_t)");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "to_float" => {
                self.writer.write("rl_ok((double)");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "to_bool" => {
                self.writer.write("rl_ok_bool((");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(") ? true : false)");
                return Ok(());
            }
            "to_byte" => {
                self.writer.write("rl_types_to_byte(");
                if !args.is_empty() {
                    self.writer.write("rl_ok(");
                    self.compile_expr(args[0])?;
                    self.writer.write(")");
                }
                self.writer.write(")");
                return Ok(());
            }
            "to_char" => {
                self.writer.write("rl_types_to_char(");
                if !args.is_empty() {
                    self.writer.write("rl_ok(");
                    self.compile_expr(args[0])?;
                    self.writer.write(")");
                }
                self.writer.write(")");
                return Ok(());
            }
            "error_unwrap" => {
                self.writer.write("rl_types_error_unwrap(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "is_bool" => {
                self.writer.write("rl_is_bool(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "is_int" => {
                self.writer.write("rl_is_int(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "is_float" => {
                self.writer.write("rl_is_float(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "is_string" => {
                self.writer.write("rl_is_string(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "is_null" => {
                self.writer.write("rl_is_null(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "is_char" => {
                self.writer.write("rl_is_char(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "is_byte" => {
                self.writer.write("rl_is_byte(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "is_error" => {
                self.writer.write("rl_is_error(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            // ---- random ----
            "rand_int" => {
                self.writer.write("rl_rand_int()");
                return Ok(());
            }
            "rand_float" => {
                self.writer.write("rl_rand_float()");
                return Ok(());
            }
            "rand_bool" => {
                self.writer.write("rl_rand_bool()");
                return Ok(());
            }
            "rand_bool_weighted" => {
                self.writer.write("rl_rand_bool_weighted(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "rand_char" => {
                self.writer.write("rl_rand_char()");
                return Ok(());
            }
            "rand_byte" => {
                self.writer.write("rl_rand_byte()");
                return Ok(());
            }
            "rand_int_range" => {
                self.writer.write("rl_ok(rl_rand_int_range(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write("))");
                return Ok(());
            }
            "rand_float_range" => {
                self.writer.write("rl_ok(rl_rand_float_range(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write("))");
                return Ok(());
            }
            "rand_dice" => {
                self.writer.write("rl_ok(rl_rand_dice(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "rand_range" => {
                self.writer.write("rl_ok(rl_rand_range(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "rand_range_step" => {
                self.writer.write("rl_ok(rl_rand_range_step(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(", ");
                if args.len() >= 3 { self.compile_expr(args[2])?; }
                self.writer.write("))");
                return Ok(());
            }
            "rand_string" => {
                self.writer.write("rl_ok(rl_rand_string(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "rand_dices" => {
                self.writer.write("rl_rand_dices(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "rand_bytes" => {
                self.writer.write("rl_rand_bytes(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "rand_choice" => {
                self.writer.write("rl_rand_choice(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "rand_choices" => {
                self.writer.write("rl_rand_choices(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "rand_sample" => {
                self.writer.write("rl_rand_sample(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "rand_shuffle" => {
                self.writer.write("rl_rand_shuffle(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            // ---- terminal ----
            "term_enter" => { self.writer.write("rl_term_enter()"); return Ok(()); }
            "term_leave" => { self.writer.write("rl_term_leave()"); return Ok(()); }
            "term_clear" => { self.writer.write("rl_term_clear()"); return Ok(()); }
            "term_clear_line" => { self.writer.write("rl_term_clear_line()"); return Ok(()); }
            "term_move" => {
                self.writer.write("rl_term_move(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "term_move_to_col" => {
                self.writer.write("rl_term_move_to_col(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "term_move_to_row" => {
                self.writer.write("rl_term_move_to_row(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "term_move_up" => {
                self.writer.write("rl_term_move_up(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "term_move_down" => {
                self.writer.write("rl_term_move_down(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "term_move_left" => {
                self.writer.write("rl_term_move_left(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "term_move_right" => {
                self.writer.write("rl_term_move_right(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "term_next_line" => {
                self.writer.write("rl_term_next_line(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "term_prev_line" => {
                self.writer.write("rl_term_prev_line(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "term_save_cursor" => { self.writer.write("rl_term_save_cursor()"); return Ok(()); }
            "term_restore_cursor" => { self.writer.write("rl_term_restore_cursor()"); return Ok(()); }
            "term_hide_cursor" => { self.writer.write("rl_term_hide_cursor()"); return Ok(()); }
            "term_show_cursor" => { self.writer.write("rl_term_show_cursor()"); return Ok(()); }
            "term_get_size" => {
                let c = self.temp_var();
                let r = self.temp_var();
                self.writer.write(&format!("{{ int64_t {0}, {1}; rl_term_get_size(&{0}, &{1}); rl_ok_arr(rl_arr_from_vals(&(int64_t[]){{{0}, {1}}}, 2, sizeof(int64_t))) }}", c, r));
                return Ok(());
            }
            "term_set_size" => {
                self.writer.write("rl_term_set_size(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "term_set_title" => {
                self.writer.write("rl_term_set_title(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "term_scroll_up" => {
                self.writer.write("rl_term_scroll_up(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "term_scroll_down" => {
                self.writer.write("rl_term_scroll_down(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "term_flush" => { self.writer.write("rl_term_flush()"); return Ok(()); }
            "term_set_fg" => {
                self.writer.write("rl_term_set_fg(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(", ");
                if args.len() >= 3 { self.compile_expr(args[2])?; }
                self.writer.write(")");
                return Ok(());
            }
            "term_set_bg" => {
                self.writer.write("rl_term_set_bg(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(", ");
                if args.len() >= 3 { self.compile_expr(args[2])?; }
                self.writer.write(")");
                return Ok(());
            }
            "term_reset_color" => { self.writer.write("rl_term_reset_color()"); return Ok(()); }
            "term_fg" => {
                self.writer.write("rl_term_fg(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "term_bg" => {
                self.writer.write("rl_term_bg(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "term_bold" => { self.writer.write("rl_term_bold()"); return Ok(()); }
            "term_dim" => { self.writer.write("rl_term_dim()"); return Ok(()); }
            "term_italic" => { self.writer.write("rl_term_italic()"); return Ok(()); }
            "term_underline" => { self.writer.write("rl_term_underline()"); return Ok(()); }
            "term_blink" => { self.writer.write("rl_term_blink()"); return Ok(()); }
            "term_reverse" => { self.writer.write("rl_term_reverse()"); return Ok(()); }
            "term_crossed_out" => { self.writer.write("rl_term_crossed_out()"); return Ok(()); }
            "term_reset_attr" => { self.writer.write("rl_term_reset_attr()"); return Ok(()); }
            "term_enable_wrap" => { self.writer.write("rl_term_enable_wrap()"); return Ok(()); }
            "term_disable_wrap" => { self.writer.write("rl_term_disable_wrap()"); return Ok(()); }
            "term_begin_sync" => { self.writer.write("rl_term_begin_sync()"); return Ok(()); }
            "term_end_sync" => { self.writer.write("rl_term_end_sync()"); return Ok(()); }
            "term_enable_mouse" => { self.writer.write("rl_term_enable_mouse()"); return Ok(()); }
            "term_disable_mouse" => { self.writer.write("rl_term_disable_mouse()"); return Ok(()); }
            "term_print" => {
                self.writer.write("rl_term_print_inline(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "term_read_key" => {
                self.writer.write("rl_term_read_key()");
                return Ok(());
            }
            "term_poll" => {
                self.writer.write("rl_ok(rl_term_poll(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            // ---- collections (extended) ----
            "set_add" => {
                self.writer.write("rl_ok(rl_set_add_s(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 {
                    let val_expr = self.ast.exprs.get(args[1]);
                    let val_type = self.infer_expr_type(&val_expr.kind);
                    self.emit_value_wrapping(&val_type, args[1])?;
                }
                self.writer.write("))");
                return Ok(());
            }
            "set_remove" => {
                self.writer.write("rl_ok(rl_set_remove_s(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 {
                    let val_expr = self.ast.exprs.get(args[1]);
                    let val_type = self.infer_expr_type(&val_expr.kind);
                    self.emit_value_wrapping(&val_type, args[1])?;
                }
                self.writer.write("))");
                return Ok(());
            }
            "set_contains" => {
                self.writer.write("rl_set_contains_s(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 {
                    let val_expr = self.ast.exprs.get(args[1]);
                    let val_type = self.infer_expr_type(&val_expr.kind);
                    self.emit_value_wrapping(&val_type, args[1])?;
                }
                self.writer.write(")");
                return Ok(());
            }
            "set_to_array" => {
                self.writer.write("rl_ok(rl_set_to_array(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "map_contains" => {
                self.writer.write("rl_map_contains_s(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "map_remove" => {
                self.writer.write("rl_map_remove_s(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "map_get" => {
                self.writer.write("rl_map_get_s(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "map_keys" => {
                self.writer.write("rl_ok(rl_map_keys(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "map_values" => {
                self.writer.write("rl_ok(rl_map_values(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "map_clear" => {
                self.writer.write("rl_map_clear(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "map_merge" => {
                self.writer.write("rl_map_merge(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "map_to_array" => {
                self.writer.write("rl_ok(rl_map_to_array_s(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            // ---- array (extended, non-mutating) ----
            "arr_first" => {
                self.writer.write("rl_arr_first(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "arr_last" => {
                self.writer.write("rl_arr_last(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "arr_contains" => {
                self.writer.write("rl_arr_contains(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "arr_index_of" => {
                self.writer.write("rl_arr_index_of(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "arr_reverse" => {
                self.writer.write("rl_ok(rl_arr_reverse(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "arr_concat" => {
                self.writer.write("rl_ok(rl_arr_concat(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write("))");
                return Ok(());
            }
            "arr_unique" => {
                self.writer.write("rl_ok(rl_arr_unique(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "arr_slice" => {
                self.writer.write("rl_ok(rl_arr_slice(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(", ");
                if args.len() >= 3 { self.compile_expr(args[2])?; }
                self.writer.write("))");
                return Ok(());
            }
            "arr_fill" => {
                self.writer.write("rl_arr_fill(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "arr_range" => {
                self.writer.write("rl_arr_range(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(", ");
                if args.len() >= 3 { self.compile_expr(args[2])?; }
                self.writer.write(")");
                return Ok(());
            }
            "arr_sum" => {
                self.writer.write("rl_arr_sum(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "arr_product" => {
                self.writer.write("rl_arr_product(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "arr_max" => {
                self.writer.write("rl_arr_max(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "arr_min" => {
                self.writer.write("rl_arr_min(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "arr_sort" => {
                self.writer.write("rl_ok(rl_arr_sort(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "arr_flatten" => {
                self.writer.write("rl_ok(rl_arr_flatten(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "arr_zip" => {
                let a_temp = self.temp_var();
                let b_temp = self.temp_var();
                let n_temp = self.temp_var();
                let i_temp = self.temp_var();
                let buf_temp = self.temp_var();
                let a_elems_temp = self.temp_var();
                let b_elems_temp = self.temp_var();
                self.writer.write(&format!("{{ rl_array {} = ", a_temp));
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("; ");
                self.writer.write(&format!("rl_array {} = ", b_temp));
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write("; ");
                self.writer.write(&format!("uint64_t {} = {}.len < {}.len ? {}.len : {}.len; ", n_temp, a_temp, b_temp, a_temp, b_temp));
                self.writer.write(&format!("char *{} = malloc({} * {}.elem_size * 2); ", buf_temp, n_temp, a_temp));
                self.writer.write(&format!("char *{} = (char *){}.data; ", a_elems_temp, a_temp));
                self.writer.write(&format!("char *{} = (char *){}.data; ", b_elems_temp, b_temp));
                self.writer.write(&format!("for (uint64_t {} = 0; {} < {}; {}++) {{ ", i_temp, i_temp, n_temp, i_temp));
                self.writer.write(&format!("memcpy({} + {} * {}.elem_size * 2, {} + {} * {}.elem_size, {}.elem_size); ", buf_temp, i_temp, a_temp, a_elems_temp, i_temp, a_temp, a_temp));
                self.writer.write(&format!("memcpy({} + {} * {}.elem_size * 2 + {}.elem_size, {} + {} * {}.elem_size, {}.elem_size); ", buf_temp, i_temp, a_temp, a_temp, b_elems_temp, i_temp, b_temp, b_temp));
                self.writer.write("} ");
                self.writer.write(&format!("rl_array _zr = {{ .data = {}, .len = {}, .cap = {}, .elem_size = {}.elem_size * 2, .type_tag = {}.type_tag }}; ", buf_temp, n_temp, n_temp, a_temp, a_temp));
                self.writer.write(&format!("rl_ok_arr(_zr); }}"));
                return Ok(());
            }
            "arr_push" => {
                self.writer.write("rl_arr_push(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "arr_pop" => {
                self.writer.write("rl_arr_pop(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(")");
                return Ok(());
            }
            "arr_insert" => {
                self.writer.write("rl_arr_insert(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(", ");
                if args.len() >= 3 { self.compile_expr(args[2])?; }
                self.writer.write(")");
                return Ok(());
            }
            "arr_remove" => {
                self.writer.write("rl_arr_remove(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write(")");
                return Ok(());
            }
            "arr_filter" => {
                // Check if second arg is a closure
                if args.len() >= 2 {
                    let second_expr = self.ast.exprs.get(args[1]);
                    if matches!(&second_expr.kind, ExpressionKind::ResolvedLambda { .. }) {
                        self.writer.write("rl_arr_filter_closure(");
                        self.compile_expr(args[0])?;
                        self.writer.write(", ");
                        self.compile_expr(args[1])?;
                        self.writer.write(")");
                        return Ok(());
                    }
                }
                // Fallback to regular call
            }
            "arr_map" => {
                if args.len() >= 2 {
                    let second_expr = self.ast.exprs.get(args[1]);
                    if matches!(&second_expr.kind, ExpressionKind::ResolvedLambda { .. }) {
                        self.writer.write("rl_arr_map_closure(");
                        self.compile_expr(args[0])?;
                        self.writer.write(", ");
                        self.compile_expr(args[1])?;
                        self.writer.write(")");
                        return Ok(());
                    }
                }
            }
            "arr_find" => {
                if args.len() >= 2 {
                    let second_expr = self.ast.exprs.get(args[1]);
                    if matches!(&second_expr.kind, ExpressionKind::ResolvedLambda { .. }) {
                        self.writer.write("rl_arr_find_closure(");
                        self.compile_expr(args[0])?;
                        self.writer.write(", ");
                        self.compile_expr(args[1])?;
                        self.writer.write(")");
                        return Ok(());
                    }
                }
            }
            "arr_reduce" => {
                if args.len() >= 3 {
                    let second_expr = self.ast.exprs.get(args[1]);
                    if matches!(&second_expr.kind, ExpressionKind::ResolvedLambda { .. }) {
                        self.writer.write("rl_arr_reduce_closure(");
                        self.compile_expr(args[0])?;
                        self.writer.write(", ");
                        self.compile_expr(args[1])?;
                        self.writer.write(", ");
                        self.compile_expr(args[2])?;
                        self.writer.write(")");
                        return Ok(());
                    }
                }
            }
            "arr_find_index" => {
                if args.len() >= 2 {
                    let second_expr = self.ast.exprs.get(args[1]);
                    if matches!(&second_expr.kind, ExpressionKind::ResolvedLambda { .. }) {
                        self.writer.write("rl_arr_find_index_closure(");
                        self.compile_expr(args[0])?;
                        self.writer.write(", ");
                        self.compile_expr(args[1])?;
                        self.writer.write(")");
                        return Ok(());
                    }
                }
            }
            "arr_all" => {
                if args.len() >= 2 {
                    let second_expr = self.ast.exprs.get(args[1]);
                    if matches!(&second_expr.kind, ExpressionKind::ResolvedLambda { .. }) {
                        self.writer.write("rl_arr_all_closure(");
                        self.compile_expr(args[0])?;
                        self.writer.write(", ");
                        self.compile_expr(args[1])?;
                        self.writer.write(")");
                        return Ok(());
                    }
                }
            }
            "arr_any" => {
                if args.len() >= 2 {
                    let second_expr = self.ast.exprs.get(args[1]);
                    if matches!(&second_expr.kind, ExpressionKind::ResolvedLambda { .. }) {
                        self.writer.write("rl_arr_any_closure(");
                        self.compile_expr(args[0])?;
                        self.writer.write(", ");
                        self.compile_expr(args[1])?;
                        self.writer.write(")");
                        return Ok(());
                    }
                }
            }
            "arr_for_each" => {
                if args.len() >= 2 {
                    let second_expr = self.ast.exprs.get(args[1]);
                    if matches!(&second_expr.kind, ExpressionKind::ResolvedLambda { .. }) {
                        self.writer.write("rl_arr_for_each_closure(");
                        self.compile_expr(args[0])?;
                        self.writer.write(", ");
                        self.compile_expr(args[1])?;
                        self.writer.write(")");
                        return Ok(());
                    }
                }
            }
            "arr_flat_map" => {
                if args.len() >= 2 {
                    let second_expr = self.ast.exprs.get(args[1]);
                    if matches!(&second_expr.kind, ExpressionKind::ResolvedLambda { .. }) {
                        self.writer.write("rl_arr_flat_map_closure(");
                        self.compile_expr(args[0])?;
                        self.writer.write(", ");
                        self.compile_expr(args[1])?;
                        self.writer.write(")");
                        return Ok(());
                    }
                }
            }
            "arr_sort_by" => {
                if args.len() >= 2 {
                    let second_expr = self.ast.exprs.get(args[1]);
                    if matches!(&second_expr.kind, ExpressionKind::ResolvedLambda { .. }) {
                        self.writer.write("rl_arr_sort_by_closure(");
                        self.compile_expr(args[0])?;
                        self.writer.write(", ");
                        self.compile_expr(args[1])?;
                        self.writer.write(")");
                        return Ok(());
                    }
                }
            }
            "result_map" => {
                if args.len() >= 2 {
                    let second_expr = self.ast.exprs.get(args[1]);
                    if matches!(&second_expr.kind, ExpressionKind::ResolvedLambda { .. }) {
                        self.writer.write("rl_result_map_closure(");
                        self.compile_expr(args[0])?;
                        self.writer.write(", ");
                        self.compile_expr(args[1])?;
                        self.writer.write(")");
                        return Ok(());
                    }
                }
            }
            "result_map_err" => {
                if args.len() >= 2 {
                    let second_expr = self.ast.exprs.get(args[1]);
                    if matches!(&second_expr.kind, ExpressionKind::ResolvedLambda { .. }) {
                        self.writer.write("rl_result_map_err_closure(");
                        self.compile_expr(args[0])?;
                        self.writer.write(", ");
                        self.compile_expr(args[1])?;
                        self.writer.write(")");
                        return Ok(());
                    }
                }
            }
            "bench"
                if args.len() >= 2 => {
                    let first_expr = self.ast.exprs.get(args[0]);
                    if matches!(&first_expr.kind, ExpressionKind::ResolvedLambda { .. }) {
                        self.writer.write("rl_bench_closure(");
                        self.compile_expr(args[0])?;
                        self.writer.write(", ");
                        self.compile_expr(args[1])?;
                        self.writer.write(")");
                        return Ok(());
                    }
                }
            "tcp_listen" if self.std_net_imports.contains("tcp_listen") => {
                self.writer.write("rl_net_tcp_listen(");
                self.compile_expr(args[0])?;
                self.writer.write(")");
                return Ok(());
            }
            "tcp_accept" if self.std_net_imports.contains("tcp_accept") => {
                self.writer.write("rl_net_tcp_accept(");
                self.compile_expr(args[0])?;
                self.writer.write(")");
                return Ok(());
            }
            "tcp_connect" if self.std_net_imports.contains("tcp_connect") => {
                self.writer.write("rl_net_tcp_connect(");
                self.compile_expr(args[0])?;
                self.writer.write(")");
                return Ok(());
            }
            "tcp_read" if self.std_net_imports.contains("tcp_read") => {
                self.writer.write("rl_net_tcp_read(");
                self.compile_expr(args[0])?;
                self.writer.write(", ");
                self.compile_expr(args[1])?;
                self.writer.write(")");
                return Ok(());
            }
            "tcp_write" if self.std_net_imports.contains("tcp_write") => {
                self.writer.write("rl_net_tcp_write(");
                self.compile_expr(args[0])?;
                self.writer.write(", ");
                self.compile_expr(args[1])?;
                self.writer.write(")");
                return Ok(());
            }
            "tcp_peer_addr" if self.std_net_imports.contains("tcp_peer_addr") => {
                self.writer.write("rl_net_tcp_peer_addr(");
                self.compile_expr(args[0])?;
                self.writer.write(")");
                return Ok(());
            }
            "tcp_local_addr" if self.std_net_imports.contains("tcp_local_addr") => {
                self.writer.write("rl_net_tcp_local_addr(");
                self.compile_expr(args[0])?;
                self.writer.write(")");
                return Ok(());
            }
            "tcp_set_timeout" if self.std_net_imports.contains("tcp_set_timeout") => {
                self.writer.write("rl_net_tcp_set_timeout(");
                self.compile_expr(args[0])?;
                self.writer.write(", ");
                self.compile_expr(args[1])?;
                self.writer.write(")");
                return Ok(());
            }
            "tcp_set_nonblocking" if self.std_net_imports.contains("tcp_set_nonblocking") => {
                self.writer.write("rl_net_tcp_set_nonblocking(");
                self.compile_expr(args[0])?;
                self.writer.write(", ");
                self.compile_expr(args[1])?;
                self.writer.write(")");
                return Ok(());
            }
            "tcp_shutdown" if self.std_net_imports.contains("tcp_shutdown") => {
                self.writer.write("rl_net_tcp_shutdown(");
                self.compile_expr(args[0])?;
                self.writer.write(", ");
                self.compile_expr(args[1])?;
                self.writer.write(")");
                return Ok(());
            }
            "tcp_close" if self.std_net_imports.contains("tcp_close") => {
                self.writer.write("rl_net_tcp_close(");
                self.compile_expr(args[0])?;
                self.writer.write(")");
                return Ok(());
            }
            "udp_bind" if self.std_net_imports.contains("udp_bind") => {
                self.writer.write("rl_net_udp_bind(");
                self.compile_expr(args[0])?;
                self.writer.write(")");
                return Ok(());
            }
            "udp_connect" if self.std_net_imports.contains("udp_connect") => {
                self.writer.write("rl_net_udp_connect(");
                self.compile_expr(args[0])?;
                self.writer.write(", ");
                self.compile_expr(args[1])?;
                self.writer.write(")");
                return Ok(());
            }
            "udp_send" if self.std_net_imports.contains("udp_send") => {
                self.writer.write("rl_net_udp_send(");
                self.compile_expr(args[0])?;
                self.writer.write(", ");
                self.compile_expr(args[1])?;
                self.writer.write(")");
                return Ok(());
            }
            "udp_send_to" if self.std_net_imports.contains("udp_send_to") => {
                self.writer.write("rl_net_udp_send_to(");
                self.compile_expr(args[0])?;
                self.writer.write(", ");
                self.compile_expr(args[1])?;
                self.writer.write(", ");
                self.compile_expr(args[2])?;
                self.writer.write(")");
                return Ok(());
            }
            "udp_recv" if self.std_net_imports.contains("udp_recv") => {
                self.writer.write("rl_net_udp_recv(");
                self.compile_expr(args[0])?;
                self.writer.write(", ");
                self.compile_expr(args[1])?;
                self.writer.write(")");
                return Ok(());
            }
            "udp_recv_from" if self.std_net_imports.contains("udp_recv_from") => {
                self.writer.write("rl_net_udp_recv_from(");
                self.compile_expr(args[0])?;
                self.writer.write(", ");
                self.compile_expr(args[1])?;
                self.writer.write(")");
                return Ok(());
            }
            "udp_close" if self.std_net_imports.contains("udp_close") => {
                self.writer.write("rl_net_udp_close(");
                self.compile_expr(args[0])?;
                self.writer.write(")");
                return Ok(());
            }
            "resolve" if self.std_net_imports.contains("resolve") => {
                self.writer.write("rl_net_resolve(");
                self.compile_expr(args[0])?;
                self.writer.write(")");
                return Ok(());
            }
            "http_server_start" if self.std_http_imports.contains("http_server_start") => {
                self.writer.write("rl_http_server_start(");
                self.compile_expr(args[0])?;
                self.writer.write(")");
                return Ok(());
            }
            "http_server_recv" if self.std_http_imports.contains("http_server_recv") => {
                self.writer.write("rl_http_server_recv(");
                self.compile_expr(args[0])?;
                self.writer.write(")");
                return Ok(());
            }
            "http_server_try_recv" if self.std_http_imports.contains("http_server_try_recv") => {
                self.writer.write("rl_http_server_try_recv(");
                self.compile_expr(args[0])?;
                self.writer.write(")");
                return Ok(());
            }
            "http_server_stop" if self.std_http_imports.contains("http_server_stop") => {
                self.writer.write("rl_http_server_stop(");
                self.compile_expr(args[0])?;
                self.writer.write(")");
                return Ok(());
            }
            "http_request_method" if self.std_http_imports.contains("http_request_method") => {
                self.writer.write("rl_http_request_method(");
                self.compile_expr(args[0])?;
                self.writer.write(")");
                return Ok(());
            }
            "http_request_url" if self.std_http_imports.contains("http_request_url") => {
                self.writer.write("rl_http_request_url(");
                self.compile_expr(args[0])?;
                self.writer.write(")");
                return Ok(());
            }
            "http_request_header" if self.std_http_imports.contains("http_request_header") => {
                self.writer.write("rl_http_request_header(");
                self.compile_expr(args[0])?;
                self.writer.write(", ");
                self.compile_expr(args[1])?;
                self.writer.write(")");
                return Ok(());
            }
            "http_request_body" if self.std_http_imports.contains("http_request_body") => {
                self.writer.write("rl_http_request_body(");
                self.compile_expr(args[0])?;
                self.writer.write(")");
                return Ok(());
            }
            "http_respond" if self.std_http_imports.contains("http_respond") => {
                self.writer.write("rl_http_respond(");
                self.compile_expr(args[0])?;
                self.writer.write(", ");
                self.compile_expr(args[1])?;
                self.writer.write(", ");
                self.compile_expr(args[2])?;
                if args.len() >= 4 {
                    self.writer.write(", ");
                    self.compile_expr(args[3])?;
                    self.writer.write(", 1");
                } else {
                    self.writer.write(", rl_str_literal(\"\", 0), 0");
                }
                self.writer.write(")");
                return Ok(());
            }
            "http_get" if self.std_http_imports.contains("http_get") => {
                self.writer.write("rl_http_get(");
                self.compile_expr(args[0])?;
                self.writer.write(")");
                return Ok(());
            }
            "http_post" if self.std_http_imports.contains("http_post") => {
                self.writer.write("rl_http_post(");
                self.compile_expr(args[0])?;
                self.writer.write(", ");
                self.compile_expr(args[1])?;
                if args.len() >= 3 {
                    self.writer.write(", ");
                    self.compile_expr(args[2])?;
                    self.writer.write(", 1");
                } else {
                    self.writer.write(", rl_str_literal(\"text/plain\", 10), 0");
                }
                self.writer.write(")");
                return Ok(());
            }
            "http_request" if self.std_http_imports.contains("http_request") => {
                self.writer.write("rl_http_request(");
                self.compile_expr(args[0])?;
                self.writer.write(", ");
                self.compile_expr(args[1])?;
                if args.len() >= 3 {
                    self.writer.write(", ");
                    self.compile_expr(args[2])?;
                    self.writer.write(", 1");
                } else {
                    self.writer.write(", rl_str_literal(\"\", 0), 0");
                }
                if args.len() >= 4 {
                    self.writer.write(", ");
                    self.compile_expr(args[3])?;
                    self.writer.write(", 1");
                } else {
                    self.writer.write(", rl_str_literal(\"\", 0), 0");
                }
                self.writer.write(")");
                return Ok(());
            }
            "compile" if self.std_c_imports.contains("compile") => {
                self.writer.write("rl_c_compile(");
                self.compile_expr(args[0])?;
                self.writer.write(")");
                return Ok(());
            }
            "load" if self.std_c_imports.contains("load") => {
                self.writer.write("rl_c_load(");
                self.compile_expr(args[0])?;
                self.writer.write(")");
                return Ok(());
            }
            "has_symbol" if self.std_c_imports.contains("has_symbol") => {
                self.writer.write("rl_c_has_symbol(");
                self.compile_expr(args[0])?;
                self.writer.write(", ");
                self.compile_expr(args[1])?;
                self.writer.write(")");
                return Ok(());
            }
            "close" if self.std_c_imports.contains("close") => {
                self.writer.write("rl_c_close(");
                self.compile_expr(args[0])?;
                self.writer.write(")");
                return Ok(());
            }
            "clear_cache" if self.std_c_imports.contains("clear_cache") => {
                self.writer.write("rl_c_clear_cache()");
                return Ok(());
            }
            "call" if self.std_c_imports.contains("call") => {
                // call(handle, fn_name, args_tuple, arg_types_array, ret_type_string)
                // Emit: rl_c_call(handle_id, fn_name, argc, argv, arg_types, ret_type)
                self.writer.write("rl_c_call(");
                self.compile_expr(args[0])?;
                self.writer.write(", ");
                self.compile_expr(args[1])?;
                // Count args from the tuple literal
                let args_expr = self.ast.exprs.get(args[2]);
                if let ExpressionKind::TupleLiteral(elems) = &args_expr.kind {
                    self.writer.write(&format!(", {}l", elems.len()));
                    // Emit argv array
                    self.writer.write(", (void*[]){");
                    for (i, elem) in elems.iter().enumerate() {
                        if i > 0 { self.writer.write(", "); }
                        self.writer.write("(void*)(intptr_t)(");
                        self.compile_expr(*elem)?;
                        self.writer.write(")");
                    }
                    self.writer.write("}");
                } else {
                    self.writer.write(", 0, NULL");
                }
                // Emit arg_types as string array
                let types_expr = self.ast.exprs.get(args[3]);
                if let ExpressionKind::ArrayLiteral(elems) = &types_expr.kind {
                    self.writer.write(", (const char*[]){");
                    for (i, elem) in elems.iter().enumerate() {
                        if i > 0 { self.writer.write(", "); }
                        let e = self.ast.exprs.get(*elem);
                        if let ExpressionKind::String(s) = &e.kind {
                            self.writer.write(&format!("\"{}\"", s));
                        }
                    }
                    self.writer.write("}");
                } else {
                    self.writer.write(", NULL");
                }
                self.writer.write(", ");
                self.compile_expr(args[4])?;
                self.writer.write(")");
                return Ok(());
            }
            _ => {}
        }

        let c_name = mangle(&path.join("_"));
        self.writer.write(&format!("{}(", c_name));
        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                self.writer.write(", ");
            }
            self.compile_expr(*arg)?;
        }
        self.writer.write(")");
        Ok(())
    }

    pub fn compile_method_call(
        &mut self,
        caller: ExprId,
        method: &[String],
        args: &[ExprId],
    ) -> Result<(), Error> {
        let method_name = method.first().map(|s| s.as_str()).unwrap_or("");

        match method_name {
            "len" => {
                self.writer.write("rl_str_len(");
                self.compile_expr(caller)?;
                self.writer.write(")");
                return Ok(());
            }
            "println" => {
                self.writer.write("rl_println(");
                self.compile_expr(caller)?;
                self.writer.write(")");
                return Ok(());
            }
            "print" => {
                self.writer.write("rl_print(");
                self.compile_expr(caller)?;
                self.writer.write(")");
                return Ok(());
            }
            _ => {}
        }

        let caller_expr = self.ast.exprs.get(caller);
        if let ExpressionKind::ResolvedIdentifier { name, .. } = &caller_expr.kind
            && let Some(ta) = self.var_types.get(name)
                && let TypeAnnotation::Record(rname) | TypeAnnotation::CRecord(rname) = ta {
                    let c_name = self.lookup(name);
                    let c_fn = format!("impl_{}_{}", rname, method_name);
                    self.writer.write(&format!("{}({}", c_fn, c_name));
                    for arg in args.iter() {
                        self.writer.write(", ");
                        self.compile_expr(*arg)?;
                    }
                    self.writer.write(")");
                    return Ok(());
                }

        self.compile_expr(caller)?;
        self.writer.write(&format!(".{}(", method_name));
        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                self.writer.write(", ");
            }
            self.compile_expr(*arg)?;
        }
        self.writer.write(")");
        Ok(())
    }

    fn compile_lambda(
        &mut self,
        params: &[rl_ast::statements::Param],
        return_type: &Option<rl_ast::statements::TypeAnnotation>,
        body: &[rl_ast::statements::Statement],
    ) -> Result<(), Error> {
        let lambda_id = self.lambda_counter;
        self.lambda_counter += 1;
        let fn_name = format!("_rl_lambda_{}", lambda_id);

        // Collect captured variables from enclosing scopes
        let mut captured_names: Vec<String> = Vec::new();
        let param_names: std::collections::HashSet<String> =
            params.iter().map(|p| p.param_name.clone()).collect();
        self.collect_captures_from_statements(body, &param_names, &mut captured_names);
        captured_names.sort();
        captured_names.dedup();

        // Build the static function
        let _c_ret = match return_type {
            Some(ta) => type_to_c(ta),
            None => "rl_result".to_string(),
        };

        let mut func_code = String::new();
        func_code.push_str(&format!("static rl_result {}(rl_closure *_self, rl_result *_args, uint64_t _argc) {{\n", fn_name));

        // Declare parameters from _args
        for (i, p) in params.iter().enumerate() {
            let c_type = type_to_c(&p.param_type);
            let c_name = mangle(&p.param_name);
            func_code.push_str(&format!("    {} {} = ", c_type, c_name));
            // Unwrap from rl_result based on type
            match &p.param_type {
                TypeAnnotation::Int | TypeAnnotation::CInt => {
                    func_code.push_str(&format!("rl_unwrap_i64(_args[{}]);\n", i));
                }
                TypeAnnotation::Float | TypeAnnotation::CFloat => {
                    func_code.push_str(&format!("rl_unwrap_f64(_args[{}]);\n", i));
                }
                TypeAnnotation::Bool | TypeAnnotation::CBool => {
                    func_code.push_str(&format!("rl_unwrap_bool(_args[{}]);\n", i));
                }
                TypeAnnotation::String | TypeAnnotation::CString => {
                    func_code.push_str(&format!("rl_unwrap_str(_args[{}]);\n", i));
                }
                _ => {
                    // Default: unwrap as rl_result and extract
                    func_code.push_str(&format!("rl_unwrap_i64(_args[{}]);\n", i));
                }
            }
        }

        // Declare captured variables from _self->captures
        for (i, name) in captured_names.iter().enumerate() {
            let c_name = mangle(name);
            let c_type = self.var_types.get(name).map(type_to_c).unwrap_or_else(|| "int64_t".to_string());
            func_code.push_str(&format!("    {} {} = ", c_type, c_name));
            match self.var_types.get(name) {
                Some(TypeAnnotation::Int) | Some(TypeAnnotation::CInt) => {
                    func_code.push_str(&format!("rl_unwrap_i64(_self->captures[{}]);\n", i));
                }
                Some(TypeAnnotation::Float) | Some(TypeAnnotation::CFloat) => {
                    func_code.push_str(&format!("rl_unwrap_f64(_self->captures[{}]);\n", i));
                }
                Some(TypeAnnotation::Bool) | Some(TypeAnnotation::CBool) => {
                    func_code.push_str(&format!("rl_unwrap_bool(_self->captures[{}]);\n", i));
                }
                Some(TypeAnnotation::String) | Some(TypeAnnotation::CString) => {
                    func_code.push_str(&format!("rl_unwrap_str(_self->captures[{}]);\n", i));
                }
                Some(TypeAnnotation::Array(_)) | Some(TypeAnnotation::CArray(_)) => {
                    func_code.push_str(&format!("rl_unwrap_arr(_self->captures[{}]);\n", i));
                }
                _ => {
                    func_code.push_str(&format!("rl_unwrap_i64(_self->captures[{}]);\n", i));
                }
            }
        }

        // Compile the body into the function
        for s in body {
            self.compile_lambda_statement(s, &mut func_code)?;
        }

        func_code.push_str("    return rl_ok_null();\n");
        func_code.push_str("}\n\n");

        self.static_funcs.push(func_code);

        // Emit closure creation at the call site
        let mut captures_code = String::new();
        for (i, name) in captured_names.iter().enumerate() {
            if i > 0 { captures_code.push_str(", "); }
            let c_name = self.lookup(name);
            let c_type = self.var_types.get(name).cloned().unwrap_or(TypeAnnotation::Int);
            match c_type {
                TypeAnnotation::Int | TypeAnnotation::CInt => {
                    captures_code.push_str(&format!("rl_ok_i64({})", c_name));
                }
                TypeAnnotation::Float | TypeAnnotation::CFloat => {
                    captures_code.push_str(&format!("rl_ok_f64({})", c_name));
                }
                TypeAnnotation::Bool | TypeAnnotation::CBool => {
                    captures_code.push_str(&format!("rl_ok_bool({})", c_name));
                }
                TypeAnnotation::String | TypeAnnotation::CString => {
                    captures_code.push_str(&format!("rl_ok_str({})", c_name));
                }
                TypeAnnotation::Array(_) | TypeAnnotation::CArray(_) => {
                    captures_code.push_str(&format!("rl_ok_arr({})", c_name));
                }
                _ => {
                    captures_code.push_str(&format!("rl_ok_i64({})", c_name));
                }
            }
        }

        let capture_count = captured_names.len();
        self.writer.write(&format!(
            "rl_closure_new({}, (rl_result[]){{ {} }}, {})",
            fn_name, captures_code, capture_count
        ));

        Ok(())
    }

    fn compile_lambda_statement(
        &mut self,
        stmt: &rl_ast::statements::Statement,
        func_code: &mut String,
    ) -> Result<(), Error> {
        use rl_ast::statements::StatementKind;
        match &stmt.kind {
            StatementKind::Return(Some(expr_id)) => {
                let expr = self.ast.exprs.get(*expr_id);
                if let ExpressionKind::Propagate(inner) = &expr.kind {
                    let temp = self.temp_var();
                    func_code.push_str(&format!("    rl_result {} = ", temp));
                    self.compile_expr_to_string(*inner, func_code)?;
                    func_code.push_str(";\n");
                    func_code.push_str(&format!("    if (!{}.is_ok) {{ return {}; }}\n", temp, temp));
                    func_code.push_str(&format!("    return {};\n", temp));
                } else {
                    func_code.push_str("    return rl_ok(");
                    self.compile_expr_to_string(*expr_id, func_code)?;
                    func_code.push_str(");\n");
                }
            }
            StatementKind::ResolvedVariableDeclaration {
                name,
                type_annotation,
                value,
                ..
            } => {
                let c_type = type_to_c(type_annotation);
                let c_name = mangle(name);
                func_code.push_str(&format!("    {} {} = ", c_type, c_name));
                self.compile_expr_to_string(*value, func_code)?;
                func_code.push_str(";\n");
            }
            StatementKind::ResolvedConstantDeclaration {
                name,
                type_annotation,
                value,
                ..
            } => {
                let c_type = type_to_c(type_annotation);
                let c_name = mangle(name);
                func_code.push_str(&format!("    const {} {} = ", c_type, c_name));
                self.compile_expr_to_string(*value, func_code)?;
                func_code.push_str(";\n");
            }
            StatementKind::Expression(expr_id) => {
                func_code.push_str("    ");
                self.compile_expr_to_string(*expr_id, func_code)?;
                func_code.push_str(";\n");
            }
            StatementKind::Conditional { if_branch, else_branch } => {
                // if_branch is a ConditionalBranch with condition and body
                if let StatementKind::ConditionalBranch { condition, body, .. } = &if_branch.kind {
                    func_code.push_str("    if (");
                    if let Some(cond) = condition {
                        self.compile_expr_to_string(*cond, func_code)?;
                    } else {
                        func_code.push('1');
                    }
                    func_code.push_str(") {\n");
                    for s in body {
                        self.compile_lambda_statement(s, func_code)?;
                    }
                }
                if let Some(else_b) = else_branch
                    && let StatementKind::ConditionalBranch { condition, body, .. } = &else_b.kind {
                        if condition.is_some() {
                            // else-if chain
                            func_code.push_str("    } else if (");
                            self.compile_expr_to_string(condition.unwrap(), func_code)?;
                            func_code.push_str(") {\n");
                        } else {
                            func_code.push_str("    } else {\n");
                        }
                        for s in body {
                            self.compile_lambda_statement(s, func_code)?;
                        }
                    }
                func_code.push_str("    }\n");
            }
            StatementKind::ResolvedForRange {
                variable,
                range,
                body,
                ..
            } => {
                let items = match &range.kind {
                    StatementKind::Range(items) => items.clone(),
                    _ => vec![],
                };
                if !items.is_empty() {
                    let first = items[0];
                    let last = items[items.len() - 1];
                    let c_name = mangle(variable);
                    func_code.push_str(&format!(
                        "    for (int64_t {} = {}; {} < {}; {}++) {{\n",
                        c_name, first, c_name, last + 1, c_name
                    ));
                    for s in body {
                        self.compile_lambda_statement(s, func_code)?;
                    }
                    func_code.push_str("    }\n");
                }
            }
            StatementKind::Loop(body) => {
                func_code.push_str("    while (1) {\n");
                for s in body {
                    self.compile_lambda_statement(s, func_code)?;
                }
                func_code.push_str("    }\n");
            }
            _ => {
                func_code.push_str("    /* unhandled statement in lambda */\n");
            }
        }
        Ok(())
    }

    fn compile_expr_to_string(&mut self, id: ExprId, output: &mut String) -> Result<(), Error> {
        // Temporarily swap writer to capture output
        let old_source = std::mem::take(&mut self.writer);
        self.writer = CWriter::new();
        self.compile_expr(id)?;
        let generated = self.writer.source().to_string();
        self.writer = old_source;
        output.push_str(&generated);
        Ok(())
    }

    fn collect_captures_from_statements(
        &self,
        stmts: &[rl_ast::statements::Statement],
        param_names: &std::collections::HashSet<String>,
        captured: &mut Vec<String>,
    ) {
        use rl_ast::statements::StatementKind;
        for stmt in stmts {
            match &stmt.kind {
                StatementKind::ResolvedVariableDeclaration { name: _, value, .. } => {
                    self.collect_captures_from_expr(*value, param_names, captured);
                }
                StatementKind::ResolvedConstantDeclaration { name: _, value, .. } => {
                    self.collect_captures_from_expr(*value, param_names, captured);
                }
                StatementKind::Expression(expr_id) => {
                    self.collect_captures_from_expr(*expr_id, param_names, captured);
                }
                StatementKind::Return(Some(expr_id)) => {
                    self.collect_captures_from_expr(*expr_id, param_names, captured);
                }
                StatementKind::Conditional { if_branch, else_branch } => {
                    if let StatementKind::ConditionalBranch { condition, body, .. } = &if_branch.kind {
                        if let Some(cond) = condition {
                            self.collect_captures_from_expr(*cond, param_names, captured);
                        }
                        self.collect_captures_from_statements(body, param_names, captured);
                    }
                    if let Some(else_b) = else_branch
                        && let StatementKind::ConditionalBranch { condition, body, .. } = &else_b.kind {
                            if let Some(cond) = condition {
                                self.collect_captures_from_expr(*cond, param_names, captured);
                            }
                            self.collect_captures_from_statements(body, param_names, captured);
                        }
                }
                StatementKind::ResolvedForRange { body, range, .. } => {
                    self.collect_captures_from_statements(body, param_names, captured);
                    if let StatementKind::Range(_items) = &range.kind {
                        // Range items are integer literals, no captures
                    }
                }
                StatementKind::Loop(body) => {
                    self.collect_captures_from_statements(body, param_names, captured);
                }
                _ => {}
            }
        }
    }

    fn collect_captures_from_expr(
        &self,
        id: ExprId,
        param_names: &std::collections::HashSet<String>,
        captured: &mut Vec<String>,
    ) {
        let kind = self.ast.exprs.get(id).kind.clone();
        match &kind {
            ExpressionKind::ResolvedIdentifier { name, .. } => {
                // Only capture if it's NOT a parameter and NOT declared locally in the lambda
                // We check if it's in the outer scope by seeing if it's NOT in param_names
                // and NOT a local declaration (we track this via var_types which only has outer scope)
                if !param_names.contains(name) {
                    // Check if this variable exists in the type checker's scope (outer scope)
                    if (self.var_types.contains_key(name) || self.scopes.iter().rev().any(|s| s.contains_key(name)))
                        && !captured.contains(name) {
                            captured.push(name.clone());
                        }
                }
            }
            ExpressionKind::Binary { left, right, .. } => {
                self.collect_captures_from_expr(*left, param_names, captured);
                self.collect_captures_from_expr(*right, param_names, captured);
            }
            ExpressionKind::Unary { operand, .. } => {
                self.collect_captures_from_expr(*operand, param_names, captured);
            }
            ExpressionKind::Call { args, .. } => {
                for arg in args {
                    self.collect_captures_from_expr(*arg, param_names, captured);
                }
            }
            ExpressionKind::CallExpr { callee, args } => {
                self.collect_captures_from_expr(*callee, param_names, captured);
                for arg in args {
                    self.collect_captures_from_expr(*arg, param_names, captured);
                }
            }
            ExpressionKind::MethodCall { caller, args, .. } => {
                self.collect_captures_from_expr(*caller, param_names, captured);
                for arg in args {
                    self.collect_captures_from_expr(*arg, param_names, captured);
                }
            }
            ExpressionKind::ArrayLiteral(elems) => {
                for elem in elems {
                    self.collect_captures_from_expr(*elem, param_names, captured);
                }
            }
            ExpressionKind::MapLiteral(entries) => {
                for (_, v) in entries {
                    self.collect_captures_from_expr(*v, param_names, captured);
                }
            }
            ExpressionKind::SetLiteral(items) => {
                for item in items {
                    self.collect_captures_from_expr(*item, param_names, captured);
                }
            }
            ExpressionKind::TupleLiteral(elems) => {
                for elem in elems {
                    self.collect_captures_from_expr(*elem, param_names, captured);
                }
            }
            ExpressionKind::StructLiteral { fields, .. } => {
                for (_, v) in fields {
                    self.collect_captures_from_expr(*v, param_names, captured);
                }
            }
            ExpressionKind::Index { target, index } => {
                self.collect_captures_from_expr(*target, param_names, captured);
                self.collect_captures_from_expr(*index, param_names, captured);
            }
            ExpressionKind::IndexAssign { target, index, value } => {
                self.collect_captures_from_expr(*target, param_names, captured);
                self.collect_captures_from_expr(*index, param_names, captured);
                self.collect_captures_from_expr(*value, param_names, captured);
            }
            ExpressionKind::FieldAccess { target, .. } => {
                self.collect_captures_from_expr(*target, param_names, captured);
            }
            ExpressionKind::FieldAssign { target, field: _, value } => {
                self.collect_captures_from_expr(*target, param_names, captured);
                self.collect_captures_from_expr(*value, param_names, captured);
            }
            ExpressionKind::Grouping(inner) => {
                self.collect_captures_from_expr(*inner, param_names, captured);
            }
            ExpressionKind::OkLiteral(inner) | ExpressionKind::ErrLiteral(inner) | ExpressionKind::ErrorLiteral(inner) => {
                self.collect_captures_from_expr(*inner, param_names, captured);
            }
            ExpressionKind::Propagate(inner) => {
                self.collect_captures_from_expr(*inner, param_names, captured);
            }
            ExpressionKind::Cast { value, .. } => {
                self.collect_captures_from_expr(*value, param_names, captured);
            }
            ExpressionKind::ResolvedAssign { value, .. } => {
                self.collect_captures_from_expr(*value, param_names, captured);
            }
            _ => {}
        }
    }

    fn write_arg_as_result(&mut self, id: ExprId) -> Result<(), Error> {
        let kind = self.ast.exprs.get(id).kind.clone();
        // Try to determine the type and wrap appropriately
        match &kind {
            ExpressionKind::Integer(v) => {
                self.writer.write(&format!("rl_ok_i64((int64_t){})", v));
            }
            ExpressionKind::Float(v) => {
                self.writer.write(&format!("rl_ok_f64((double){})", v));
            }
            ExpressionKind::Bool(v) => {
                self.writer.write(if *v { "rl_ok_bool(true)" } else { "rl_ok_bool(false)" });
            }
            ExpressionKind::String(v) => {
                let escaped = escape_c_string(v);
                self.writer.write(&format!("rl_ok_str(rl_str_literal(\"{}\", {}))", escaped, escaped.len()));
            }
            ExpressionKind::ResolvedIdentifier { name, .. } => {
                // Look up the type and wrap accordingly
                if let Some(ta) = self.var_types.get(name) {
                    let c_name = self.lookup(name);
                    match ta {
                        TypeAnnotation::Int | TypeAnnotation::CInt => {
                            self.writer.write(&format!("rl_ok_i64({})", c_name));
                        }
                        TypeAnnotation::Float | TypeAnnotation::CFloat => {
                            self.writer.write(&format!("rl_ok_f64({})", c_name));
                        }
                        TypeAnnotation::Bool | TypeAnnotation::CBool => {
                            self.writer.write(&format!("rl_ok_bool({})", c_name));
                        }
                        TypeAnnotation::String | TypeAnnotation::CString => {
                            self.writer.write(&format!("rl_ok_str({})", c_name));
                        }
                        TypeAnnotation::Array(_) | TypeAnnotation::CArray(_) => {
                            self.writer.write(&format!("rl_ok_arr({})", c_name));
                        }
                        _ => {
                            self.writer.write(&format!("rl_ok_i64({})", c_name));
                        }
                    }
                } else {
                    // Default: just pass as is, hope for the best
                    self.compile_expr(id)?;
                }
            }
            _ => {
                // For complex expressions, wrap in rl_ok
                self.writer.write("rl_ok(");
                self.compile_expr(id)?;
                self.writer.write(")");
            }
        }
        Ok(())
    }

    fn infer_expr_type(&self, kind: &ExpressionKind) -> TypeAnnotation {
        match kind {
            ExpressionKind::Integer(_) => TypeAnnotation::Int,
            ExpressionKind::Float(_) => TypeAnnotation::Float,
            ExpressionKind::Bool(_) => TypeAnnotation::Bool,
            ExpressionKind::String(_) => TypeAnnotation::String,
            ExpressionKind::Character(_) => TypeAnnotation::Char,
            ExpressionKind::ArrayLiteral(elems) if !elems.is_empty() => TypeAnnotation::Array(Box::new(TypeAnnotation::Infer)),
            ExpressionKind::ResolvedIdentifier { name, .. } => {
                self.var_types.get(name).cloned().unwrap_or(TypeAnnotation::Int)
            }
            _ => TypeAnnotation::Int,
        }
    }
}
