use crate::codegen::CCodegen;
use crate::name_mangle::mangle;
use crate::types::type_to_c;
use rl_ast::{ExprId, nodes::ExpressionKind, statements::*};
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
                self.var_types.insert(name.clone(), type_annotation.clone());
                let expr = self.ast.exprs.get(*value);
                if let ExpressionKind::Propagate(inner) = &expr.kind {
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
                    self.writer.write_indent();
                    self.writer.write(&format!("{} {} = {}.data.ok_value;\n", c_type, c_name, temp));
                } else {
                    self.writer.write_indent();
                    self.writer.write(&format!("{} {} = ", c_type, c_name));
                    self.compile_expr(*value)?;
                    self.writer.write(";\n");
                }
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
                let expr = self.ast.exprs.get(*value);
                if let ExpressionKind::Propagate(inner) = &expr.kind {
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
                    self.writer.write_indent();
                    self.writer
                        .write(&format!("const {} {} = {}.data.ok_value;\n", c_type, c_name, temp));
                } else {
                    self.writer.write_indent();
                    self.writer
                        .write(&format!("const {} {} = ", c_type, c_name));
                    self.compile_expr(*value)?;
                    self.writer.write(";\n");
                }
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
                self.writer.write_indent();
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
                self.writer.write(") {\n");
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
                self.writer.write_indent();
                self.writer.write("}\n\n");
            }
            StatementKind::Expression(expr_id) => {
                self.writer.write_indent();
                self.compile_expr(*expr_id)?;
                self.writer.write(";\n");
            }
            StatementKind::Return(Some(expr_id)) => {
                let expr = self.ast.exprs.get(*expr_id);
                if let ExpressionKind::Propagate(inner) = &expr.kind {
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
                    self.writer.write_indent();
                    self.writer.write(&format!("return {}.data.ok_value;\n", temp));
                } else {
                    self.writer.write_indent();
                    self.writer.write("return ");
                    self.compile_expr(*expr_id)?;
                    self.writer.write(";\n");
                }
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
                self.writer.write_indent();
                self.writer.write("while (");
                self.compile_expr(*condition)?;
                self.writer.write(") {\n");
                self.writer.indent();
                for s in body {
                    self.compile_statement(s)?;
                }
                self.writer.dedent();
                self.writer.write_indent();
                self.writer.write("}\n");
            }
            StatementKind::ResolvedFor {
                initializer,
                condition,
                increment,
                body,
            } => {
                self.writer.write_indent();
                self.writer.write("for (");
                self.compile_for_init(initializer)?;
                self.writer.write("; ");
                self.compile_expr(*condition)?;
                self.writer.write("; ");
                self.compile_expr(*increment)?;
                self.writer.write(") {\n");
                self.writer.indent();
                for s in body {
                    self.compile_statement(s)?;
                }
                self.writer.dedent();
                self.writer.write_indent();
                self.writer.write("}\n");
            }
            StatementKind::Break => self.writer.writeln("break;"),
            StatementKind::Continue => self.writer.writeln("continue;"),
            StatementKind::Match { value, arms } => {
                self.compile_match(*value, arms)?;
            }
            StatementKind::ResolvedArray {
                name,
                type_annotation,
                value,
                ..
            } => {
                let c_name = mangle(name);
                self.declare(name, &c_name);
                self.var_types.insert(
                    name.clone(),
                    TypeAnnotation::Array(Box::new(type_annotation.clone())),
                );
                self.writer.write_indent();
                self.writer
                    .write(&format!("rl_array {} = ", c_name));
                self.compile_expr(*value)?;
                self.writer.write(";\n");
            }
            StatementKind::ResolvedConstantArray {
                name,
                type_annotation,
                value,
                ..
            } => {
                let c_name = mangle(name);
                self.declare(name, &c_name);
                self.var_types.insert(
                    name.clone(),
                    TypeAnnotation::CArray(Box::new(type_annotation.clone())),
                );
                self.writer.write_indent();
                self.writer
                    .write(&format!("const rl_array {} = ", c_name));
                self.compile_expr(*value)?;
                self.writer.write(";\n");
            }
            StatementKind::ResolvedMap {
                name,
                type_annotation,
                value,
                ..
            } => {
                let c_name = mangle(name);
                self.declare(name, &c_name);
                self.var_types.insert(name.clone(), type_annotation.clone());
                self.writer.write_indent();
                self.writer
                    .write(&format!("rl_map {} = rl_map_new();\n", c_name));
                let map_expr = self.ast.exprs.get(*value);
                if let ExpressionKind::MapLiteral(entries) = &map_expr.kind {
                    for (key_id, val_id) in entries {
                        let key_expr = self.ast.exprs.get(*key_id);
                        if let ExpressionKind::String(key_str) = &key_expr.kind {
                            self.writer.write_indent();
                            self.writer.write(&format!(
                                "rl_map_set(&{}, \"{}\", ",
                                c_name, key_str
                            ));
                            self.compile_expr(*val_id)?;
                            self.writer.write(");\n");
                        }
                    }
                }
            }
            StatementKind::ResolvedConstantMap {
                name,
                type_annotation,
                value,
                ..
            } => {
                let c_name = mangle(name);
                self.declare(name, &c_name);
                self.var_types.insert(name.clone(), type_annotation.clone());
                self.writer.write_indent();
                self.writer.write(&format!(
                    "const rl_map {} = rl_map_new();\n",
                    c_name
                ));
                let map_expr = self.ast.exprs.get(*value);
                if let ExpressionKind::MapLiteral(entries) = &map_expr.kind {
                    for (key_id, val_id) in entries {
                        let key_expr = self.ast.exprs.get(*key_id);
                        if let ExpressionKind::String(key_str) = &key_expr.kind {
                            self.writer.write_indent();
                            self.writer.write(&format!(
                                "rl_map_set((rl_map*)&{}, \"{}\", ",
                                c_name, key_str
                            ));
                            self.compile_expr(*val_id)?;
                            self.writer.write(");\n");
                        }
                    }
                }
            }
            StatementKind::ResolvedSet {
                name,
                type_annotation,
                value,
                ..
            } => {
                let c_name = mangle(name);
                self.declare(name, &c_name);
                self.var_types.insert(name.clone(), type_annotation.clone());
                self.writer.write_indent();
                self.writer
                    .write(&format!("rl_set {} = rl_set_new();\n", c_name));
                let set_expr = self.ast.exprs.get(*value);
                if let ExpressionKind::SetLiteral(items) = &set_expr.kind {
                    for item_id in items {
                        self.writer.write_indent();
                        self.writer
                            .write(&format!("rl_set_add(&{}, ", c_name));
                        self.compile_expr(*item_id)?;
                        self.writer.write(");\n");
                    }
                }
            }
            StatementKind::ResolvedConstantSet {
                name,
                type_annotation,
                value,
                ..
            } => {
                let c_name = mangle(name);
                self.declare(name, &c_name);
                self.var_types.insert(name.clone(), type_annotation.clone());
                self.writer.write_indent();
                self.writer.write(&format!(
                    "const rl_set {} = rl_set_new();\n",
                    c_name
                ));
                let set_expr = self.ast.exprs.get(*value);
                if let ExpressionKind::SetLiteral(items) = &set_expr.kind {
                    for item_id in items {
                        self.writer.write_indent();
                        self.writer.write(&format!(
                            "rl_set_add((rl_set*)&{}, ",
                            c_name
                        ));
                        self.compile_expr(*item_id)?;
                        self.writer.write(");\n");
                    }
                }
            }
            StatementKind::ResolvedDestructureDeclaration {
                bindings,
                value,
                ..
            } => {
                let temp = self.temp_var();
                self.writer.write_indent();
                self.writer.write(&format!("rl_tuple_{} {} = ", bindings.len(), temp));
                self.compile_expr(*value)?;
                self.writer.write(";\n");
                for (i, (type_annotation, name)) in bindings.iter().enumerate() {
                    let c_type = type_to_c(type_annotation);
                    let c_name = mangle(name);
                    self.declare(name, &c_name);
                    self.var_types.insert(name.clone(), type_annotation.clone());
                    self.writer.write_indent();
                    self.writer.write(&format!(
                        "{} {} = {}.field_{};\n",
                        c_type, c_name, temp, i
                    ));
                }
            }
            StatementKind::ResolvedImplBlock { record, methods } => {
                for m in methods {
                    if let StatementKind::ResolvedFunctionDeclaration {
                        name,
                        params,
                        return_type,
                        body,
                        ..
                    } = &m.kind
                    {
                        let c_ret = type_to_c(return_type);
                        let c_fn_name = format!("impl_{}_{}", record, name);
                        self.writer.write_indent();
                        self.writer
                            .write(&format!("{} {}(", c_ret, c_fn_name));

                        let c_params: Vec<String> = params
                            .iter()
                            .map(|p| {
                                let c_type = type_to_c(&p.param_type);
                                let c_name = mangle(&p.param_name);
                                format!("{} {}", c_type, c_name)
                            })
                            .collect();
                        self.writer.write(&c_params.join(", "));
                        self.writer.write(") {\n");
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
                        self.writer.write_indent();
                        self.writer.write("}\n\n");
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn compile_for_init(&mut self, stmt: &Statement) -> Result<(), Error> {
        if let StatementKind::ResolvedVariableDeclaration {
            name,
            type_annotation,
            value,
            ..
        } = &stmt.kind
        {
            let c_type = type_to_c(type_annotation);
            let c_name = mangle(name);
            self.declare(name, &c_name);
            self.writer.write(&format!("{} {} = ", c_type, c_name));
            self.compile_expr(*value)?;
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
            self.writer.write_indent();
            if let Some(cond) = condition {
                self.writer.write("if (");
                self.compile_expr(*cond)?;
                self.writer.write(") {\n");
            } else {
                self.writer.write("{\n");
            }
            self.writer.indent();
            for s in body {
                self.compile_statement(s)?;
            }
            self.writer.dedent();
            self.writer.write_indent();
            if else_branch.is_some() {
                self.writer.write("} ");
            } else {
                self.writer.write("}\n");
            }
        }

        if let Some(else_stmt) = else_branch {
            self.write_else_branch(else_stmt)?;
        }

        Ok(())
    }

    fn write_else_branch(&mut self, else_stmt: &Statement) -> Result<(), Error> {
        match &else_stmt.kind {
            StatementKind::ConditionalBranch {
                condition, body, ..
            } => {
                if let Some(cond) = condition {
                    self.writer.write("else if (");
                    self.compile_expr(*cond)?;
                    self.writer.write(") {\n");
                } else {
                    self.writer.write("else {\n");
                }
                self.writer.indent();
                for s in body {
                    self.compile_statement(s)?;
                }
                self.writer.dedent();
                self.writer.write_indent();
                self.writer.write("}\n");
            }
            StatementKind::Conditional {
                if_branch,
                else_branch,
            } => {
                self.writer.write("else ");
                // Recurse but skip the indent since we're already on the } line
                if let StatementKind::ConditionalBranch {
                    condition, body, ..
                } = &if_branch.kind
                {
                    if let Some(cond) = condition {
                        self.writer.write("if (");
                        self.compile_expr(*cond)?;
                        self.writer.write(") {\n");
                    } else {
                        self.writer.write("{\n");
                    }
                    self.writer.indent();
                    for s in body {
                        self.compile_statement(s)?;
                    }
                    self.writer.dedent();
                    self.writer.write_indent();
                    if else_branch.is_some() {
                        self.writer.write("} ");
                    } else {
                        self.writer.write("}\n");
                    }
                    if let Some(inner_else) = else_branch {
                        self.write_else_branch(inner_else)?;
                    }
                }
            }
            _ => {
                self.writer.write("else {\n");
                self.writer.indent();
                self.compile_statement(else_stmt)?;
                self.writer.dedent();
                self.writer.write_indent();
                self.writer.write("}\n");
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
                    self.writer.write_indent();
                    if i == 0 {
                        self.writer.write("if (");
                    } else {
                        self.writer.write("else if (");
                    }
                    self.compile_expr(value)?;
                    self.writer.write(" == ");
                    self.compile_expr(*lit_id)?;
                    self.writer.write(") {\n");
                    self.writer.indent();
                    for s in body {
                        self.compile_statement(s)?;
                    }
                    self.writer.dedent();
                    self.writer.write_indent();
                    self.writer.write("}\n");
                }
                MatchPattern::Wildcard => {
                    self.writer.write_indent();
                    self.writer.write("else {\n");
                    self.writer.indent();
                    for s in body {
                        self.compile_statement(s)?;
                    }
                    self.writer.dedent();
                    self.writer.write_indent();
                    self.writer.write("}\n");
                }
            }
        }
        Ok(())
    }
}
