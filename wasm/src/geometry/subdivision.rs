// port of Anti-Grain Geometry curve subdivision
// http://agg.sourceforge.net/antigrain.com/research/adaptive_bezier/index.html

use std::f64::consts::PI;

use cgmath::{point2, Point2};

use super::path::Segment;

const curve_distance_epsilon: f64 = 1e-30;
const curve_collinearity_epsilon: f64 = 1e-30;
const curve_angle_tolerance_epsilon: f64 = 0.01;
const curve_recursion_limit: u32 = 32;

fn calc_sq_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    return dx * dx + dy * dy;
}

pub struct Subdivision {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64,
    x4: f64,
    y4: f64,
    approximation_scale: f64,
    distance_tolerance_square: f64,
    angle_tolerance: f64,
    cusp_limit: f64,
    points: Vec<Point2<f64>>,
}

impl Subdivision {
    pub fn new(cubic: &Segment) {
        let (x1, y1, x2, y2, x3, y3, x4, y4) = match cubic {
            Segment::Cubic(p1, p2, p3, p4) => (
                p1.x as f64,
                p1.y as f64,
                p2.x as f64,
                p2.y as f64,
                p3.x as f64,
                p3.y as f64,
                p4.x as f64,
                p4.y as f64,
            ),
            _ => panic!("Subdivision::new() called with non-cubic segment"),
        };

        let approximation_scale = 1.0;

        let mut distance_tolerance_square = 0.5 / approximation_scale;
        distance_tolerance_square *= distance_tolerance_square;

        Subdivision {
            x1,
            y1,
            x2,
            y2,
            x3,
            y3,
            x4,
            y4,
            approximation_scale,
            distance_tolerance_square,
            angle_tolerance: 0.0,
            cusp_limit: 0.0,
            points: Vec::new(),
        };
    }

