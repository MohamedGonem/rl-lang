use std::ops::Range;

/// A byte-offset range into the source string.
///
/// Used for pointing error reports at exact source locations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// A sentinel span used when no real location is known.
    pub fn dummy() -> Self {
        Self { start: 0, end: 0 }
    }

    /// Span covering both `self` and `other` (and everything between).
    pub fn join(self, other: Self) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

impl From<Span> for Range<usize> {
    fn from(s: Span) -> Self {
        s.start..s.end
    }
}

impl From<Range<usize>> for Span {
    fn from(r: Range<usize>) -> Self {
        Self {
            start: r.start,
            end: r.end,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Span;
    use std::ops::Range;

    const DEFAULT_START: usize = 0;
    const DEFAULT_END: usize = 10;

    #[test]
    fn span_basic() {
        let span = Span::new(DEFAULT_START, DEFAULT_END);
        assert_eq!(span.start, DEFAULT_START);
        assert_eq!(span.end, DEFAULT_END);
    }

    #[test]
    fn span_dummy() {
        let span = Span::dummy();
        assert_eq!(span.start, DEFAULT_START);
        assert_eq!(span.end, DEFAULT_START);
    }

    #[test]
    fn span_join_non_overlapping() {
        let span = Span::new(DEFAULT_START, DEFAULT_START + 3);
        let span_other = Span::new(DEFAULT_START, DEFAULT_START + 15);
        assert_eq!(
            span.join(span_other),
            Span::new(DEFAULT_START, DEFAULT_START + 15)
        );
    }

    #[test]
    fn span_join_overlapping() {
        let span = Span::new(DEFAULT_START, DEFAULT_START + 5);
        let span_other = Span::new(DEFAULT_START + 10, DEFAULT_START + 15);
        assert_eq!(
            span.join(span_other),
            Span::new(DEFAULT_START, DEFAULT_START + 15)
        );
    }

    #[test]
    fn span_from_range() {
        let range = DEFAULT_START..DEFAULT_END;
        assert_eq!(Span::new(DEFAULT_START, DEFAULT_END), Span::from(range));
    }

    #[test]
    fn range_from_span() {
        let span = Span::new(DEFAULT_START, DEFAULT_END);
        assert_eq!(DEFAULT_START..DEFAULT_END, Range::from(span));
    }

    #[test]
    fn range_from_span_using_into() {
        let span = Span::new(DEFAULT_START, DEFAULT_END);
        let range: Range<usize> = span.into();
        assert_eq!(range, DEFAULT_START..DEFAULT_END);
    }
}
