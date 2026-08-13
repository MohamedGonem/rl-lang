//! Assignment type checking - validates reassignment against the declared type.

use crate::{
    structs::{CheckedExpr, TypeChecker},
    units::Unit,
};
use rl_utils::{span::Span, suggest::closest_match};

impl TypeChecker {
    /// Checks that assigning `value` to `name` is valid.
    ///
    /// Walks scopes from innermost to outermost. Emits an error if:
    /// - The variable is declared `const`
    /// - The value type doesn't match the declared type
    /// - The value's unit isn't compatible with the declared unit
    /// - The name is not declared in any scope (with a "did you mean?" suggestion)
    pub fn assign(&mut self, name: &str, value: CheckedExpr, span: Span) {
        let mut const_error: Option<String> = None;
        let mut type_error: Option<String> = None;
        let mut unit_error: Option<String> = None;
        let mut found = false;

        for scope in self.scopes.iter_mut().rev() {
            if let Some(item) = scope.get_mut(name) {
                found = true;

                if item.is_const {
                    const_error = Some(format!("cannot assign to constant '{}'", name));
                } else {
                    if !value.ty.matches(&item.type_annotation) && !value.ty.is_null() {
                        type_error = Some(format!(
                            "cannot assign {} to variable '{}' declared as {}",
                            value.ty.info(),
                            name,
                            item.type_annotation.info(),
                        ));
                    }

                    if let Some(msg) = unit_mismatch(name, &value.unit, &item.unit) {
                        unit_error = Some(msg);
                    }
                }
                break;
            }
        }

        if let Some(msg) = const_error.or(type_error).or(unit_error) {
            self.error(msg, span);
            return;
        }

        if !found {
            let all_keys: Vec<String> = self
                .scopes
                .iter()
                .flat_map(|s| s.keys().cloned().collect::<Vec<_>>())
                .collect();
            let suggestion = closest_match(name, all_keys.iter().map(|s| s.as_str()));

            self.error_with_help(format!("undefined variable '{}'", name), span, suggestion);
        }
    }
}

/// Returns an error message if `value`'s unit isn't compatible with the
/// declared unit of the target binding.
///
/// Rules:
/// - A binding with no declared unit expects dimensionless values; assigning
///   a value carrying a non-dimensionless unit is an error.
/// - A binding with a declared unit accepts the same unit or a dimensionless
///   value (plain literals adopt the unit, F#-style).
fn unit_mismatch(
    name: &str,
    value: &Option<Unit>,
    declared: &Option<Unit>,
) -> Option<String> {
    let value_unit = value.as_ref();
    let declared_unit = declared.as_ref();

    match (value_unit, declared_unit) {
        // no declared unit: the binding is dimensionless, so a
        // non-dimensionless value cannot be stored in it
        (Some(v), None) if !v.is_dimensionless() => Some(format!(
            "cannot assign {} to variable '{}' declared without a unit",
            v, name
        )),
        // both units present but incompatible
        (Some(v), Some(d)) if !v.is_compatible_with(d) => {
            Some(format!("unit mismatch: expected {}, got {}", d, v))
        }
        // everything else is fine (value dimensionless, units equal, or the
        // value carries no unit)
        _ => None,
    }
}