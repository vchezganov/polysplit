use std::cmp::{Ord, Ordering, PartialEq, Eq, PartialOrd};

/// CutRatioResult presents the closest projection of the point to the segment.
pub enum CutRatioResult {
    /// The closest projection is the start of the segment.
    Begin,
    /// The closest projection is the point on the segment that splits it in
    /// the defined proportion (usually `0.0 < ratio < 1.0` where `0.0` is the start
    /// and `1.0` is the end of the segment).
    Medium(f64),
    /// The closest projection is the end of the segment.
    End,
}

impl PartialEq for CutRatioResult {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Begin, Self::Begin) => true,
            (Self::Medium(s), Self::Medium(o)) => s.eq(o),
            (Self::End, Self::End) => true,
            _ => false,
        }
    }
}

impl Eq for CutRatioResult {}

impl PartialOrd for CutRatioResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CutRatioResult {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Begin, Self::Begin) => Ordering::Equal,
            (Self::Begin, _) => Ordering::Less,
            (Self::Medium(_), Self::Begin) => Ordering::Greater,
            (Self::Medium(s), Self::Medium(o)) => s.total_cmp(o),
            (Self::Medium(_), Self::End) => Ordering::Less,
            (Self::End, Self::End) => Ordering::Equal,
            (Self::End, _) => Ordering::Greater,
        }
    }
}
