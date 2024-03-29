//! # Polyline Split
//!
//! `polysplit` crate implements the algorithm allowing to split polylines
//! into segments by the defined list of points not necessary belonging to the polyline.
mod polysplit;
pub use crate::polysplit::{CutRatioResult, DistanceToSegmentResult, PolySplit};
pub use crate::polysplit::{PolySplitErrorKind, PolySplitError, Result};
pub use crate::polysplit::polyline_split;

pub mod euclidean;

#[cfg(test)]
mod tests {
    use crate::polysplit::polyline_split;
    use crate::euclidean::Point;

    fn is_equal(actual: &Vec<Vec<Point>>, expected: &Vec<Vec<(f64, f64)>>) -> bool {
        const EPS: f64 = 1E-16;

        if actual.len() != expected.len() {
            return false;
        }

        for (actual_segment, expected_segment) in actual.iter().zip(expected) {
            if actual_segment.len() != expected_segment.len() {
                return false;
            }

            for (actual_point, expected_point) in actual_segment.iter().zip(expected_segment) {
                if actual_point.distance_to(&Point(expected_point.0, expected_point.1)) > EPS {
                    return false;
                }
            }
        }

        true
    }

    #[test]
    fn tests_simple() {
        let tests = vec![
            // Simple tests
            (
                vec![(0.0, 0.0), (10.0, 0.0), (20.0, 0.0)],
                vec![(1.0, 1.0), (19.0, 1.0)],
                vec![vec![(1.0, 0.0), (10.0, 0.0), (19.0, 0.0)]],
            ),

            (
                vec![(0.0, 0.0), (10.0, 0.0), (20.0, 0.0)],
                vec![(10.0, 1.0), (19.0, 1.0)],
                vec![vec![(10.0, 0.0), (19.0, 0.0)]],
            ),

            (
                vec![(0.0, 0.0), (10.0, 0.0), (20.0, 0.0)],
                vec![(11.0, 1.0), (19.0, 1.0)],
                vec![vec![(11.0, 0.0), (19.0, 0.0)]],
            ),

            (
                vec![(0.0, 0.0), (10.0, 0.0), (20.0, 0.0)],
                vec![(0.0, 1.0), (10.0, 1.0)],
                vec![vec![(0.0, 0.0), (10.0, 0.0)]],
            ),

            (
                vec![(0.0, 0.0), (10.0, 0.0), (20.0, 0.0)],
                vec![(1.0, 1.0), (5.0, 1.0)],
                vec![vec![(1.0, 0.0), (5.0, 0.0)]],
            ),

            (
                vec![(0.0, 0.0), (10.0, 0.0), (20.0, 0.0)],
                vec![(10.0, 1.0), (10.0, 1.0)],
                vec![vec![(10.0, 0.0), (10.0, 0.0)]],
            ),

            (
                vec![(0.0, 0.0), (10.0, 0.0), (20.0, 0.0)],
                vec![(5.0, 1.0), (5.0, -1.0)],
                vec![vec![(5.0, 0.0), (5.0, 0.0)]],
            ),

            // One segment tests
            (
                vec![(0.0, 0.0), (10.0, 0.0)],
                vec![(1.0, 1.0), (9.0, 1.0)],
                vec![vec![(1.0, 0.0), (9.0, 0.0)]],
            ),

            (
                vec![(0.0, 0.0), (10.0, 0.0)],
                vec![(0.0, 1.0), (10.0, 1.0)],
                vec![vec![(0.0, 0.0), (10.0, 0.0)]],
            ),

            (
                vec![(0.0, 0.0), (10.0, 0.0)],
                vec![(5.0, 1.0), (5.0, 1.0)],
                vec![vec![(5.0, 0.0), (5.0, 0.0)]],
            ),

            (
                vec![(0.0, 0.0), (10.0, 0.0)],
                vec![(0.0, 1.0), (0.0, 1.0)],
                vec![vec![(0.0, 0.0), (0.0, 0.0)]],
            ),

            (
                vec![(0.0, 0.0), (10.0, 0.0)],
                vec![(10.0, 1.0), (10.0, 1.0)],
                vec![vec![(10.0, 0.0), (10.0, 0.0)]],
            ),

            (
                vec![(0.0, 0.0), (10.0, 0.0)],
                vec![(-2.0, 1.0), (-1.0, 1.0)],
                vec![vec![(0.0, 0.0), (0.0, 0.0)]],
            ),
        ];

        for (polyline, points, expected) in &tests {
            let polyline: Vec<Point> = polyline.iter().map(|p| Point(p.0, p.1)).collect();
            let points: Vec<Point> = points.iter().map(|p| Point(p.0, p.1)).collect();

            let result = polyline_split(&polyline, &points, None);
            let actual = result.unwrap();

            assert!(is_equal(&actual, expected), "polyline={:?}, points={:?}, actual={:?}", polyline, points, actual);
        }
    }

