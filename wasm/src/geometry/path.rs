use cgmath::{point2, Point2};

pub struct Path {
    commands: Vec<PathCommand>,
}

impl Path {
    pub fn new() -> Path {
        Path {
            commands: Vec::new(),
        }
    }

    pub fn move_to(&mut self, point: Point2<f32>) {
        self.commands.push(PathCommand::MoveTo(point));
    }

    pub fn line_to(&mut self, point: Point2<f32>) {
        self.commands.push(PathCommand::LineTo(point));
    }

    pub fn quad_to(&mut self, control: Point2<f32>, point: Point2<f32>) {
        self.commands.push(PathCommand::QuadTo(control, point));
    }

    pub fn cubic_to(&mut self, control1: Point2<f32>, control2: Point2<f32>, point: Point2<f32>) {
        self.commands
            .push(PathCommand::CubicTo(control1, control2, point));
    }

    pub fn close(&mut self) {
        self.commands.push(PathCommand::Close);
    }

    pub fn commands(&self) -> &[PathCommand] {
        &self.commands
    }

    // TODO: use iterator
    pub fn to_segments(&self) -> Vec<Segment> {
        let mut start = point2(0.0, 0.0);
        let mut segments = Vec::new();

        for command in self.commands {
            match command {
                PathCommand::MoveTo(point) => {
                    start = point;
                }
                PathCommand::LineTo(point) => {
                    segments.push(Segment::Line(start, point));
                    start = point;
                }
                PathCommand::QuadTo(control, point) => {
                    segments.push(Segment::Quad(start, control, point));
                    start = point;
                }
                PathCommand::CubicTo(control1, control2, point) => {
                    segments.push(Segment::Cubic(start, control1, control2, point));
                    start = point;
                }
                PathCommand::Close => {
                    // TODO
                }
            }
        }

        return segments;
    }
}

pub enum PathCommand {
    MoveTo(Point2<f32>),
    LineTo(Point2<f32>),
    QuadTo(Point2<f32>, Point2<f32>),
    CubicTo(Point2<f32>, Point2<f32>, Point2<f32>),
    Close,
}

pub enum Segment {
    Line(Point2<f32>, Point2<f32>),
    Quad(Point2<f32>, Point2<f32>, Point2<f32>),
    Cubic(Point2<f32>, Point2<f32>, Point2<f32>, Point2<f32>),
}