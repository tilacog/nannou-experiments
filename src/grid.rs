use nannou::{
    prelude::{pt2, Point2, PLUM},
    Draw,
};

pub struct Grid {
    points: Vec<Point2>,
}

impl Grid {
    pub(crate) fn new(width: f32, height: f32, spacing: f32) -> Grid {
        let mut points = Vec::new();

        let num_rows = (width / spacing) as usize;
        let num_cols = (height / spacing) as usize;

        for row in 1..num_rows {
            for col in 1..num_cols {
                let x = -width / 2.0 + spacing * row as f32;
                let y = -height / 2.0 + spacing * col as f32;
                let p = pt2(x, y);
                points.push(p);
            }
        }
        Grid { points }
    }

    pub(crate) fn draw(&self, draw: &Draw) {
        for p in &self.points {
            draw.ellipse().xy(*p).w_h(2.0, 2.0).color(PLUM);
        }
    }
}
