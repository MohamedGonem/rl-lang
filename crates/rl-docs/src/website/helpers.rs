use std::collections::HashMap;

use crate::entry::{DescriptionEntry, FnEntry};

pub fn page(title: &str, sidebar_html: &str, body: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title}</title>
<link rel="stylesheet" href="style.css">
<script src="rl-highlight.js" defer></script>
</head>

<body>
<header class="topbar">
<button class="menu-toggle" type="button" aria-label="Toggle navigation" aria-expanded="false">
<span></span><span></span><span></span>
</button>
<a class="topbar-brand" href="index.html">rl docs</a>
</header>

<div class="layout">
<div class="sidebar-backdrop"></div>

<nav class="sidebar">
{sidebar_html}
</nav>

<main>
{body}
</main>

</div>

<script>
(function(){{
    var btn = document.querySelector(".menu-toggle");
    var sidebar = document.querySelector(".sidebar");
    var backdrop = document.querySelector(".sidebar-backdrop");

    function close(){{
        sidebar.classList.remove("open");
        backdrop.classList.remove("open");
        btn.setAttribute("aria-expanded", "false");
    }}

    function toggle(){{
        var open = sidebar.classList.toggle("open");
        backdrop.classList.toggle("open", open);
        btn.setAttribute("aria-expanded", open ? "true" : "false");
    }}

    btn.addEventListener("click", toggle);
    backdrop.addEventListener("click", close);

    sidebar.addEventListener("click", function(e){{
        if (e.target.tagName === "A") close();
    }});
}})();
</script>

</body>
</html>
"#,
        title = title,
        sidebar_html = sidebar_html,
        body = body,
    )
}

/// Escapes the five characters that are meaningful in HTML text content.
pub fn html_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            other => out.push(other),
        }
    }
    out
}

///Turn a name into a safe filename/id (letters, digits, hyphens).
pub fn slugify(text: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = false;

    for ch in text.to_lowercase().chars() {
        if ch.is_alphanumeric() {
            slug.push(ch);
            last_was_dash = false
        } else if !last_was_dash {
            slug.push('-');
            last_was_dash = true
        }
    }

    let slug = slug.trim_matches('-');

    if slug.is_empty() {
        "entry".into()
    } else {
        slug.to_string()
    }
}

pub fn render_fn_entry(func: &FnEntry, fn_link_map: &HashMap<String, String>) -> String {
    let mut out = format!("<h3><code>{}</code></h3>\n", html_escape(func.signature));

    if let Some(since) = func.since {
        out.push_str(&format!("<p><em>since {}</em></p>\n", html_escape(since)));
    }

    out.push_str(&format!(
        "<p>{}</p>\n<p><strong>Returns:</strong> {}</p>\n",
        html_escape(func.description),
        html_escape(func.returns)
    ));

    if let Some(errors) = func.errors {
        out.push_str(&format!(
            "<p><strong>Errors:</strong> {}</p>\n",
            html_escape(errors)
        ));
    }

    out.push_str(&render_example_block(func.example, func.expected_output));

    out.push_str(&render_related(
        "See also",
        func.see_also.to_vec(),
        fn_link_map,
        "",
    ));

    out
}

/// Render a labeled list of related names.
/// link_map is {name: filename} so related items become links
/// to their own page instead of plain text.
pub fn render_related(
    label: &str,
    names: Vec<&str>,
    link_map: &HashMap<String, String>,
    prefix: &str,
) -> String {
    if names.is_empty() {
        return "".into();
    }

    let mut items = Vec::new();

    for n in names {
        let display = html_escape(&format!("{prefix}{n}"));
        if let Some(link) = link_map.get(n) {
            items.push(format!(
                "<a href=\"{}\"><code>{display}</code></a>",
                html_escape(link)
            ));
        } else {
            items.push(format!("<code>{display}</code>"));
        }
    }

    format!(
        "<p><strong>{}:</strong> {}</p>\n",
        html_escape(label),
        items.join(", ")
    )
}
pub fn render_description_entry(desc: &DescriptionEntry) -> String {
    let mut out = String::new();

    if let Some(title) = desc.title {
        out.push_str(&format!("<h4>{}</h4>\n", html_escape(title)));
    }

    out.push_str(&format!(
        "<p><strong>{}:</strong> {}</p>\n",
        desc.kind, desc.description
    ));

    for (i, example) in desc.examples.iter().enumerate() {
        let expected = desc.expected_output.get(i).copied();
        out.push_str(&render_example_block(example, expected));
    }

    out
}

fn render_example_block(example: &str, expected_output: Option<&str>) -> String {
    let mut out = format!("<pre class=\"rl-code\">{}</pre>\n", html_escape(example),);

    if let Some(expected) = expected_output {
        out.push_str(&format!(
            "<p><em>output:</em></p>\n<pre>{}</pre>\n",
            html_escape(expected)
        ));
    }

    out
}
