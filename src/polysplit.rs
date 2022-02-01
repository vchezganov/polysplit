use std::fmt;
use std::error;
use itertools::Itertools;
use std::cmp::{Ord, Ordering, PartialEq, Eq, PartialOrd};
use std::collections::BinaryHeap;
use std::fmt::Debug;
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

type Result<T> = std::result::Result<T, PolySplitError>;

struct Vertex<P, D> {
    point_index: usize,
    segment_index: usize,
    cut_ratio: f64,
    cut_point: P,
    distance_to: D,
}

#[derive(Copy, Clone, PartialEq)]
struct State<D> {
    distance_total: D,
    position: usize,
}

impl<D: PartialOrd> Eq for State<D> {
}

// Heap order depends on `Ord`, so implementing trait to make min-heap instead of max-heap
impl<D: Copy + PartialOrd> Ord for State<D> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.distance_total
            .partial_cmp(&self.distance_total)
            .unwrap()
            .then_with(|| self.position.cmp(&other.position))
    }
}

// It is required `PartialOrd` to be also implemented
impl<D: Copy + PartialOrd> PartialOrd for State<D> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn polyline_split<P, D>(
    polyline: &[P],
    points: &[P],
    distance_threshold: Option<D>,
) -> Result<Vec<Vec<P>>>
where
    P: DistanceToSegment<D>,
    D: Copy + PartialOrd + Add<Output = D>,
{
    if polyline.len() <= 1 {
        return Err(PolySplitError{
            kind: PolySplitErrorKind::InvalidPolyline,
            message: "polyline has not enough points".to_string(),
        });
    }

    if points.len() <= 1 {
        return Err(PolySplitError{
            kind: PolySplitErrorKind::InvalidPoints,
            message: "number of points are not enough".to_string(),
        });
    }

    let segments_len = polyline.len() - 1;
    let points_len = points.len();
    let segments: Vec<(&P, &P)> = polyline.iter().tuple_windows().collect();

    // Building graph
    let mut vertexes: Vec<Vertex<P, D>> = Vec::new();
    let mut edges: Vec<(usize, usize)> = Vec::new();

    let mut last_reachable_segment_index = 0;

    for (point_index, point) in points.iter().enumerate() {
        let start_position = vertexes.len();
        let mut first_match_segment_index = None;

        for (segment_index, segment) in segments.iter().enumerate().take(segments_len).skip(last_reachable_segment_index) {
            let psd: DistanceToSegmentResult<P, D> = point.distance_to_segment(*segment);
            if let Some(dt) = distance_threshold {
                if psd.distance > dt {
                    continue;
                }
            }

            if first_match_segment_index.is_none() {
                first_match_segment_index = Some(segment_index);
            }

            vertexes.push(Vertex {
                point_index,
                segment_index,
                distance_to: psd.distance,
                cut_point: psd.cut_point,
                cut_ratio: psd.cut_ratio,
            });
        }

        let end_position = vertexes.len();
        if start_position == end_position {
            return Err(PolySplitError{
                kind: PolySplitErrorKind::PointFarAway,
                message: "point has no closest segments".to_string(),
            });
        }

        edges.push((start_position, end_position));
        last_reachable_segment_index = first_match_segment_index.unwrap_or_default();
    }

    // Initializing
    let vertexes_len = vertexes.len();
    let mut dist: Vec<Option<D>> = (0..vertexes_len).map(|_| None).collect();
    let mut prev: Vec<Option<usize>> = (0..vertexes_len).map(|_| None).collect();
    let mut priority_queue = BinaryHeap::new();

    for idx in edges[0].0..edges[0].1 {
        let vertex = &vertexes[idx];

        dist[idx] = Some(vertex.distance_to);
        prev[idx] = None;
        priority_queue.push(State {
            distance_total: vertex.distance_to,
            position: idx,
        });
    }

    // Calculating
    let mut destination = None;
    while let Some(State { distance_total, position }) = priority_queue.pop() {
        let current_vertex = &vertexes[position];

        // Goal is reached
        if current_vertex.point_index + 1 == points_len {
            destination = Some(position);
            break;
        }

        // Useless state because there is better one
        if let Some(d) = dist[position] {
            if distance_total > d {
                continue;
            }
        }

        // Iterating connected vertexes
        let (from_idx, to_idx) = edges[current_vertex.point_index + 1];
        for idx in from_idx..to_idx {
            let neighbour_vertex = &vertexes[idx];

            if current_vertex.segment_index > neighbour_vertex.segment_index ||
                (current_vertex.segment_index == neighbour_vertex.segment_index && current_vertex.cut_ratio > neighbour_vertex.cut_ratio) {
                continue;
            }

            let relaxed_distance_total = distance_total + neighbour_vertex.distance_to;
            if dist[idx].map_or(true, |d| d > relaxed_distance_total) {
                    dist[idx] = Some(relaxed_distance_total);
                    prev[idx] = Some(position);
                    priority_queue.push(State {
                        distance_total: relaxed_distance_total,
                        position: idx,
                    });
            }
        }
    }

    // Restoring path
    let mut path = Vec::new();
    while let Some(idx) = destination {
        path.push(idx);
        destination = prev[idx];
    }

    if path.is_empty() {
        return Err(PolySplitError{
            kind: PolySplitErrorKind::CannotSplit,
            message: "cannot split polyline".to_string(),
        });
    }

    path.reverse();

    // Building sub-segments
    let mut segments: Vec<_> = Vec::with_capacity(segments_len);
    let mut current_vertex = &vertexes[path[0]];

    for next_idx in path[1..].iter() {
        let next_vertex: &Vertex<P, D> = &vertexes[*next_idx];
        let mut segment: Vec<_> = Vec::new();

        if current_vertex.cut_ratio < 1.0 {
            segment.push(current_vertex.cut_point);
        }

        for segment_idx in current_vertex.segment_index..next_vertex.segment_index {
            segment.push(polyline[segment_idx + 1]);
        }

        if next_vertex.cut_ratio > 0.0 {
            segment.push(next_vertex.cut_point);
        }

        if segment.len() == 1 {
            segment.push(segment[0]);
        }

        segments.push(segment);
        current_vertex = next_vertex;
    }

    Ok(segments)
}
