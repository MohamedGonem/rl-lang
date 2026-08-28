use crate::codegen::CCodegen;
use crate::name_mangle::mangle;
use std::collections::HashMap;

impl<'a> CCodegen<'a> {
    pub fn declare(&mut self, rl_name: &str, c_name: &str) {
        self.scopes
            .last_mut()
            .unwrap()
            .insert(rl_name.to_string(), c_name.to_string());
    }

    pub fn lookup(&self, rl_name: &str) -> String {
        for scope in self.scopes.iter().rev() {
            if let Some(c_name) = scope.get(rl_name) {
                return c_name.clone();
            }
        }
        mangle(rl_name)
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn temp_var(&mut self) -> String {
        let name = format!("_r_{}", self.temp_counter);
        self.temp_counter += 1;
        name
    }
}
