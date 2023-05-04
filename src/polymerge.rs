use std::ops::Add;

use crate::{CutRatioResult, DistanceToSegmentResult};
use crate::{polyline_split, euclidean::Point};

/// PolyMerge defines methods for types that can be used in **polyline_merge** method.
pub trait PolyMerge<D>
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


pub fn polyline_merge(b: &[Point], a: &[Point], threshold: f64) -> bool {
    let threshold = Some(threshold);

    let first_split = polyline_split(a, b, threshold);
    let first_segments = match first_split {
        Ok(v) => v,
        Err(_) => return false,
    };

    let mut polyline = Vec::new();
    polyline.extend_from_slice(&first_segments[0]);
    for segment in first_segments.iter().skip(1) {
        polyline.extend_from_slice(&segment[1..]);
    }

    let second_split = polyline_split(b, &polyline, threshold);
    let second_segments = match second_split {
        Ok(v) => v,
        Err(_) => return false,
    };

    // 0
    let p0 = polyline[0];
    let p1 = second_segments[0][0];
    polyline[0] = Point((p0.0 + p1.0) / 2.0, (p0.1 + p1.1) / 2.0);

    for (i, p0) in polyline.iter_mut().skip(1).enumerate() {
        let p1 = second_segments[i].last().unwrap();
        // polyline[i] = Point((p0.0 + p1.0) / 2.0, (p0.1 + p1.1) / 2.0);
        *p0 = Point((p0.0 + p1.0) / 2.0, (p0.1 + p1.1) / 2.0);
    }

    let s = format!("{:?}", polyline).replace("Point", "");
    panic!("{}", s);

    // println!("{:?}", polyline);
    // panic!("asd");

    true
}

pub fn polyline_equal3(a: &[Point], b: &[Point], threshold: f64) -> bool {
    let threshold = Some(threshold);

    let a_split_by_b = match polyline_split(a, b, threshold) {
        Ok(segments) => segments,
        Err(_) => return false,
    };

    let mut c = Vec::<Point>::new();
    c.extend_from_slice(&a_split_by_b[0]);
    for segment in a_split_by_b.iter().skip(1) {
        c.extend_from_slice(&segment[1..]);
    }

    // println!("{}", format!("{:?}", c).replace("Point", ""));

    let b_split_by_c = match polyline_split(b, &c, threshold) {
        Ok(segments) => segments,
        Err(_) => return false,
    };


    let mut d = Vec::<Point>::new();
    d.extend_from_slice(&b_split_by_c[0]);
    for segment in b_split_by_c.iter().skip(1) {
        d.extend_from_slice(&segment[1..]);
    }

    // println!("{}", format!("{:?}", d).replace("Point", ""));


    // Building new polyline
    let mut result = Vec::<Point>::new();

    let mut b_point_index = 0;
    let mut index = 0;
    for segment in a_split_by_b.iter() {
        let p0 = segment[0];
        let p1 = b[b_point_index];
        b_point_index += 1;
        index += 1;

        result.push(Point((p0.0 + p1.0) / 2.0, (p0.1 + p1.1) / 2.0));

        for p0 in segment.iter().skip(1).take(segment.len() - 2) {
            // println!("index = {} {:?}", index, b_split_by_c[index][0]);
            let p1 = b_split_by_c[index][0];
            index += 1;

            result.push(Point((p0.0 + p1.0) / 2.0, (p0.1 + p1.1) / 2.0));
        }

        // index += 1;
    }

    let p0 = a_split_by_b.last().unwrap().last().unwrap();
    let p1 = b[b_point_index];

    result.push(Point((p0.0 + p1.0) / 2.0, (p0.1 + p1.1) / 2.0));

    let s = format!("{:?}", result).replace("Point", "");
    panic!("{}", s);

    true
}

// 0 ---  1 ---  2 ---  3 ---  4 --- 5
//
// 0 - 1, 1 - 2, 2 - 3, 3 - 4, 4 - 5



pub fn polyline_equal1(a: &[Point], b: &[Point], threshold: f64) -> bool {
    let threshold = Some(threshold);

    let first_split = polyline_split(a, b, threshold);
    let segments = match first_split {
        Ok(v) => v,
        Err(_) => return false,
    };

    let mut polyline = Vec::new();
    polyline.extend_from_slice(&segments[0]);
    for segment in segments.iter().skip(1) {
        polyline.extend_from_slice(&segment[1..]);
    }

    let second_split = polyline_split(b, &polyline, threshold);

    second_split.is_ok()
}
