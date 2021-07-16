use crate::{HEIGHT, WIDTH, X_OFFSET, Y_OFFSET};
use itertools::Itertools;
use nannou::prelude::*;

pub struct Grid {
    num_lines: u32,
    resolution: u32,
}
impl Grid {
    pub fn new(num_lines: u32, resolution: u32) -> Grid {
        Grid {
            num_lines,
            resolution,
        }
    }

    pub fn iter_points(&self) -> impl Iterator<Item = Point2> {
        let xs = (0..=self.resolution).into_iter().map(|n| n as f32);
        let ys = (0..=self.num_lines).into_iter().map(|n| n as f32);

        let scale_h: f32 = WIDTH as f32 / self.resolution as f32;
        let scale_v: f32 = HEIGHT as f32 / self.num_lines as f32;

        ys.cartesian_product(xs)
            .map(move |(y, x)| vec2(x * scale_h - X_OFFSET, y * scale_v - Y_OFFSET))
    }

    pub fn _draw(&self, draw: &Draw) {
        self.iter_points()
            .tuple_windows()
            .filter(|(p0, p1)| p0.y == p1.y)
            .for_each(|(p0, p1)| {
                draw.line().start(p0).end(p1).weight(2.0);
            })
    }
}
