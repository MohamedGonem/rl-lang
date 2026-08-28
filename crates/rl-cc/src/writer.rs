pub struct CWriter {
    source: String,
    indent_level: usize,
}

impl Default for CWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl CWriter {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            indent_level: 0,
        }
    }

    pub fn write(&mut self, s: &str) {
        self.source.push_str(s);
    }

    pub fn writeln(&mut self, s: &str) {
        self.write_indent();
        self.source.push_str(s);
        self.source.push('\n');
    }

    pub fn blank_line(&mut self) {
        self.source.push('\n');
    }

    pub fn indent(&mut self) {
        self.indent_level += 1;
    }

    pub fn dedent(&mut self) {
        self.indent_level -= 1;
    }

    fn write_indent(&mut self) {
        for _ in 0..self.indent_level {
            self.source.push_str("      ");
        }
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn into_source(self) -> String {
        self.source
    }
}
