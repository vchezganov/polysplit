use crate::polysplit::{DistanceToSegment, DistanceToSegmentResult};

#[derive(Clone, Copy, Debug)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance_to(&self, to: &Point) -> f64 {
        ((self.0 - to.0).powf(2.0) + (self.1 - to.1).powf(2.0)).sqrt()
    }
}

impl DistanceToSegment<f64> for Point {
    fn distance_to_segment(&self, s: (&Point, &Point)) -> DistanceToSegmentResult<Point, f64> {
        let segment_distance = s.0.distance_to(s.1);
        if segment_distance < 1e-9 {
            let distance = self.distance_to(s.0);
            return DistanceToSegmentResult{
                distance,
                cut_point: *s.0,
                cut_ratio: 0.0,
            };
        }

        let vx = s.1.0 - s.0.0;
        let vy = s.1.1 - s.0.1;

        let ux = self.0 - s.0.0;
        let uy = self.1 - s.0.1;

        let ratio = (ux*vx+uy*vy)/(vx*vx+vy*vy);
        let cut_ratio = ratio.min(1.0).max(0.0);

        if cut_ratio <= 0.0 {
            let distance = self.distance_to(s.0);

            DistanceToSegmentResult{
                distance,
                cut_point: *s.0,
                cut_ratio: 0.0,
            }
        } else if cut_ratio >= 1.0 {
            let distance = self.distance_to(s.1);

            DistanceToSegmentResult{
                distance,
                cut_point: *s.1,
                cut_ratio: 1.0,
            }
        } else {
            let x = s.0.0 + cut_ratio * vx;
            let y = s.0.1 + cut_ratio * vy;
            let cut_point = Point(x, y);
            let distance = self.distance_to(&cut_point);

            DistanceToSegmentResult{
                distance,
                cut_point,
                cut_ratio,
            }
        }
    }
}