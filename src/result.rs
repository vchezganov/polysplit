use std::fmt;
use std::error;
use std::fmt::Debug;


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
    pub point_index: Option<usize>,
}

impl PolySplitError {
    pub fn kind(&self) -> &PolySplitErrorKind {
        &self.kind
    }
}

impl fmt::Display for PolySplitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            PolySplitErrorKind::InvalidPolyline => {
                write!(f, "polyline has not enough points")
            },
            PolySplitErrorKind::InvalidPoints => {
                write!(f, "number of points are not enough")
            },
            PolySplitErrorKind::PointFarAway => {
                write!(f, "point {} has no closest segments", self.point_index.unwrap())
            },
            PolySplitErrorKind::CannotSplit => {
                write!(f, "cannot split polyline")
            },
        }
    }
}

impl error::Error for PolySplitError {}

pub type Result<T> = std::result::Result<T, PolySplitError>;