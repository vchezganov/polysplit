use std::fmt;
use std::error;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PolySplitErrorKind {
    InvalidPolyline,
    InvalidPoints,
    PointFarAway,
    CannotSplit,
}

#[derive(Debug)]
pub struct PolySplitError {
    pub(super) kind: PolySplitErrorKind,
    pub message: String,
}

impl PolySplitError {
    pub fn kind(&self) -> &PolySplitErrorKind {
        &self.kind
    }
}

impl fmt::Display for PolySplitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for PolySplitError {}

pub type Result<T> = std::result::Result<T, PolySplitError>;
