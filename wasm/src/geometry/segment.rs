use cgmath::Point2;

pub enum Segment {
    Line(Point2<f64>, Point2<f64>),
    Quad(Point2<f64>, Point2<f64>, Point2<f64>),
    Cubic(Point2<f64>, Point2<f64>, Point2<f64>, Point2<f64>),
}
