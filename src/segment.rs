use crate::grid::Grid;
use nannou::prelude::*;

pub struct Segment {
    pub start: Point2,
    pub end: Point2,
}

impl Segment {
    pub fn draw(&self, draw: &Draw, grid: &Grid, angle: f32) {
        let line_color = hsva(0.0, 0.0, 0.0, 0.75);
        draw.line()
            .start(self.start)
            .end(self.end)
            .weight(2.5)
            .color(line_color);
    }
}
