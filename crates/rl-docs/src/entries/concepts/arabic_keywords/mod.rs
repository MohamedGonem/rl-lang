use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static ARABIC_KEYWORDS: ConceptEntry = ConceptEntry {
    name: "arabic keywords",
    summary: "every language keyword has an Arabic equivalent - use either form interchangeably / لكل كلمة مفتاحية في اللغة ما يعادلها بالعربية - استخدم أي شكل بشكل مترادف",
    category: ConceptCategory::Syntax,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("Arabic keyword aliases / مرادفات الكلمات المفتاحية بالعربية"),
            description: "all 38 language keywords have Arabic equivalents. the lexer accepts either form, and you can mix Arabic and English keywords in the same program / جميع الكلمات المفتاحية لها ما يعادلها بالعربية. المحلل يقبل أي شكل، ويمكنك خلط الكلمات العربية والإنجليزية في نفس البرنامج",
            examples: &[
                "// English keywords\nfn add(int a, int b) -> int {\n    return a + b\n}",
                "// Arabic keywords / كلمات مفتاحية عربية\nدالة جمع(int a, int b) -> int {\n    return a + b\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("Arabic control flow / التحكم بالتدفق بالعربية"),
            description: "loop and condition keywords work identically in Arabic / كلمات الحلقات والشروط تعمل بنفس الطريقة بالعربية",
            examples: &[
                "// English\ndec int x = 0\nwhile x < 5 {\n    x += 1\n}",
                "// Arabic / عربية\nثابت x = 0\nبينما x < 5 {\n    x += 1\n}",
                "// English\nfor i in 0..3 {\n    println(i)\n}",
                "// Arabic / عربية\nلكل i in 0..3 {\n    println(i)\n}",
                "// English\nif true {\n    println(\"yes\")\n} else {\n    println(\"no\")\n}",
                "// Arabic / عربية\nإذا صحيح {\n    println(\"yes\")\n} وإلا {\n    println(\"no\")\n}",
            ],
            expected_output: &["0", "1", "2"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("Arabic type and declaration keywords / أنواع الكلمات والتعريفات بالعربية"),
            description: "variable and type keywords have Arabic equivalents / كلمات المتغيرات والأنواع لها ما يعادلها بالعربية",
            examples: &[
                "// English\nconst float PI = 3.14\ndec int x = 10\nprintln(PI + x)",
                "// Arabic / عربية\nثابت PI = 3.14\nأعلن int x = 10\nprintln(PI + x)",
            ],
            expected_output: &["13.14"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: Some("mixed Arabic and English / خلط العربية والإنجليزية"),
            description: "you can freely mix Arabic and English keywords and identifiers in the same file / يمكنك خلط الكلمات المفتاحية والمعرفات العربية والإنجليزية بحرية في نفس الملف",
            examples: &[
                "fn خلص() -> نص {\n    return \"done\"\n}\nprintln(خلص())",
            ],
            expected_output: &["done"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Note,
            title: Some("identifiers are not keywords / المعرفات ليست كلمات مفتاحية"),
            description: "Arabic text inside strings is treated as literal text, not as keywords / النص العربي داخل السلاسل يُعامل كنص حرفي، وليس كلمات مفتاحية",
            examples: &[
                "println(\"إذا صحيح\")  // prints the string, not parsed as keywords",
            ],
            expected_output: &["إذا صحيح"],
        },
    ],
    pitfalls: &[
        "Arabic keywords are token-level aliases - they produce the same token type as their English counterparts, so there is no semantic difference / مرادفات الكلمات المفتاحية العربية على مستوى الرمز - تنتج نفس نوع الرمز counterpart لها، لذلك لا يوجد فرق دلالي",
    ],
    related: &["variables", "constants", "functions", "flow_control"],
    related_stdlib: &[],
    since: Some("v2.1.0"),
};
