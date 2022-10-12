use std::cmp::PartialOrd;
use std::ops::Add;


pub struct DistanceToSegmentResult<P, D>
where
    P: Copy,
    D: Copy + PartialOrd + Add<Output = D>,
{
    pub cut_ratio: f64,
    pub cut_point: P,
    pub distance: D,
}

pub trait DistanceToSegment<D>
where
    Self: Copy,
    D: Copy + PartialOrd + Add<Output = D>,
{
    fn distance_to_segment(&self, segment: (&Self, &Self)) -> DistanceToSegmentResult<Self, D>;
}
