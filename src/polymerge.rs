use std::ops::Add;

use crate::{PolySplit, polyline_split};
use crate::euclidean::Point;

/// PolyMerge defines methods for types that can be used in **polyline_merge** method.
pub trait PolyMerge<D>
where
    Self: PolySplit<D>,
    D: Copy + PartialOrd + Add<Output = D>,
{
    /// Returns middle point between current and another points.
    ///
    /// # Arguments
    ///
    /// * `point` - A point for what middle point should be returned
    fn middle_point(&self, point: &Self) -> Self;
}


// fn polyline_split_signature<P, D>(
//     polyline: &[P],
//     points: &[P],
//     distance_threshold: Option<D>,
// ) -> Result<Vec<Vec<P>>>
// where
//     P: PolySplit<D> + std::fmt::Debug,
//     D: Copy + PartialOrd + Add<Output = D>,
// {}

pub fn polyline_merge<P, D>(b: &[P], a: &[P], distance_threshold: Option<D>) -> bool
where
    P: PolyMerge<D> + std::fmt::Debug,
    D: Copy + PartialOrd + Add<Output = D>,
{
    let first_split = polyline_split(a, b, distance_threshold);
    let first_segments = match first_split {
        Ok(v) => v,
        Err(_) => return false,
    };

    let mut polyline = Vec::new();
    polyline.extend_from_slice(&first_segments[0]);
    for segment in first_segments.iter().skip(1) {
        polyline.extend_from_slice(&segment[1..]);
    }

    let second_split = polyline_split(b, &polyline, distance_threshold);
    let second_segments = match second_split {
        Ok(v) => v,
        Err(_) => return false,
    };

    // 0
    let p0 = polyline[0];
    let p1 = second_segments[0][0];
    // polyline[0] = Point((p0.0 + p1.0) / 2.0, (p0.1 + p1.1) / 2.0);
    polyline[0] = p0.middle_point(&p1);

    for (i, p0) in polyline.iter_mut().skip(1).enumerate() {
        let p1 = second_segments[i].last().unwrap();
        // polyline[i] = Point((p0.0 + p1.0) / 2.0, (p0.1 + p1.1) / 2.0);
        // *p0 = Point((p0.0 + p1.0) / 2.0, (p0.1 + p1.1) / 2.0);
        *p0 = p0.middle_point(&p1);
    }

    let s = format!("{:?}", polyline).replace("Point", "");
    panic!("{}", s);

    // println!("{:?}", polyline);
    // panic!("asd");

    true
}

fn polyline_equal3(a: &[Point], b: &[Point], threshold: f64) -> bool {
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



fn polyline_equal1(a: &[Point], b: &[Point], threshold: f64) -> bool {
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


#[cfg(test)]
mod tests {
    use crate::euclidean::Point;
    use crate::polyline_merge;

    #[test]
    fn test_example01() {
        let a = vec![(20.0, 20.0), (80.0, 40.0), (160.0, 80.0), (240.0, 60.0), (300.0, 60.0), (360.0, 100.0), (440.0, 100.0), (500.0, 80.0), (560.0, 60.0), (620.0, 60.0), (660.0, 100.0), (720.0, 100.0), (780.0, 80.0), (840.0, 60.0)];
        let b = vec![(40.0, 40.0), (80.0, 20.0), (140.0, 40.0), (200.0, 80.0), (260.0, 60.0), (340.0, 60.0), (340.0, 100.0), (400.0, 80.0), (480.0, 100.0), (520.0, 60.0), (600.0, 40.0), (680.0, 100.0), (740.0, 60.0), (860.0, 40.0)];

        let p_a: Vec<_> = a.iter().map(|p| Point(p.0, p.1)).collect();
        let p_b: Vec<_> = b.iter().map(|p| Point(p.0, p.1)).collect();

        assert_eq!(polyline_merge(&p_a, &p_b, Some(35.0)), true);
        assert_eq!(polyline_merge(&p_a, &p_b, Some(10.0)), false);
    }

    #[test]
    fn test_example02() {
        let a = vec![(40.0, 60.0), (140.0, 60.0), (160.0, 100.0), (220.0, 120.0), (280.0, 100.0), (320.0, 80.0), (420.0, 60.0), (440.0, 120.0), (500.0, 160.0), (540.0, 140.0), (580.0, 120.0), (640.0, 80.0), (720.0, 100.0), (760.0, 160.0), (860.0, 140.0), (980.0, 120.0), (1080.0, 80.0), (1080.0, 40.0)];
        let b = vec![(60.0, 40.0), (160.0, 60.0), (200.0, 100.0), (260.0, 80.0), (340.0, 60.0), (380.0, 80.0), (440.0, 80.0), (500.0, 120.0), (520.0, 160.0), (580.0, 140.0), (640.0, 100.0), (680.0, 100.0), (720.0, 140.0), (780.0, 140.0), (800.0, 160.0), (840.0, 140.0), (880.0, 120.0), (920.0, 140.0), (940.0, 100.0), (1000.0, 100.0), (1040.0, 100.0), (1100.0, 60.0), (1080.0, 40.0)];

        let p_a: Vec<_> = a.iter().map(|p| Point(p.0, p.1)).collect();
        let p_b: Vec<_> = b.iter().map(|p| Point(p.0, p.1)).collect();

        assert_eq!(polyline_merge(&p_a, &p_b, Some(35.0)), true);
        assert_eq!(polyline_merge(&p_a, &p_b, Some(10.0)), false);
    }

    #[test]
    fn test_example03() {
        let a = vec![(20.0, 20.0), (80.0, 60.0), (140.0, 100.0), (180.0, 120.0), (220.0, 160.0), (260.0, 200.0), (300.0, 240.0), (340.0, 220.0), (360.0, 200.0), (400.0, 160.0), (420.0, 120.0), (480.0, 100.0), (500.0, 120.0), (560.0, 120.0), (620.0, 120.0), (720.0, 100.0), (780.0, 100.0), (840.0, 80.0), (860.0, 120.0)];
        let b = vec![(40.0, 60.0), (100.0, 40.0), (120.0, 60.0), (160.0, 80.0), (200.0, 120.0), (240.0, 120.0), (280.0, 140.0), (320.0, 140.0), (340.0, 140.0), (400.0, 120.0), (420.0, 100.0), (460.0, 80.0), (500.0, 100.0), (540.0, 120.0), (580.0, 140.0), (640.0, 120.0), (660.0, 100.0), (720.0, 80.0), (740.0, 80.0), (800.0, 100.0), (840.0, 100.0)];

        let p_a: Vec<_> = a.iter().map(|p| Point(p.0, p.1)).collect();
        let p_b: Vec<_> = b.iter().map(|p| Point(p.0, p.1)).collect();

        assert_eq!(polyline_merge(&p_a, &p_b, Some(35.0)), false);
        assert_eq!(polyline_merge(&p_a, &p_b, Some(10.0)), false);
    }
}