    fn recursive_bezier(
        &mut self,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        x3: f64,
        y3: f64,
        x4: f64,
        y4: f64,
        level: u32,
    ) {
        if level > curve_recursion_limit {
            return;
        }

        // Calculate all the mid-points of the line segments
        //----------------------
        let x12 = (x1 + x2) / 2.0;
        let y12 = (y1 + y2) / 2.0;
        let x23 = (x2 + x3) / 2.0;
        let y23 = (y2 + y3) / 2.0;
        let x34 = (x3 + x4) / 2.0;
        let y34 = (y3 + y4) / 2.0;
        let x123 = (x12 + x23) / 2.0;
        let y123 = (y12 + y23) / 2.0;
        let x234 = (x23 + x34) / 2.0;
        let y234 = (y23 + y34) / 2.0;
        let x1234 = (x123 + x234) / 2.0;
        let y1234 = (y123 + y234) / 2.0;

        // Try to approximate the full cubic curve by a single straight line
        //------------------
        let dx = x4 - x1;
        let dy = y4 - y1;

        let mut d2 = ((x2 - x4) * dy - (y2 - y4) * dx).abs();
        let mut d3 = ((x3 - x4) * dy - (y3 - y4) * dx).abs();
        let mut da1: f64;
        let mut da2: f64;
        let mut k: f64;

        match (((d2 > curve_collinearity_epsilon) as u32) << 1)
            + (d3 > curve_collinearity_epsilon) as u32
        {
            0 => {
                // All collinear OR p1==p4
                //----------------------
                k = dx * dx + dy * dy;
                if k == 0.0 {
                    d2 = calc_sq_distance(x1, y1, x2, y2);
                    d3 = calc_sq_distance(x4, y4, x3, y3);
                } else {
                    k = 1.0 / k;
                    da1 = x2 - x1;
                    da2 = y2 - y1;
                    d2 = k * (da1 * dx + da2 * dy);
                    da1 = x3 - x1;
                    da2 = y3 - y1;
                    d3 = k * (da1 * dx + da2 * dy);
                    if d2 > 0.0 && d2 < 1.0 && d3 > 0.0 && d3 < 1.0 {
                        // Simple collinear case, 1---2---3---4
                        // We can leave just two endpoints
                        return;
                    }
                    if d2 <= 0.0 {
                        d2 = calc_sq_distance(x2, y2, x1, y1);
                    } else if d2 >= 1.0 {
                        d2 = calc_sq_distance(x2, y2, x4, y4);
                    } else {
                        d2 = calc_sq_distance(x2, y2, x1 + d2 * dx, y1 + d2 * dy);
                    }

                    if d3 <= 0.0 {
                        d3 = calc_sq_distance(x3, y3, x1, y1);
                    } else if d3 >= 1.0 {
                        d3 = calc_sq_distance(x3, y3, x4, y4);
                    } else {
                        d3 = calc_sq_distance(x3, y3, x1 + d3 * dx, y1 + d3 * dy);
                    }
                }
                if d2 > d3 {
                    if d2 < self.distance_tolerance_square {
                        self.points.push(point2(x2, y2));
                        return;
                    }
                } else {
                    if d3 < self.distance_tolerance_square {
                        self.points.push(point2(x3, y3));
                        return;
                    }
                }
            }
            1 => {
                // p1,p2,p4 are collinear, p3 is significant
                //----------------------
                if d3 * d3 <= self.distance_tolerance_square * (dx * dx + dy * dy) {
                    if self.angle_tolerance < curve_angle_tolerance_epsilon {
                        self.points.push(point2(x23, y23));
                        return;
                    }

                    // Angle Condition
                    //----------------------
                    da1 = (f64::atan2(y4 - y3, x4 - x3) - f64::atan2(y3 - y2, x3 - x2)).abs();
                    if da1 >= PI {
                        da1 = 2.0 * PI - da1;
                    }

                    if da1 < self.angle_tolerance {
                        self.points.push(point2(x2, y2));
                        self.points.push(point2(x3, y3));
                        return;
                    }

                    if self.cusp_limit != 0.0 {
                        if da1 > self.cusp_limit {
                            self.points.push(point2(x3, y3));
                            return;
                        }
                    }
                }
            }
            2 => {
                // p1,p3,p4 are collinear, p2 is significant
                //----------------------
                if d2 * d2 <= self.distance_tolerance_square * (dx * dx + dy * dy) {
                    if self.angle_tolerance < curve_angle_tolerance_epsilon {
                        self.points.push(point2(x23, y23));
                        return;
                    }

                    // Angle Condition
                    //----------------------
                    da1 = (f64::atan2(y3 - y2, x3 - x2) - f64::atan2(y2 - y1, x2 - x1)).abs();
                    if da1 >= PI {
                        da1 = 2.0 * PI - da1;
                    }

                    if da1 < self.angle_tolerance {
                        self.points.push(point2(x2, y2));
                        self.points.push(point2(x3, y3));
                        return;
                    }

                    if self.cusp_limit != 0.0 {
                        if da1 > self.cusp_limit {
                            self.points.push(point2(x2, y2));
                            return;
                        }
                    }
                }
            }
            3 => {
                // Regular case
                //-----------------
                if (d2 + d3) * (d2 + d3) <= self.distance_tolerance_square * (dx * dx + dy * dy) {
                    // If the curvature doesn't exceed the distance_tolerance value
                    // we tend to finish subdivisions.
                    //----------------------
                    if self.angle_tolerance < curve_angle_tolerance_epsilon {
                        self.points.push(point2(x23, y23));
                        return;
                    }

                    // Angle & Cusp Condition
                    //----------------------
                    k = f64::atan2(y3 - y2, x3 - x2);
                    da1 = (k - f64::atan2(y2 - y1, x2 - x1)).abs();
                    da2 = (f64::atan2(y4 - y3, x4 - x3) - k).abs();
                    if da1 >= PI {
                        da1 = 2.0 * PI - da1;
                    }
                    if da2 >= PI {
                        da2 = 2.0 * PI - da2;
                    }

                    if da1 + da2 < self.angle_tolerance {
                        // Finally we can stop the recursion
                        //----------------------
                        self.points.push(point2(x23, y23));
                        return;
                    }

                    if self.cusp_limit != 0.0 {
                        if da1 > self.cusp_limit {
                            self.points.push(point2(x2, y2));
                            return;
                        }

                        if da2 > self.cusp_limit {
                            self.points.push(point2(x3, y3));
                            return;
                        }
                    }
                }
            }
            _ => {
                unreachable!();
            }
        }

        // Continue subdivision
        //----------------------
        self.recursive_bezier(x1, y1, x12, y12, x123, y123, x1234, y1234, level + 1);
        self.recursive_bezier(x1234, y1234, x234, y234, x34, y34, x4, y4, level + 1);
    }
}
