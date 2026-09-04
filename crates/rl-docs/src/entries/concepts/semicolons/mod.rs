use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static SEMICOLONS: ConceptEntry = ConceptEntry {
    name: "semicolons",
    summary: "statements are separated by newlines; a trailing `;` is optional and can be used as an alternative statement terminator",
    category: ConceptCategory::Syntax,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("newline-terminated statements"),
            description: "the default way to end a statement is with a newline - no special character is needed",
            examples: &[
                "dec int x = 10\ndec int y = 20\nprintln(x + y)  // 30",
            ],
            expected_output: &["30"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("optional semicolons"),
            description: "a semicolon `;` can optionally follow any statement to terminate it - it is silently consumed and has no effect on semantics",
            examples: &[
                "dec int x = 10;\ndec int y = 20;\nprintln(x + y);  // 30",
            ],
            expected_output: &["30"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("mixed usage"),
            description: "semicolons and newlines can be freely mixed - some statements can end with `;` while others rely on the newline",
            examples: &[
                "dec int a = 1;\ndec int b = 2\ndec int c = 3;\nprintln(a + b + c)  // 6",
            ],
            expected_output: &["6"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Note,
            title: Some("semicolons inside blocks"),
            description: "semicolons work inside blocks too - function bodies, loop bodies, and other brace-delimited blocks all accept trailing semicolons on their inner statements",
            examples: &[
                "fn add(int a, int b) -> int {\n    return a + b;\n}\nprintln(add(3, 4))  // 7",
            ],
            expected_output: &["7"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("no double semicolons"),
            description: "writing two semicolons in a row (`dec int x = 1;;`) is a parse error - the first `;` ends the statement, and the second is not a valid start for a new statement",
            examples: &[],
            expected_output: &[],
        },
    ],
    pitfalls: &[
        "double semicolons (`;;`) are a parse error - the first terminates the statement and the second cannot begin a new one",
    ],
    related: &["variables", "constants", "functions"],
    related_stdlib: &[],
    since: Some("v2.1.0"),
};
