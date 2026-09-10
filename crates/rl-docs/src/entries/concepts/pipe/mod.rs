use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static PIPE: ConceptEntry = ConceptEntry {
    name: "pipe operator",
    summary: "the `|>` operator chains function calls by passing the left-hand side as the receiver",
    category: ConceptCategory::Syntax,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("basic pipe"),
            description: "`a |> f(args)` desugars to `a.f(args)` - the left-hand side becomes the receiver of the method call",
            examples: &[
                "\"hello\" |> to_upper()",
            ],
            expected_output: &["HELLO"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("pipe with arguments"),
            description: "the original call's arguments are preserved - only the receiver changes",
            examples: &[
                "\"foo bar\" |> replace(\"foo\", \"baz\")",
            ],
            expected_output: &["baz bar"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("chained pipes"),
            description: "pipes can be chained left-to-right for readable function composition",
            examples: &[
                "\"hello world\" |> to_upper() |> reverse()",
            ],
            expected_output: &["DLROW OLLEH"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("pipe with user functions"),
            description: "the pipe works with any function - user-defined or stdlib",
            examples: &[
                "fn double(int x) -> int {\n    return x * 2\n}\n5 |> double()",
            ],
            expected_output: &["10"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("right-hand side must be a call"),
            description: "the right-hand side of `|>` must be a function or method call - a bare identifier is not allowed",
            examples: &[],
            expected_output: &[],
        },
    ],
    pitfalls: &[
        "the right-hand side of `|>` must be a function or method call - a bare identifier like `x |> y` is a parse error",
    ],
    related: &["functions", "variables"],
    related_stdlib: &[],
    since: Some("v2.1.0"),
};
