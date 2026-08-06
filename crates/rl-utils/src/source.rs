use std::sync::Arc;

/// A named source file (or `<repl>` snippet) carried through each pipeline
/// stage so error reports can quote the original source text.
#[derive(Clone)]
pub struct SourceFile {
    /// The file name shown in error report headers (e.g. `"main.rl"`, `"<repl>"`).
    pub name: Arc<str>,
    /// The full source text, reference-counted to avoid cloning across pipeline stages.
    pub text: Arc<String>,
}

impl SourceFile {
    /// Creates a new [`SourceFile`] from a name and source text.
    pub fn new(name: impl Into<Arc<str>>, text: impl Into<Arc<String>>) -> Self {
        Self {
            name: name.into(),
            text: text.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SourceFile;

    #[test]
    fn source_file_basic() {
        let name = "main.rl";
        let text = "println(\"foobar\")";
        
        let source_file = SourceFile::new(name, text.to_string());
        
        assert_eq!(&*source_file.name, name);
        assert_eq!(&*source_file.text, text);
    }
}