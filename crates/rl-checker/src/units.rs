//! Compile-time units of measure.
//!
//! A [`Unit`] is a normalized product of base symbols each raised to an
//! integer power, e.g. `m/s` becomes `{ m: 1, s: -1 }`. Units are attached to
//! `int`/`float` (and friends) declarations and expressions by the type
//! checker, which enforces dimensional consistency:
//!
//! - `+` / `-` require compatible (or dimensionless) units
//! - `*` / `/` combine units by adding / subtracting exponents
//!
//! Units are a compile-time-only concept and are discarded after the checker
//! pass - they never reach the resolver, VM, or interpreter.

use rl_ast::statements::UnitAnnotation;
use std::collections::BTreeMap;
use std::fmt;

/// Normalized representation of a unit of measure.
///
/// Each base symbol is stored with its exponent:
///
/// - `m`       => `{ "m": 1 }`
/// - `m / s`   => `{ "m": 1, "s": -1 }`
/// - `m / s²`  => `{ "m": 1, "s": -2 }`
/// - `m * s²`  => `{ "m": 1, "s": 2 }`
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Unit {
    powers: BTreeMap<String, i32>,
}

impl Unit {
    /// Creates a dimensionless unit (e.g. `m / m`).
    pub fn dimensionless() -> Self {
        Self::default()
    }

    /// Creates a unit containing one symbol with exponent `1`.
    pub fn symbol(name: impl Into<String>) -> Self {
        Self::powers(name, 1)
    }

    /// Creates a unit containing one symbol with the given exponent.
    pub fn powers(name: impl Into<String>, power: i32) -> Self {
        let mut unit = Self::dimensionless();
        unit.add_exponent(&name.into(), power);
        unit
    }

    /// Returns the exponent associated with a symbol.
    ///
    /// Symbols not present in the unit have exponent `0`.
    pub fn exponent(&self, symbol: &str) -> i32 {
        self.powers.get(symbol).copied().unwrap_or(0)
    }

    /// Returns `true` if the unit is dimensionless.
    pub fn is_dimensionless(&self) -> bool {
        self.powers.is_empty()
    }

    /// Returns `true` if `self` and `other` can be added or subtracted.
    ///
    /// Two units are compatible when they are exactly equal, or when either
    /// side is dimensionless (a plain numeric literal may adopt any unit).
    pub fn is_compatible_with(&self, other: &Self) -> bool {
        self == other || self.is_dimensionless() || other.is_dimensionless()
    }

    /// Multiplies two units by adding their exponents.
    ///
    /// `(m / s) * s = m`
    pub fn multiply(&self, other: &Self) -> Self {
        let mut result = self.clone();

        for (symbol, exponent) in &other.powers {
            result.add_exponent(symbol, *exponent);
        }

        result
    }

    /// Divides two units by subtracting their exponents.
    ///
    /// `m / s = m * s⁻¹`
    pub fn divide(&self, other: &Self) -> Self {
        let mut result = self.clone();

        for (symbol, exponent) in &other.powers {
            result.add_exponent(symbol, -*exponent);
        }

        result
    }

    /// Constructs a `Unit` from a parsed [`UnitAnnotation`] syntax tree,
    /// recursively combining symbols with the multiplication and division
    /// rules above.
    pub fn from_annotation(annotation: &UnitAnnotation) -> Self {
        match annotation {
            UnitAnnotation::Symbol(name) => Self::symbol(name),

            UnitAnnotation::Multiply(left, right) => {
                Self::from_annotation(left).multiply(&Self::from_annotation(right))
            }

            UnitAnnotation::Divide(left, right) => {
                Self::from_annotation(left).divide(&Self::from_annotation(right))
            }
        }
    }

    fn add_exponent(&mut self, symbol: &str, amount: i32) {
        let new_exponent = self.exponent(symbol) + amount;

        if new_exponent == 0 {
            self.powers.remove(symbol);
        } else {
            self.powers.insert(symbol.to_owned(), new_exponent);
        }
    }
}

impl fmt::Display for Unit {
    /// Formats the unit in a human-readable form, e.g. `m/s`, `m*s²`, or
    /// `kg*m/s²`. A dimensionless unit formats as `1`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut numerator = Vec::new();
        let mut denominator = Vec::new();

