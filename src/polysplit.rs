use std::cmp::{Ord, Ordering, PartialEq, Eq, PartialOrd};
use std::collections::BinaryHeap;
use std::ops::Add;

use crate::{PolySplitErrorKind, PolySplitError, Result};
use crate::{CutRatioResult, DistanceToSegmentResult};

/// PolySplit defines methods for types that can be used in **polyline_split** method.
pub trait PolySplit<D>
where
    Self: Copy,
    D: Copy + PartialOrd + Add<Output = D>,
{
    /// Returns distance to another point.
    ///
    /// # Arguments
    ///
    /// * `point` - A point distance to should be calculated
    fn distance_to_point(&self, point: &Self) -> D;
    /// Returns projection [results](DistanceToSegmentResult) to the segment.
    ///
    /// # Arguments
    ///
    /// * `segment` - A segment presented by a tuple of points
    fn distance_to_segment(&self, segment: (&Self, &Self)) -> DistanceToSegmentResult<Self, D>;
}

struct CutPoint<P>
where P: std::fmt::Debug {
    segment_index: usize,
    cut_ratio: CutRatioResult,
    cut_point: P,
}

struct Vertex<D> {
    point_index: usize,
    cut_point_index: usize,
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

/// Splits polyline into segments by the defined list of points.
///
/// # Examples
///
/// ```
/// use polysplit::euclidean::Point;
/// use polysplit::polyline_split;
///
/// let polyline = vec![
///     Point(0.0, 0.0),
///     Point(10.0, 0.0),
///     Point(20.0, 0.0),
/// ];
/// let points = vec![
///     Point(1.0, 1.0),
///     Point(19.0, 1.0),
/// ];
///
/// let segments = polyline_split(&polyline, &points, None).unwrap();
///
/// assert_eq!(segments.len(), 1);
/// println!("{:?}", segments[0]);
/// ```
pub fn polyline_split<P, D>(
    polyline: &[P],
    points: &[P],
    distance_threshold: Option<D>,
) -> Result<Vec<Vec<P>>>
where
    P: PolySplit<D> + std::fmt::Debug,
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

    // Collecting all possible cut points
    let mut cut_points: Vec<CutPoint<P>> = Vec::new();

    for segment_index in 0..segments_len {
        let segment_a = &polyline[segment_index];
        let segment_b = &polyline[segment_index + 1];

        let mut is_start_added = false;
        let mut is_end_added = false;

        for point in points.iter() {
            let psd: DistanceToSegmentResult<P, D> = point.distance_to_segment((segment_a, segment_b));
            if let Some(dt) = distance_threshold {
                if psd.distance > dt {
                    continue;
                }
            }

            match psd.cut_ratio {
                CutRatioResult::Begin => {
                    if segment_index == 0 && !is_start_added {
                        cut_points.push(CutPoint {
                            segment_index,
                            cut_ratio: psd.cut_ratio,
                            cut_point: *segment_a,
                        });

                        is_start_added = true;
                    }
                }

                CutRatioResult::End => {
                    if !is_end_added {
                        cut_points.push(CutPoint {
                            segment_index,
                            cut_ratio: psd.cut_ratio,
                            cut_point: *segment_b,
                        });

                        is_end_added = true;
                    }
                },

                _ => {
                    cut_points.push(CutPoint {
                        segment_index,
                        cut_ratio: psd.cut_ratio,
                        cut_point: psd.cut_point,
                    });
                }
            }
        }
    }

    cut_points.sort_unstable_by(|a, b| {
        match a.segment_index.cmp(&b.segment_index) {
            Ordering::Equal => a.cut_ratio.partial_cmp(&b.cut_ratio).unwrap(),
            v => v,
        }
    });

    // Building graph
    let mut vertexes: Vec<Vertex<D>> = Vec::new();
    let mut edges: Vec<(usize, usize)> = Vec::new();

    let mut last_reachable_cut_point_index = 0;

    for (point_index, point) in points.iter().enumerate() {
        let start_position = vertexes.len();
        let mut first_match_cut_point_index = None;

        for (cut_point_index, cut_point) in cut_points.iter().enumerate().skip(last_reachable_cut_point_index) {
            let distance_to = point.distance_to_point(&cut_point.cut_point);
            if let Some(dt) = distance_threshold {
                if distance_to > dt {
                    continue;
                }
            }

            if first_match_cut_point_index.is_none() {
                first_match_cut_point_index = Some(cut_point_index);
            }

            vertexes.push(Vertex {
                point_index,
                cut_point_index,
                distance_to,
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
        last_reachable_cut_point_index = first_match_cut_point_index.unwrap_or_default();
    }

    // Initializing start points
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

    // Searching for shortest path using Dijkstra's algorithm
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

            if current_vertex.cut_point_index > neighbour_vertex.cut_point_index {
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
    let mut current_cut_point = &cut_points[current_vertex.cut_point_index];

    for next_idx in path[1..].iter() {
        let next_vertex: &Vertex<D> = &vertexes[*next_idx];
        let next_cut_point = &cut_points[next_vertex.cut_point_index];
        let mut segment: Vec<_> = Vec::new();

        if !matches!(current_cut_point.cut_ratio, CutRatioResult::End) {
            segment.push(current_cut_point.cut_point);
        }

        for segment_idx in current_cut_point.segment_index..next_cut_point.segment_index {
            segment.push(polyline[segment_idx + 1]);
        }

        if !matches!(next_cut_point.cut_ratio, CutRatioResult::Begin) {
            segment.push(next_cut_point.cut_point);
        }

        // Two points are matched to same cut point
        // So adding same point to be valid segment
        if segment.len() == 1 {
            segment.push(segment[0]);
        }

        segments.push(segment);
        current_vertex = next_vertex;
        current_cut_point = &cut_points[current_vertex.cut_point_index];
    }

    Ok(segments)
}
