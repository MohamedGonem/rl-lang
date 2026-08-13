//! Variable and constant declaration for the type checker scope.

use crate::{
    structs::{CheckType, ScopeItem, TypeChecker},
    units::Unit,
};
use rl_utils::span::Span;

impl TypeChecker {
    /// Declares `name` in the current (innermost) scope.
    ///
    /// For constants, emits an error if the name is already declared in the
    /// same scope. On success, pushes a hover entry showing the kind, name,
    /// and type.
    pub fn declare(&mut self, name: String, item_type: CheckType, is_const: bool, span: Span) {
        self.declare_with_unit(name, item_type, None, is_const, span);
    }

    /// Declares `name` together with a compile-time unit of measure.
    ///
    /// Identical to [`TypeChecker::declare`], but also stores the optional
    /// unit attached to numeric declarations (`dec float speed: m/s = 12.5`).
    pub fn declare_with_unit(
        &mut self,
        name: String,
        item_type: CheckType,
        unit: Option<Unit>,
        is_const: bool,
        span: Span,
    ) {
        if let Some(scope) = self.scopes.last_mut() {
            if is_const && scope.contains_key(&name) {
                self.errors
                    .push(self.err(format!("'{}' is already declared", name), span));
                return;
            }

            let kind = if is_const { "const" } else { "variable" };
            let unit_suffix = unit
                .as_ref()
                .map(|u| format!("[{}]", u))
                .unwrap_or_default();
            let hover_text = format!(
                "```rl\n{} {}: {}{}\n```",
                kind,
                name,
                item_type.info(),
                unit_suffix
            );

            scope.insert(
                name,
                ScopeItem::new_with_unit(item_type, unit, is_const, span),
            );
            self.push_hover(span, hover_text);
        }
    }
}
