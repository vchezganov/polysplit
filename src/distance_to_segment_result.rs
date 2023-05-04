use std::ops::Add;

use crate::CutRatioResult;

/// DistanceToSegmentResult presents the projection results of the point to the segment.
pub struct DistanceToSegmentResult<P, D>
where
    P: Copy,
    D: Copy + PartialOrd + Add<Output = D>,
{
    pub cut_ratio: CutRatioResult,
    pub cut_point: P,
    pub distance: D,
}
