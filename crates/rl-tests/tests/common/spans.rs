use rl_utils::span::Span;

/// Returns the byte span of the first occurrence of `needle` in `source`.
///
/// Useful in tests so spans don't have to be hand-computed as raw byte
/// offsets: they are derived from the source instead, and stay correct when
/// the source text changes. Panics with a descriptive message if `needle` is
/// absent.
pub fn span_of(source: &str, needle: &str) -> Span {
    let start = source
        .find(needle)
        .unwrap_or_else(|| panic!("needle {:?} not found in source {:?}", needle, source));
    Span::new(start, start + needle.len())
}

/// Returns the byte span of the last occurrence of `needle` in `source`.
///
/// Useful when the same substring appears more than once (e.g. the same body
/// literal in both branches of an `if`/`else`). Panics with a descriptive
/// message if `needle` is absent.
pub fn span_of_last(source: &str, needle: &str) -> Span {
    let start = source
        .rfind(needle)
        .unwrap_or_else(|| panic!("needle {:?} not found in source {:?}", needle, source));
    Span::new(start, start + needle.len())
}

/// Returns the byte span of the `occurrence`-th (0-indexed) occurrence of
/// `needle` in `source`.
///
/// Useful when the same identifier appears many times and a specific one is
/// meant (e.g. `i` in a `for` loop header). Panics with a descriptive message
/// if there are fewer than `occurrence + 1` matches.
pub fn span_of_nth(source: &str, needle: &str, occurrence: usize) -> Span {
    let mut search_from = 0;
    let mut remaining = occurrence;
    let start = loop {
        let found = source[search_from..]
            .find(needle)
            .map(|rel| search_from + rel)
            .unwrap_or_else(|| {
                panic!(
                    "occurrence {} of needle {:?} not found in source {:?}",
                    occurrence, needle, source
                )
            });
        if remaining == 0 {
            break found;
        }
        remaining -= 1;
        search_from = found + needle.len();
    };
    Span::new(start, start + needle.len())
}

/// Returns a span covering the whole of `source`.
pub fn span_whole(source: &str) -> Span {
    Span::new(0, source.len())
}

/// Returns a span from the start of `source` to the end of the first
/// occurrence of `needle`.
///
/// Useful for statement spans that cover everything up to (but not past) the
/// value expression, e.g. `dec byte x = 65 as byte` records the statement as
/// spanning `dec byte x = 65`. Panics if `needle` is absent.
pub fn span_up_to(source: &str, needle: &str) -> Span {
    let span = span_of(source, needle);
    Span::new(0, span.end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_of_finds_first_occurrence() {
        let source = "dec int x = 1000";
        assert_eq!(span_of(source, "1000"), Span::new(12, 16));
        assert_eq!(span_of(source, "x"), Span::new(8, 9));
    }

    #[test]
    fn span_of_is_byte_offset() {
        let source = "dec string x = \"héllo\"";
        // 'é' is two bytes, so the quoted string spans more bytes than chars.
        assert_eq!(span_of(source, "\"héllo\""), Span::new(15, 23));
        assert_eq!(span_of(source, "é"), Span::new(17, 19));
    }

    #[test]
    fn span_of_last_finds_last_occurrence() {
        let source = "if (true) {1} else {0}";
        assert_eq!(span_of_last(source, "{0}"), Span::new(19, 22));
        assert_eq!(span_of_last(source, "else"), Span::new(14, 18));
    }

    #[test]
    fn span_of_and_last_differ_on_repeated_needles() {
        let source = "match x { 1 => {0} _ => {1} }";
        assert_eq!(span_of(source, "1"), Span::new(10, 11));
        assert_eq!(span_of_last(source, "1"), Span::new(25, 26));
    }

    #[test]
    fn span_of_nth_finds_specific_occurrence() {
        let source = "for [int i = 1, i < 10, i += 1] {0}";
        assert_eq!(span_of_nth(source, "i", 0), Span::new(5, 6));
        assert_eq!(span_of_nth(source, "i", 1), Span::new(9, 10));
        assert_eq!(span_of_nth(source, "i", 2), Span::new(16, 17));
        assert_eq!(span_of_nth(source, "i", 3), Span::new(24, 25));
    }

    #[test]
    #[should_panic(expected = "occurrence 4 of needle \"i\" not found")]
    fn span_of_nth_panics_when_out_of_range() {
        let _ = span_of_nth("for [int i = 1, i < 10, i += 1] {0}", "i", 4);
    }

    #[test]
    fn span_whole_covers_source() {
        let source = "import math";
        assert_eq!(span_whole(source), Span::new(0, 11));
    }

    #[test]
    fn span_up_to_covers_source_start_to_needle_end() {
        let source = "dec byte x = 65 as byte";
        assert_eq!(span_up_to(source, "65"), Span::new(0, 15));
        assert_eq!(span_up_to(source, "x"), Span::new(0, 10));
    }

    #[test]
    #[should_panic(expected = "needle \"??\" not found")]
    fn span_of_panics_when_needle_absent() {
        let _ = span_of("dec int x = 1000", "??");
    }
}