    #[test]
    fn tests_distance_threshold() {
        let tests = vec![
            (
                vec![(0.0, 0.0), (10.0, 0.0), (20.0, 0.0)],
                vec![(1.0, 1.0), (19.0, 1.0)],
            ),
        ];

        let polyline: Vec<Point> = tests[0].0.iter().map(|p| Point(p.0, p.1)).collect();
        let points: Vec<Point> = tests[0].1.iter().map(|p| Point(p.0, p.1)).collect();

        let result = polyline_split(
            &polyline,
            &points,
            Some(0.1),
        );

        assert!(result.is_err());
    }

    #[test]
    fn tests_polyline_errors() {
        let points : Vec<Point> = vec![];

        let polyline: Vec<Point> = vec![];
        assert!(polyline_split(&polyline, &points, None).is_err());

        let polyline: Vec<Point> = vec![Point(0.0, 0.0)];
        assert!(polyline_split(&polyline, &points, None).is_err());
    }

    #[test]
    fn tests_point_errors() {
        let polyline: Vec<Point> = vec![Point(0.0, 0.0), Point(1.0, 1.0)];

        let points: Vec<Point> = vec![];
        assert!(polyline_split(&polyline, &points, None).is_err());

        let points: Vec<Point> = vec![Point(0.0, 0.0)];
        assert!(polyline_split(&polyline, &points, None).is_err());
    }

