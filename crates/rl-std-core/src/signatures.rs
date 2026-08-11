//! Typed `(params, return_type)` signatures for `std` functions and the
//! module tree that holds them (rl-lang#250).
//!
//! Moved here from `rl-commons` so the single stdlib source (`rl-std`) can
//! emit signatures next to the implementations via `#[native_fn]`, and the
//! checker/LSP can consume them without depending on either runtime.
//!
//! Each entry in [`StdFn::signatures`] is one accepted overload:
//! `(params, return_type)`. `params` is a [`TypeAnnotation::Tuple`] listing the
//! expected argument types in order (an empty tuple means no arguments).
//! Several functions accept more than one combination of argument types (e.g.
//! `pow(int, int)`, `pow(int, float)`, ...) - that is why this is a `Vec`: the
//! checker tries each overload in turn and uses the first whose params match.
//!
//! A function with an **empty** `signatures` vec is "not yet typed": the
//! checker treats calls to it as fully permissive.

use rl_ast::statements::TypeAnnotation;
use std::collections::HashMap;

/// A `std` function's known signature(s), used by the checker to validate
/// call arguments and infer the result type statically.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StdFn {
    pub name: String,
    pub signatures: Vec<(TypeAnnotation, TypeAnnotation)>,
}

impl StdFn {
    /// A function with no recorded signature - calls to it are unchecked.
    pub fn untyped(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            signatures: Vec::new(),
        }
    }

    /// A function with one or more known `(params, return_type)` overloads.
    pub fn typed(
        name: impl Into<String>,
        signatures: Vec<(TypeAnnotation, TypeAnnotation)>,
    ) -> Self {
        Self {
            name: name.into(),
            signatures,
        }
    }
}

/// A named collection of [`StdFn`] signatures, optionally containing
/// sub-modules. Mirrors the runtime `Module` tree used for registration, but
/// carries only names + types (no implementations).
#[derive(Debug, Clone, Default)]
pub struct ModuleNames {
    pub name: String,
    pub functions: HashMap<String, StdFn>,
    pub submodules: HashMap<String, ModuleNames>,
}

impl ModuleNames {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            functions: HashMap::new(),
            submodules: HashMap::new(),
        }
    }

    /// Bulk-adds function names with no signature (unchecked calls).
    pub fn with_functions(mut self, names: &[&str]) -> Self {
        self.functions
            .extend(names.iter().map(|s| (s.to_string(), StdFn::untyped(*s))));
        self
    }

    /// Adds (or overwrites) a single function with a known signature.
    pub fn with_typed_function(mut self, f: StdFn) -> Self {
        self.functions.insert(f.name.clone(), f);
        self
    }

    pub fn with_module(mut self, m: ModuleNames) -> Self {
        self.submodules.insert(m.name.clone(), m);
        self
    }

    /// Resolves a full stdlib path (e.g. `std::io::print`) to its [`StdFn`].
    pub fn resolve(&self, path: &[String]) -> Option<&StdFn> {
        if path.is_empty() {
            return None;
        }
        let path = if path.first().map(String::as_str) == Some(self.name.as_str()) {
            &path[1..]
        } else {
            path
        };
        if path.is_empty() {
            return None;
        }
        let mut module = self;
        for seg in &path[..path.len() - 1] {
            module = module.submodules.get(seg)?;
        }
        module.functions.get(&path[path.len() - 1])
    }

    pub fn collect_fn_names(&self, out: &mut HashMap<String, StdFn>) {
        out.extend(self.functions.iter().map(|(k, v)| (k.clone(), v.clone())));
        for sub in self.submodules.values() {
            sub.collect_fn_names(out);
        }
    }
}
