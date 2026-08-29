use crate::codegen::CCodegen;
use crate::codegen::ops::token_to_c_op;
use crate::name_mangle::{escape_c_char, escape_c_string, mangle};
use crate::types::type_to_c;
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
                self.writer.write("0");
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
                self.writer.write(&c_name);
            }
            ExpressionKind::Identifier(name) => {
                let c_name = self.lookup(name);
                self.writer.write(&c_name);
            }
            ExpressionKind::Call { path, args } => {
                self.compile_func_call(path, args)?;
            }
            ExpressionKind::CallExpr { callee, args } => {
                self.compile_expr(*callee)?;
                self.writer.write("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.writer.write(", ");
                    }
                    self.compile_expr(*arg)?;
                }
                self.writer.write(")");
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
                self.writer.write(&format!("{}.data.ok_value", temp));
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
                        self.writer.write_indent();
                        self.writer.write(&format!(
                            "rl_map_set(&{}, \"{}\", ",
                            temp, key_str
                        ));
                        self.compile_expr(*val_id)?;
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
                    self.writer.write_indent();
                    self.writer.write(&format!("rl_set_add(&{}, ", temp));
                    self.compile_expr(*item_id)?;
                    self.writer.write(");\n");
                }
                self.writer.write(&temp);
            }
            ExpressionKind::TupleLiteral(elems) => {
                let tuple_name = format!("rl_tuple_{}", elems.len());
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
                self.writer.write(&format!("{} = ", c_name));
                self.compile_expr(*value)?;
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
                                    let print_fn = if is_ln { "rl_println" } else { "rl_print" };
                                    self.writer.write(&format!("{}_rl_tuple_{}({})", print_fn, elems.len(), c_name));
                                }
                                TypeAnnotation::Record(rname) => {
                                    let c_name = self.lookup(name);
                                    let print_fn = if is_ln { "rl_println" } else { "rl_print" };
                                    self.writer.write(&format!("{}_rl_Record_{}({})", print_fn, rname, c_name));
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
            | "exp" | "sign" | "degrees" | "radians" => {
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
                self.writer.write("rl_ok((int64_t)llabs(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
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
                self.writer.write("rl_ok(pow(");
                if args.len() >= 2 {
                    self.compile_expr(args[0])?;
                    self.writer.write(", ");
                    self.compile_expr(args[1])?;
                }
                self.writer.write("))");
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
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(".data.ok_value");
                return Ok(());
            }
            "result_unwrap_err" => {
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(".data.err_value");
                return Ok(());
            }
            "result_unwrap_or" => {
                self.writer.write("(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(".is_ok ? ");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(".data.ok_value : ");
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
                self.writer.write("rl_ok(rl_io_read_file(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
                return Ok(());
            }
            "read_lines" => {
                self.writer.write("rl_ok(rl_io_read_lines(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write("))");
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
                self.writer.write("rl_ok(rl_io_write_file(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write("))");
                return Ok(());
            }
            "append_file" => {
                self.writer.write("rl_ok(rl_io_append_file(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write("))");
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
            "is_bool" | "is_int" | "is_float" | "is_string" | "is_null" | "is_char" | "is_byte" | "is_error" => {
                self.writer.write("1");
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
            // ---- collections (extended) ----
            "set_add" => {
                self.writer.write("rl_ok(rl_set_add_s(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write("))");
                return Ok(());
            }
            "set_remove" => {
                self.writer.write("rl_ok(rl_set_remove_s(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
                self.writer.write("))");
                return Ok(());
            }
            "set_contains" => {
                self.writer.write("rl_set_contains_s(");
                if !args.is_empty() { self.compile_expr(args[0])?; }
                self.writer.write(", ");
                if args.len() >= 2 { self.compile_expr(args[1])?; }
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
}
