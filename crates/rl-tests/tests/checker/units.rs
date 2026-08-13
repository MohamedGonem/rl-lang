use {
    super::common::{assert_checker_clean, assert_checker_msg},
    crate::common::check,
    rl_ast::statements::UnitAnnotation,
    rl_checker::{
        structs::CheckType,
        units::Unit,
    },
    rl_ast::statements::TypeAnnotation,
};

// ---- `Unit` algebra ----

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
    let speed = Unit::symbol("m").divide(&Unit::symbol("s"));

    assert_eq!(speed.exponent("m"), 1);
    assert_eq!(speed.exponent("s"), -1);
}

#[test]
fn produces_dimensionless_unit_when_symbols_cancel() {
    let result = Unit::symbol("m").divide(&Unit::symbol("m"));

    assert!(result.is_dimensionless());
}

#[test]
fn equal_units_are_compatible() {
    assert!(Unit::symbol("m").is_compatible_with(&Unit::symbol("m")));
    assert!(!Unit::symbol("m").is_compatible_with(&Unit::symbol("s")));
}

#[test]
fn dimensionless_is_compatible_with_everything() {
    let dimensionless = Unit::dimensionless();

    assert!(dimensionless.is_compatible_with(&Unit::symbol("m")));
    assert!(Unit::symbol("m").is_compatible_with(&dimensionless));
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

// ---- declarations store units in scope ----

#[test]
fn stores_unit_for_variable_declaration() {
    let checker = check("dec float speed: m/s = 12.5");

    assert!(
        checker.errors.is_empty(),
        "unexpected checker errors: {:?}",
        checker.errors
    );

    let item = checker.scopes[0]
        .get("speed")
        .expect("speed should be declared");

    assert_eq!(item.type_annotation, CheckType::Known(TypeAnnotation::Float));
    assert!(!item.is_const);

    let unit = item.unit.as_ref().expect("speed should have a unit");
    assert_eq!(unit.exponent("m"), 1);
    assert_eq!(unit.exponent("s"), -1);
}

#[test]
fn stores_unit_for_constant_declaration() {
    let checker = check("CONST float SPEED: m/s = 12.5");

    assert!(
        checker.errors.is_empty(),
        "unexpected checker errors: {:?}",
        checker.errors
    );

    let item = checker.scopes[0]
        .get("SPEED")
        .expect("SPEED should be declared");

    assert_eq!(item.type_annotation, CheckType::Known(TypeAnnotation::CFloat));
    assert!(item.is_const);

    let unit = item.unit.as_ref().expect("SPEED should have a unit");
    assert_eq!(unit.exponent("m"), 1);
    assert_eq!(unit.exponent("s"), -1);
}

#[test]
fn stores_normalized_compound_unit() {
    let checker = check("dec float value: x*y*r/i = 1.0");

    assert!(
        checker.errors.is_empty(),
        "unexpected checker errors: {:?}",
        checker.errors
    );

    let unit = checker.scopes[0]["value"]
        .unit
        .as_ref()
        .expect("value should have a unit");

    assert_eq!(unit.exponent("x"), 1);
    assert_eq!(unit.exponent("y"), 1);
    assert_eq!(unit.exponent("r"), 1);
    assert_eq!(unit.exponent("i"), -1);
}

#[test]
fn declaration_without_unit_has_none() {
    let checker = check("dec float value = 12.5");

    assert!(checker.scopes[0]["value"].unit.is_none());
}

#[test]
fn units_supported_on_all_numeric_types() {
    assert_checker_clean("dec int n: count = 5");
    assert_checker_clean("dec uint n: count = 5 as uint");
    assert_checker_clean("dec small int n: count = 5 as small int");
    assert_checker_clean("dec small uint n: count = 5 as small uint");
    assert_checker_clean("dec float f: ratio = 1.5");
    assert_checker_clean("dec small float f: ratio = 1.5 as small float");
    assert_checker_clean("dec byte b: bits = 8 as byte");
    assert_checker_clean("dec big byte b: bits = 8 as big byte");
    assert_checker_clean("CONST float SPEED: m/s = 12.5");
    assert_checker_clean("CONST int N: count = 5");
}

#[test]
fn units_rejected_on_non_numeric_types() {
    for source in ["dec string s: m = \"hi\"", "CONST bool b: m = true"] {
        let file = rl_utils::source::SourceFile::new("test", source.to_string());
        let tokens = rl_lexer::tokenizer::Tokenizer::lex(file.clone()).unwrap();
        let result = rl_parser::parser_logic::Parser::parse(tokens, file);
        assert!(
            result.is_err(),
            "expected a parse error for {source:?}"
        );
    }
}

// ---- issue #347 examples ----

#[test]
fn inferred_unit_from_multiplication() {
    assert_checker_clean(
        r#"
dec float speed: m/s = 12.5
dec float time: s = 4.0
dec float distance: m = speed * time
"#,
    );

    let checker = check(
        r#"
dec float speed: m/s = 12.5
dec float time: s = 4.0
dec float distance: m = speed * time
"#,
    );
    let unit = checker.scopes[0]["distance"].unit.as_ref().unwrap();
    assert_eq!(unit.exponent("m"), 1);
}

#[test]
fn mismatched_addition_is_rejected() {
    assert_checker_msg(
        r#"
dec float speed: m/s = 12.5
dec float time: s = 4.0
dec float bad = speed + time
"#,
        "unit mismatch on +: got m/s and s",
    );
}

#[test]
fn mismatched_subtraction_is_rejected() {
    assert_checker_msg(
        r#"
dec float speed: m/s = 12.5
dec float distance: m = 100.0
dec float bad = distance - speed
"#,
        "unit mismatch on -",
    );
}

#[test]
fn division_divides_units() {
    assert_checker_clean(
        r#"
dec float distance: m = 100.0
dec float time: s = 4.0
dec float speed: m/s = distance / time
"#,
    );
}

#[test]
fn division_can_produce_dimensionless() {
    assert_checker_clean(
        r#"
dec float distance: m = 100.0
dec float time: s = 4.0
dec float speed: m/s = distance / time
dec float ratio = (speed * time) / distance
"#,
    );
}

#[test]
fn literal_adopts_declared_unit() {
    assert_checker_clean("dec float distance: m = 100.0");
    assert_checker_clean("dec int seconds: s = 4");
}

#[test]
fn compatible_units_compare() {
    assert_checker_clean(
        r#"
dec float a: m/s = 10.0
dec float b: m/s = 20.0
dec bool faster = a > b
dec bool equal = a == b
"#,
    );
}

#[test]
fn incompatible_units_cannot_compare() {
    assert_checker_msg(
        r#"
dec float speed: m/s = 12.5
dec float distance: m = 100.0
dec bool bad = speed > distance
"#,
        "unit mismatch on >",
    );
}

// ---- declaration unit mismatches ----

#[test]
fn declared_unit_must_match_initializer() {
    assert_checker_msg(
        r#"
dec float speed: m/s = 12.5
dec float time: s = 4.0
dec float distance: s = speed * time
"#,
        "unit mismatch on declaration: expected s, got m",
    );
}

#[test]
fn dimensionless_declaration_rejects_unit_value() {
    assert_checker_msg(
        r#"
dec float speed: m/s = 12.5
dec float bad = speed
"#,
        "unit mismatch on declaration: expected dimensionless, got m/s",
    );
}

// ---- assignment unit mismatches ----

#[test]
fn assignment_requires_compatible_units() {
    assert_checker_msg(
        r#"
dec float speed: m/s = 12.5
dec float distance: m = 100.0
distance = speed
"#,
        "unit mismatch: expected m, got m/s",
    );
}

#[test]
fn assignment_to_dimensionless_variable_rejects_unit_value() {
    assert_checker_msg(
        r#"
dec float speed: m/s = 12.5
dec float x = 1.0
x = speed
"#,
        "cannot assign m/s to variable 'x' declared without a unit",
    );
}

#[test]
fn assignment_of_dimensionless_value_is_allowed() {
    assert_checker_clean(
        r#"
dec float distance: m = 100.0
distance = 250.0
"#,
    );
}

#[test]
fn assignment_between_compatible_units_is_allowed() {
    assert_checker_clean(
        r#"
dec float speed: m/s = 12.5
dec float velocity: m/s = 0.0
velocity = speed
"#,
    );
}

#[test]
fn units_propagate_through_unary_minus() {
    assert_checker_clean(
        r#"
dec float distance: m = 100.0
dec float opposite: m = -distance
"#,
    );
}

#[test]
fn incompatible_units_break_through_grouping() {
    assert_checker_msg(
        r#"
dec float speed: m/s = 12.5
dec float time: s = 4.0
dec float bad = (speed) + (time)
"#,
        "unit mismatch on +",
    );
}

#[test]
fn convertible_units_are_accepted_on_assignment() {
    assert_checker_clean(
        r#"
#![convert(kg=1000(g))]
dec float weight_kg: kg = 2.5
dec float weight_g: g = weight_kg
dec float heavy: g = 0.0
heavy = weight_kg
"#,
    );
}

#[test]
fn convertible_units_can_be_added_and_subtracted() {
    assert_checker_clean(
        r#"
#![convert(kg=1000(g))]
dec float weight_kg: kg = 2.5
dec float weight_g: g = 300.0
dec float total_kg: kg = weight_kg + weight_g
dec float diff_g: g = weight_g - weight_kg
"#,
    );
}

#[test]
fn convertible_units_can_be_compared() {
    assert_checker_clean(
        r#"
#![convert(kg=1000(g))]
dec float weight_kg: kg = 2.5
dec float weight_g: g = 300.0
dec bool heavier = weight_kg > weight_g
"#,
    );
}

#[test]
fn unrelated_units_remain_incompatible() {
    assert_checker_msg(
        r#"
#![convert(kg=1000(g))]
dec float weight: kg = 2.5
dec float time: s = 4.0
dec float bad: kg = time
"#,
        "unit mismatch",
    );
}

#[test]
fn conversion_keeps_dimensions_separate() {
    assert_checker_msg(
        r#"
#![convert(km=1000(m))]
dec float speed: m/s = 12.5
dec float time: s = 4.0
dec float bad: km = speed
"#,
        "unit mismatch",
    );
}
