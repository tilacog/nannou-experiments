use nannou::prelude::*;

#[derive(Debug)]
pub struct Segment {
    pub start: Point2,
    pub end: Point2,
}

impl Segment {
    pub fn draw(&self, draw: &Draw, color: Option<Hsva>) {
        let line_color = if let Some(color) = color {
            color
        } else {
            hsva(0.0, 0.0, 0.5, 0.75)
        };
        draw.line()
            .start(self.start)
            .end(self.end)
            .weight(1.1)
            .color(line_color);
    }
}