    #[test]
    fn tests_examples() {
        let tests = vec![
            // Example 01
            (
                vec![(100.0, 140.0), (140.0, 200.0), (260.0, 300.0), (300.0, 240.0), (400.0, 220.0), (380.0, 260.0), (420.0, 340.0), (460.0, 340.0), (500.0, 320.0), (580.0, 280.0), (600.0, 240.0), (620.0, 180.0), (580.0, 160.0), (520.0, 140.0), (480.0, 100.0), (480.0, 60.0), (520.0, 40.0), (560.0, 40.0), (620.0, 60.0), (660.0, 80.0), (780.0, 180.0), (780.0, 300.0), (640.0, 360.0)],
                vec![(180.0, 200.0), (260.0, 240.0), (340.0, 260.0), (500.0, 280.0), (540.0, 160.0), (520.0, 60.0), (700.0, 160.0), (680.0, 380.0)],
                vec![
                    vec![(163.60655737704917, 219.672131147541), (260.0, 300.0), (287.6923076923077, 258.46153846153845)],
                    vec![(287.6923076923077, 258.46153846153845), (300.0, 240.0), (334.61538461538464, 233.07692307692307)],
                    vec![(334.61538461538464, 233.07692307692307), (400.0, 220.0), (380.0, 260.0), (420.0, 340.0), (460.0, 340.0), (500.0, 320.0), (516.0, 312.0)],
                    vec![(516.0, 312.0), (580.0, 280.0), (600.0, 240.0), (620.0, 180.0), (580.0, 160.0), (544.0, 148.0)],
                    vec![(544.0, 148.0), (520.0, 140.0), (480.0, 100.0), (480.0, 60.0), (512.0, 44.0)],
                    vec![(512.0, 44.0), (520.0, 40.0), (560.0, 40.0), (620.0, 60.0), (660.0, 80.0), (722.9508196721312, 132.45901639344262)],
                    vec![(722.9508196721312, 132.45901639344262), (780.0, 180.0), (780.0, 300.0), (666.551724137931, 348.62068965517244)],
                ],
            ),

            // Example 02
            (
                vec![(40.0, 60.0), (120.0, 60.0), (120.0, 140.0), (160.0, 200.0), (220.0, 200.0), (260.0, 140.0), (260.0, 60.0), (340.0, 60.0), (420.0, 200.0), (520.0, 60.0)],
                vec![(60.0, 80.0), (200.0, 180.0), (180.0, 120.0), (380.0, 180.0), (400.0, 60.0), (520.0, 100.0)],
                vec![
                    vec![(60.0, 60.0), (120.0, 60.0), (120.0, 140.0), (160.0, 200.0), (200.0, 200.0)],
                    vec![(200.0, 200.0), (220.0, 200.0), (244.6153846153846, 163.07692307692307)],
                    vec![(244.6153846153846, 163.07692307692307), (260.0, 140.0), (260.0, 60.0), (340.0, 60.0), (401.53846153846155, 167.69230769230768)],
                    vec![(401.53846153846155, 167.69230769230768), (420.0, 200.0), (479.4594594594595, 116.75675675675676)],
                    vec![(479.4594594594595, 116.75675675675676), (501.0810810810811, 86.48648648648648)]
                ]
            ),

            // Example 03
            (
                vec![(60.0, 80.0), (100.0, 140.0), (120.0, 160.0), (200.0, 160.0), (240.0, 140.0), (300.0, 100.0), (340.0, 60.0), (400.0, 60.0), (440.0, 80.0), (460.0, 120.0), (460.0, 180.0), (420.0, 220.0), (380.0, 260.0), (360.0, 280.0), (380.0, 320.0), (400.0, 360.0), (480.0, 320.0), (540.0, 300.0), (580.0, 260.0), (600.0, 220.0), (620.0, 160.0), (660.0, 120.0), (720.0, 100.0), (800.0, 100.0), (820.0, 140.0)],
                vec![(60.0, 120.0), (160.0, 140.0), (280.0, 80.0), (420.0, 120.0), (340.0, 140.0), (420.0, 320.0), (420.0, 260.0), (560.0, 220.0), (680.0, 120.0), (780.0, 140.0)],
                vec![
                    vec![(78.46153846153847, 107.6923076923077), (100.0, 140.0), (120.0, 160.0), (160.0, 160.0)],
                    vec![(160.0, 160.0), (200.0, 160.0), (240.0, 140.0), (295.38461538461536, 103.07692307692307)],
                    vec![(295.38461538461536, 103.07692307692307), (300.0, 100.0), (340.0, 60.0), (400.0, 60.0), (440.0, 80.0), (452.0, 104.0)],
                    vec![(452.0, 104.0), (460.0, 120.0), (460.0, 180.0), (420.0, 220.0)],
                    vec![(420.0, 220.0), (380.0, 260.0), (360.0, 280.0), (380.0, 320.0), (400.0, 360.0), (432.0, 344.0)],
                    vec![(432.0, 344.0), (456.0, 332.0)],
                    vec![(456.0, 332.0), (480.0, 320.0), (540.0, 300.0), (580.0, 260.0), (592.0, 236.0)],
                    vec![(592.0, 236.0), (600.0, 220.0), (620.0, 160.0), (660.0, 120.0), (678.0, 114.0)],
                    vec![(678.0, 114.0), (720.0, 100.0), (800.0, 100.0), (812.0, 124.0)]
                ]
            ),
        ];

        for (polyline, points, expected) in &tests {
            let polyline: Vec<Point> = polyline.iter().map(|p| Point(p.0, p.1)).collect();
            let points: Vec<Point> = points.iter().map(|p| Point(p.0, p.1)).collect();

            let result = polyline_split(&polyline, &points, None);
            let actual = result.unwrap();

            if !is_equal(&actual, expected) {
                let s = format!("{:?}", actual);
                let r = s.replace("Point", "");
                println!("{}", r);
            }

            assert!(is_equal(&actual, expected), "polyline={:?}, points={:?}", polyline, points);
        }
    }
}