        for (symbol, exponent) in &self.powers {
            if *exponent > 0 {
                numerator.push(format_power(symbol, *exponent));
            } else {
                denominator.push(format_power(symbol, -*exponent));
            }
        }

        if numerator.is_empty() {
            write!(f, "1")?;
        } else {
            write!(f, "{}", numerator.join("*"))?;
        }

        if !denominator.is_empty() {
            write!(f, "/{}", denominator.join("*"))?;
        }

        Ok(())
    }
}

fn format_power(symbol: &str, exponent: i32) -> String {
    if exponent == 1 {
        symbol.to_string()
    } else {
        format!("{symbol}^{exponent}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_symbol_unit() {
        let meters = Unit::symbol("m");

        assert_eq!(meters.exponent("m"), 1);
        assert_eq!(meters.exponent("s"), 0);
    }

    #[test]
    fn creates_power_unit() {
        let area = Unit::powers("m", 2);

        assert_eq!(area.exponent("m"), 2);
    }

    #[test]
    fn multiplies_units_and_cancels_symbols() {
        let meters = Unit::symbol("m");
        let seconds = Unit::symbol("s");

        let speed = meters.divide(&seconds);
        let distance = speed.multiply(&seconds);

        assert_eq!(distance, Unit::symbol("m"));
    }

    #[test]
    fn divides_units() {
        let meters = Unit::symbol("m");
        let seconds = Unit::symbol("s");

        let speed = meters.divide(&seconds);

        assert_eq!(speed.exponent("m"), 1);
        assert_eq!(speed.exponent("s"), -1);
    }

    #[test]
    fn combines_repeated_symbols() {
        let meters = Unit::symbol("m");

        let area = meters.multiply(&meters);

        assert_eq!(area.exponent("m"), 2);
    }

    #[test]
    fn produces_dimensionless_unit_when_symbols_cancel() {
        let meters = Unit::symbol("m");

        let result = meters.divide(&meters);

        assert!(result.is_dimensionless());
    }

    #[test]
    fn equal_units_are_compatible() {
        assert!(Unit::symbol("m").is_compatible_with(&Unit::symbol("m")));
        assert!(!Unit::symbol("m").is_compatible_with(&Unit::symbol("s")));
    }

    #[test]
    fn dimensionless_is_compatible_with_everything() {
        assert!(Unit::dimensionless().is_compatible_with(&Unit::symbol("m")));
        assert!(Unit::symbol("m").is_compatible_with(&Unit::dimensionless()));
        assert!(Unit::dimensionless().is_compatible_with(&Unit::dimensionless()));
    }

    #[test]
    fn normalizes_compound_annotation() {
        let annotation = UnitAnnotation::Divide(
            Box::new(UnitAnnotation::Multiply(
                Box::new(UnitAnnotation::Multiply(
                    Box::new(UnitAnnotation::Symbol("x".to_string())),
                    Box::new(UnitAnnotation::Symbol("y".to_string())),
                )),
                Box::new(UnitAnnotation::Symbol("r".to_string())),
            )),
            Box::new(UnitAnnotation::Symbol("i".to_string())),
        );

        let unit = Unit::from_annotation(&annotation);

        assert_eq!(unit.exponent("x"), 1);
        assert_eq!(unit.exponent("y"), 1);
        assert_eq!(unit.exponent("r"), 1);
        assert_eq!(unit.exponent("i"), -1);
    }

    #[test]
    fn formats_units_readably() {
        assert_eq!(Unit::dimensionless().to_string(), "1");
        assert_eq!(Unit::symbol("m").to_string(), "m");
        assert_eq!(
            Unit::symbol("m").divide(&Unit::symbol("s")).to_string(),
            "m/s"
        );
        assert_eq!(Unit::powers("m", 2).to_string(), "m^2");
        assert_eq!(
            Unit::symbol("kg")
                .multiply(&Unit::symbol("m"))
                .divide(&Unit::powers("s", 2))
                .to_string(),
            "kg*m/s^2"
        );
    }
}
