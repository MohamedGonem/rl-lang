//! "units" language concept - compile-time units of measure.

use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static UNITS: ConceptEntry = ConceptEntry {
    name: "units",
    summary: "attach a compile-time unit of measure like `m/s` or `kg` to numeric declarations, and declare convertible units with `#![convert(kg=1000(g))]`",
    category: ConceptCategory::Types,
    prerequisites: &["types", "variables", "constants"],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("attaching a unit"),
            description: "any mutable or constant numeric declaration can carry a unit annotation after its name: `dec float speed: m/s = 12.5`. units compose with `*` and `/`, parsed left to right, so `kg*m/s` is one compound unit",
            examples: &[
                "dec float speed: m/s = 12.5",
                "dec float force: kg*m/s = 20.0",
                "const float GRAVITY: m/s^2 = 9.8",
                "dec float distance: m = speed * 4.0",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("dimensional algebra"),
            description: "the checker tracks units through expressions: `*` and `/` combine them (`m/s * s` is `m`), while `+`, `-`, and comparisons require both sides to share the same unit, or for one side to be a plain number which adopts the other side's unit",
            examples: &[
                "dec float t: s = 4.0",
                "dec float d: m = speed * t // m/s * s = m",
                "dec float v: m/s = speed + 1.0 // literal adopts m/s",
                "dec float bad = speed + t // error: unit mismatch",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("declaring convertible units"),
            description: "a program-level `#![convert(symbol=factor(base))]` attribute declares that `1 symbol` equals `factor` of `base` - e.g. `#![convert(kg=1000(g))]` makes `kg` and `g` interchangeable, so values carrying either unit can be assigned, added, or compared",
            examples: &[
                "#![convert(kg=1000(g))]",
                "dec float weight_g: g = 2500.0",
                "dec float weight_kg: kg = weight_g // convertible, ok",
                "dec float total: kg = weight_kg + weight_g",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("compile-time only"),
            description: "units exist only between parsing and type checking - they never reach the VM or interpreter, and a conversion factor does not scale a value at runtime (it only makes the units compatible). `0`-factor and malformed attributes are rejected at parse time",
            examples: &[
                "#![convert(cm=0.01(m))]",
                "dec float length_cm: cm = 100.0",
                "dec float length_m: m = length_cm // cm is compatible with m",
                "println(length_m) // prints 100.0, values are not scaled",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[
        "units are only allowed on numeric types - `dec string x: m = \"hi\"` is a parse error",
        "mixing unrelated units (e.g. `m` and `s`) in `+`, `-`, comparisons, or assignment is a compile-time error unless a `#![convert(...)]` joins them",
        "conversion factors are compile-time compatibility only: `convert` does not scale runtime values yet",
        "a declared unit expects a matching unit on the right-hand side; a plain literal or dimensionless expression adopts the declared unit instead",
    ],
    related: &["types", "constants", "variables", "operators"],
    related_stdlib: &[],
    since: Some("v0.1.5"),
};