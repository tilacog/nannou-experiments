use nannou::{
    prelude::{pt2, Point2, Rgb, ORANGE, PLUM},
    Draw,
};

pub struct Grid {
    rows: usize,
    cols: usize,
    points: Vec<Point2>,
}

impl Grid {
    pub fn new(width: f32, height: f32, spacing: f32) -> Grid {
        let mut points = Vec::new();
        let rows = (width / spacing) as usize;
        let cols = (height / spacing) as usize;

        for row in 0..rows {
            for col in 0..cols {
                let x = -width / 2.0 + spacing * row as f32;
                let y = -height / 2.0 + spacing * col as f32;
                let p = pt2(x, y);
                points.push(p);
            }
        }
        Grid { rows, cols, points }
    }

    pub fn draw(&self, draw: &Draw) {
        for p in &self.points {
            draw.ellipse().xy(*p).w_h(2.0, 2.0).color(PLUM);
        }
    }

    pub fn iter(&self) -> SquareIterator {
        SquareIterator {
            grid: self,
            cursor: 0,
        }
    }
}

/// A---B
/// |   |
/// C---D
#[derive(Debug)]
pub struct Square<'a> {
    a: &'a Point2,
    b: &'a Point2,
    c: &'a Point2,
    d: &'a Point2,
}

impl<'a> Square<'a> {
    pub fn draw(&self, draw: &Draw, color: Option<Rgb<u8>>) {
        let color = match color {
            Some(c) => c,
            None => ORANGE,
        };

        for (m, n) in &[
            (self.a, self.b),
            (self.b, self.d),
            (self.d, self.c),
            (self.c, self.a),
        ] {
            draw.line().start(**m).end(**n).weight(3.0).color(color);
        }
    }
}

pub struct SquareIterator<'a> {
    grid: &'a Grid,
    cursor: usize,
}

impl<'a> Iterator for SquareIterator<'a> {
    type Item = Square<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // the last item is anchored in penultimate column of the penultimate row
        let grid_size = self.grid.rows * self.grid.cols;
        let last_anchor_position = grid_size - self.grid.cols - 2;

        if self.cursor > last_anchor_position {
            None
        } else {
            let a = &self.grid.points[self.cursor];
            let b = &self.grid.points[self.cursor + 1];
            let c = &self.grid.points[self.cursor + self.grid.cols];
            let d = &self.grid.points[self.cursor + self.grid.cols + 1];

            self.cursor += 1;

            Some(Square { a, b, c, d })
        }
    }
}
