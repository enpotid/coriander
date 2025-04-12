use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub file: String,
    pub range: Range<usize>,
}
