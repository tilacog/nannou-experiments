use crate::{HEIGHT, WIDTH, X_OFFSET, Y_OFFSET};

use nannou::prelude::*;

pub struct Grid {
    pub num_lines: u32,
    pub resolution: u32,
}
impl Grid {
    pub fn new(num_lines: u32, resolution: u32) -> Grid {
        Grid {
            num_lines,
            resolution,
        }
    }

    pub fn grid_of_points(&self) -> Vec<Vec<Point2>> {
        let scale_h: f32 = WIDTH as f32 / self.resolution as f32;
        let scale_v: f32 = HEIGHT as f32 / self.num_lines as f32;
        let mut grid = Vec::new();
        for y in (0..=self.num_lines).into_iter().map(|n| n as f32) {
            let mut line = Vec::new();
            for x in (0..self.resolution).into_iter().map(|n| n as f32) {
                let point = pt2(x * scale_h - X_OFFSET, y * scale_v - Y_OFFSET);
                line.push(point);
            }
            grid.push(line);
        }
        grid
    }
}
