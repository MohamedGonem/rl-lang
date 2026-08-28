use crate::codegen::CCodegen;
use crate::codegen::ops::token_to_c_op;
use crate::name_mangle::{escape_c_char, escape_c_string, mangle};
use crate::types::type_to_c;
use rl_ast::{ExprId, nodes::ExpressionKind};
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
                self.writer.write("rl_arr_from_vals((");
                for (i, elem) in elems.iter().enumerate() {
                    if i > 0 {
                        self.writer.write(", ");
                    }
                    self.compile_expr(*elem)?;
                }
                self.writer.write(&format!("), {})", elems.len()));
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

        match func_name {
            "println" => {
                self.writer.write("rl_println(");
                if !args.is_empty() {
                    self.compile_expr(args[0])?;
                }
                self.writer.write(")");
                return Ok(());
            }
            "print" => {
                self.writer.write("rl_print(");
                if !args.is_empty() {
                    self.compile_expr(args[0])?;
                }
                self.writer.write(")");
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
