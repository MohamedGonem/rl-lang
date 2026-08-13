use std::{collections::HashMap, fs, path::PathBuf};

use crate::{
    entry::{ConceptEntry, StdEntry},
    website::helpers::{
        html_escape, page, render_description_entry, render_fn_entry, render_related, slugify,
    },
};

const STYLE_CSS: &str = include_str!("../../assets/style.css");
const RL_HIGHLIGHT_JS: &str = include_str!("../../assets/rl-highlight.js");

pub struct SiteBuilder<'a> {
    out_dir: PathBuf,
    stdlib: &'a [&'a StdEntry],
    std_files: HashMap<String, String>,
    concepts: &'a [&'a ConceptEntry],
    concept_files: HashMap<String, String>,
    tutorials: &'a [&'a ConceptEntry],
    tutorial_files: HashMap<String, String>,
    fn_files: HashMap<String, String>,
}

impl<'a> SiteBuilder<'a> {
    pub fn new(
        out_dir: PathBuf,
        stdlib: &'a [&'a StdEntry],
        concepts: &'a [&'a ConceptEntry],
        tutorials: &'a [&'a ConceptEntry],
    ) -> Self {
        // filename maps, so cross-references (see_also / related / related_stdlib)
        // can become real links between pages
        let mut concept_files = HashMap::new();
        for e in concepts {
            let filename = format!("concept_{}.html", slugify(e.name));
            concept_files.insert(e.name.into(), filename);
        }

        let mut tutorial_files = HashMap::new();
        for e in tutorials {
            let filename = format!("tutorial_{}.html", slugify(e.name));
            tutorial_files.insert(e.name.into(), filename);
        }

        let mut std_files = HashMap::new();

        // flat map of every function's bare name -> its module page, for "see also" links
        let mut fn_files = HashMap::new();

        for e in stdlib {
            let filename = format!("std_{}.html", slugify(e.name));
            std_files.insert(e.name.into(), filename.clone());

            for func in e.functions {
                let bare = func.signature.split('(').next().unwrap_or("");

                fn_files.insert(bare.to_string(), filename.clone());
            }
        }

        Self {
            out_dir,
            stdlib,
            std_files,
            concepts,
            concept_files,
            tutorials,
            tutorial_files,
            fn_files,
        }
    }

    pub fn build(self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.out_dir)?;

        self.create_asset_file("style.css", STYLE_CSS)?;
        self.create_asset_file("rl-highlight.js", RL_HIGHLIGHT_JS)?;

        self.build_index()?;

        for entry in self.stdlib {
            self.build_std_page(entry)?;
        }

        for entry in self.concepts {
            self.build_concept_or_tutorial_page(entry, &self.concept_files)?;
        }

        for entry in self.tutorials {
            self.build_concept_or_tutorial_page(entry, &self.tutorial_files)?;
        }

        Ok(())
    }

    /// Writes a static asset (css/js) into the output directory.
    ///
    /// The asset contents are provided at compile time and written
    /// to the generated website directory.
    fn create_asset_file(&self, filename: &str, contents: &str) -> std::io::Result<()> {
        let path = self.out_dir.join(filename);
        std::fs::write(path, contents)?;
        Ok(())
    }

    fn write(&self, filename: &str, title: &str, body: &str) -> std::io::Result<()> {
        let contents = page(title, &self.sidebar_html(Some(filename)), body);
        std::fs::write(self.out_dir.join(filename), contents)?;
        Ok(())
    }

    fn build_index(&self) -> std::io::Result<()> {
        let mut body = String::from("<h1>rl docs</h1>\n");

        if !self.stdlib.is_empty() || !self.concepts.is_empty() || !self.tutorials.is_empty() {
            body.push_str("<p>Pick an item from the sidebar to get started.</p>\n");
        } else {
            body.push_str("<p>No documentation entries found.</p>\n");
        }

        fs::write(
            self.out_dir.join("index.html"),
            page("rl docs", &self.sidebar_html(Some("index.html")), &body),
        )?;

        Ok(())
    }

    /// Builds the full sidebar shown on every page: a Home link plus
    /// one section per category (stdlib/concepts/tutorial), each listing
    /// every entry. The current page's link gets the 'active' class.
    fn sidebar_html(&self, active_filename: Option<&str>) -> String {
        let mut out = String::new();

        let link = |filename: &str, label: &str| {
            let class = if Some(filename) == active_filename {
                " class=\"active\""
            } else {
                ""
            };

            format!(
                "<li><a href=\"{}\"{}>{}</a></li>\n",
                html_escape(filename),
                class,
                html_escape(label)
            )
        };

        out.push_str("<a class=\"home-link\" href=\"index.html\">rl docs</a>\n");

        if !self.stdlib.is_empty() {
            out.push_str("<h2>stdlib</h2>\n<ul>\n");

            for entry in self.stdlib {
                let name = entry.name;

                out.push_str(&link(&self.std_files[name], &format!("std::{}", name)));
            }

            out.push_str("</ul>\n");
        }

        if !self.concepts.is_empty() {
            out.push_str("<h2>concepts</h2>\n<ul>\n");

            for entry in self.concepts {
                let name = entry.name;

                out.push_str(&link(&self.concept_files[name], name));
            }

            out.push_str("</ul>\n");
        }

        if !self.tutorials.is_empty() {
            out.push_str("<h2>tutorial</h2>\n<ul>\n");

            for entry in self.tutorials {
                let name = entry.name;

                out.push_str(&link(&self.tutorial_files[name], name));
            }

            out.push_str("</ul>\n");
        }

        out
    }

    fn build_std_page(&self, entry: &StdEntry) -> std::io::Result<()> {
        let name = entry.name;
        let mut body = String::new();

        body.push_str(&format!("<h1>std::{}</h1>\n", html_escape(name)));

        let mut meta_bits = Vec::new();

        if let Some(since) = entry.since {
            meta_bits.push(format!("since {}", html_escape(since)));
        };

        if entry.unstable {
            meta_bits.push("unstable".to_string());
        };

        if !meta_bits.is_empty() {
            body.push_str(&format!("<p><em>{}</em></p>\n", meta_bits.join(" | ")));
        };

        body.push_str(&format!("<p>{}</p>\n", html_escape(entry.description)));

        for func in entry.functions {
            body.push_str(&render_fn_entry(func, &self.fn_files));
        }

        self.write(&self.std_files[name], &format!("std::{}", name), &body)
    }

    fn build_concept_or_tutorial_page(
        &self,
        entry: &ConceptEntry,
        file_map: &HashMap<String, String>,
    ) -> std::io::Result<()> {
        let name = entry.name;
        let mut body = format!("<h1>{}</h1>\n", html_escape(name));

        let mut meta_bits = vec![entry.category.to_string()];

        if let Some(since) = entry.since {
            meta_bits.push(format!("since {}", html_escape(since)));
            body.push_str(&format!("<p><em>{}</em></p>\n", meta_bits.join(" | ")));
        }

        body.push_str(&format!("<p>{}</p>\n", entry.summary));

        body.push_str(&render_related(
            "Prerequisites",
            entry.prerequisites.to_vec(),
            &self.concept_files,
            "",
        ));

        for desc in entry.descriptions {
            body.push_str(&render_description_entry(desc));
        }

        if !entry.pitfalls.is_empty() {
            body.push_str("<p><strong>Pitfalls:</strong></p>\n<ul>\n");
            for p in entry.pitfalls {
                body.push_str(&format!("<li>{}</li>\n", p));
            }
            body.push_str("</ul>\n");
        }

        body.push_str(&render_related(
            "Related concepts",
            entry.related.to_vec(),
            &self.concept_files,
            "",
        ));

        body.push_str(&render_related(
            "Related stdlib",
            entry.related_stdlib.to_vec(),
            &self.std_files,
            "std::",
        ));

        self.write(&file_map[name], name, &body)
    }
}
