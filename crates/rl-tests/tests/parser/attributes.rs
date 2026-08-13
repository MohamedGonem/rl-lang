use rl_ast::statements::ProgramAttribute;

use crate::common::parse;

#[test]
fn program_convert_attribute() {
    let (ast, statements) = parse("#![convert(kg=1000(g))]");
    assert!(statements.is_empty());

    assert_eq!(
        ast.program_attributes,
        vec![ProgramAttribute::Convert {
            symbol: "kg".to_string(),
            factor: 1000.0,
            base_symbol: "g".to_string(),
        }],
    );
}

#[test]
fn multiple_program_attributes() {
    let (ast, statements) = parse("#![convert(kg=1000(g))]\n#![convert(km=1000(m))]");
    assert!(statements.is_empty());

    assert_eq!(
        ast.program_attributes,
        vec![
            ProgramAttribute::Convert {
                symbol: "kg".to_string(),
                factor: 1000.0,
                base_symbol: "g".to_string(),
            },
            ProgramAttribute::Convert {
                symbol: "km".to_string(),
                factor: 1000.0,
                base_symbol: "m".to_string(),
            },
        ],
    );
}

#[test]
fn convert_attribute_can_use_float_factor() {
    let (ast, _) = parse("#![convert(cm=0.01(m))]");

    assert_eq!(
        ast.program_attributes,
        vec![ProgramAttribute::Convert {
            symbol: "cm".to_string(),
            factor: 0.01,
            base_symbol: "m".to_string(),
        }],
    );
}

#[test]
fn convert_attribute_before_declarations() {
    let (ast, statements) = parse("#![convert(kg=1000(g))]\ndec float weight: kg = 2.5");
    assert_eq!(statements.len(), 1);

    assert_eq!(
        ast.program_attributes,
        vec![ProgramAttribute::Convert {
            symbol: "kg".to_string(),
            factor: 1000.0,
            base_symbol: "g".to_string(),
        }],
    );
}