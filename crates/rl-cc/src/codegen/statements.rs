use crate::codegen::CCodegen;
use crate::name_mangle::mangle;
use crate::types::type_to_c;
use rl_ast::{ExprId, statements::*};
use rl_utils::errors::Error;

impl<'a> CCodegen<'a> {
    pub fn compile_statement(&mut self, stmt: &Statement) -> Result<(), Error> {
        match &stmt.kind {
            StatementKind::ResolvedVariableDeclaration {
                name,
                type_annotation,
                value,
                ..
            } => {
                let c_type = type_to_c(type_annotation);
                let c_name = mangle(name);
                self.declare(name, &c_name);
                self.writer.write(&format!("{} {} = ", c_type, c_name));
                self.compile_expr(*value)?;
                self.writer.writeln(";");
            }
            StatementKind::ResolvedConstantDeclaration {
                name,
                type_annotation,
                value,
                ..
            } => {
                let c_type = type_to_c(type_annotation);
                let c_name = mangle(name);
                self.declare(name, &c_name);
                self.writer
                    .write(&format!("const {} {} = ", c_type, c_name));
                self.compile_expr(*value)?;
                self.writer.writeln(";");
            }
            StatementKind::ResolvedFunctionDeclaration {
                name,
                params,
                return_type,
                body,
                ..
            } => {
                let c_ret = type_to_c(return_type);
                let c_name = mangle(name);
                self.writer.write(&format!("{} {}(", c_ret, c_name));

                let c_params: Vec<String> = params
                    .iter()
                    .map(|p| {
                        let c_type = type_to_c(&p.param_type);
                        let c_name = mangle(&p.param_name);
                        format!("{} {}", c_type, c_name)
                    })
                    .collect();
                self.writer.write(&c_params.join(", "));
                self.writer.writeln(") {");
                self.writer.indent();

                self.push_scope();
                for p in params {
                    self.declare(&p.param_name, &mangle(&p.param_name));
                }
                for s in body {
                    self.compile_statement(s)?;
                }
                self.pop_scope();
                self.writer.dedent();
                self.writer.writeln("}");
                self.writer.blank_line();
            }
            StatementKind::Expression(expr_id) => {
                self.compile_expr(*expr_id)?;
                self.writer.writeln(";");
            }
            StatementKind::Return(Some(expr_id)) => {
                self.writer.write("return ");
                self.compile_expr(*expr_id)?;
                self.writer.writeln(";");
            }
            StatementKind::Return(None) => {
                self.writer.writeln("return;");
            }
            StatementKind::Conditional {
                if_branch,
                else_branch,
            } => {
                self.write_conditional(if_branch, else_branch)?;
            }
            StatementKind::While { condition, body } => {
                self.writer.write("while (");
                self.compile_expr(*condition)?;
                self.writer.writeln(") {");
                self.writer.indent();
                for s in body {
                    self.compile_statement(s)?;
                }
                self.writer.dedent();
                self.writer.writeln("}");
            }
            StatementKind::ResolvedFor {
                initializer,
                condition,
                increment,
                body,
            } => {
                self.writer.write("for (");
                self.compile_statement(initializer)?;
                self.compile_expr(*condition)?;
                self.writer.write("; ");
                self.compile_expr(*increment)?;
                self.writer.writeln(") {");
                self.writer.indent();
                for s in body {
                    self.compile_statement(s)?;
                }
                self.writer.dedent();
                self.writer.writeln("}");
            }
            StatementKind::Break => self.writer.writeln("break;"),
            StatementKind::Continue => self.writer.writeln("continue;"),
            StatementKind::Match { value, arms } => {
                self.compile_match(*value, arms)?;
            }
            _ => {
                self.writer.writeln("/* unhandled statement */");
            }
        }
        Ok(())
    }

    pub fn write_conditional(
        &mut self,
        if_branch: &Statement,
        else_branch: &Option<Box<Statement>>,
    ) -> Result<(), Error> {
        if let StatementKind::ConditionalBranch {
            condition, body, ..
        } = &if_branch.kind
        {
            if let Some(cond) = condition {
                self.writer.write("if (");
                self.compile_expr(*cond)?;
                self.writer.writeln(") {");
            } else {
                self.writer.writeln("{");
            }
            self.writer.indent();
            for s in body {
                self.compile_statement(s)?;
            }
            self.writer.dedent();
            self.writer.writeln("}");
        }

        if let Some(else_stmt) = else_branch {
            match &else_stmt.kind {
                StatementKind::ConditionalBranch {
                    condition, body, ..
                } => {
                    if let Some(cond) = condition {
                        self.writer.write(" else if (");
                        self.compile_expr(*cond)?;
                        self.writer.writeln(") {");
                    } else {
                        self.writer.writeln(" else {");
                    }
                    self.writer.indent();
                    for s in body {
                        self.compile_statement(s)?;
                    }
                    self.writer.dedent();
                    self.writer.writeln("}");
                }
                StatementKind::Conditional {
                    if_branch,
                    else_branch,
                } => {
                    self.writer.write(" else ");
                    self.write_conditional(if_branch, else_branch)?;
                }
                _ => {
                    self.writer.writeln(" else {");
                    self.writer.indent();
                    self.compile_statement(else_stmt)?;
                    self.writer.dedent();
                    self.writer.writeln("}");
                }
            }
        }

        Ok(())
    }

    pub fn compile_match(
        &mut self,
        value: ExprId,
        arms: &[(MatchPattern, Vec<Statement>)],
    ) -> Result<(), Error> {
        for (i, (pattern, body)) in arms.iter().enumerate() {
            match pattern {
                MatchPattern::Literal(lit_id) => {
                    if i == 0 {
                        self.writer.write("if (");
                    } else {
                        self.writer.write("} else if (");
                    }
                    self.writer.write("(");
                    self.compile_expr(value)?;
                    self.writer.write(" == ");
                    self.compile_expr(*lit_id)?;
                    self.writer.writeln(")) {");
                    self.writer.indent();
                    for s in body {
                        self.compile_statement(s)?;
                    }
                    self.writer.dedent();
                }
                MatchPattern::Wildcard => {
                    self.writer.writeln("} else {");
                    self.writer.indent();
                    for s in body {
                        self.compile_statement(s)?;
                    }
                    self.writer.dedent();
                }
            }
        }
        self.writer.writeln("}");
        Ok(())
    }
}
