use crate::{
    entry::{ConceptEntry, StdEntry},
    website::{
        builder::SiteBuilder,
        platform::{docs_dir, open_browser},
    },
};

mod builder;
mod helpers;
mod platform;

/// Builds the documentation website if it has not already been generated,
/// then opens the generated index page in the user's default browser.
///
/// The website is stored in the platform-specific docs directory. Existing
/// generated documentation is reused instead of being rebuilt.
pub fn build_and_open_website(
    std_entries: &[&StdEntry],
    concept_entries: &[&ConceptEntry],
    tutorial_entries: &[&ConceptEntry],
) {
    let docs_dir = docs_dir();
    let index_path = docs_dir.join("index.html");
    if !index_path.exists() {
        let site_builder =
            SiteBuilder::new(docs_dir, std_entries, concept_entries, tutorial_entries);
        if let Err(e) = site_builder.build() {
            eprintln!("error: building website {}", e);
            return;
        };
    }
    if let Err(e) = open_browser(&index_path) {
        eprintln!("error: opening browser {}", e);
    }
}
