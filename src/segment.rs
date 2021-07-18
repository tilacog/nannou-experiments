use crate::grid::Grid;
use nannou::prelude::*;

#[derive(Debug)]
pub struct Segment {
    pub start: Point2,
    pub end: Point2,
}

impl Segment {
    pub fn draw(&self, draw: &Draw) {
        let line_color = hsva(0.0, 0.0, 0.0, 0.75);
        draw.line()
            .start(self.start)
            .end(self.end)
            .weight(1.0)
            .color(line_color);
    }
}
