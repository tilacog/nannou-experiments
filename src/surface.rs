use crate::grid::Grid;
use crate::segment::Segment;
use itertools::Itertools;
use nannou::{noise::*, prelude::*};

const SCALE: f64 = 0.01;

pub struct Surface {
    noise: OpenSimplex,
}

impl Surface {
    pub fn new() -> Surface {
        Surface {
            noise: OpenSimplex::new(),
        }
    }

    fn project_grid_of_points(&self, grid_2d: Vec<Vec<Point2>>, offset: Vec2) -> Vec<Vec<Point3>> {
        let mut grid_3d = Vec::new();
        for line_2d in grid_2d.into_iter() {
            let mut line_3d = Vec::new();
            for point_2d in line_2d.into_iter() {
                let z = {
                    let point = point_2d + offset;
                    let x = point.x as f64 * SCALE;
                    let y = point.y as f64 * SCALE;
                    self.noise.get([x, y]) as f32
                };
                let point_3d = point_2d.extend(z);
                line_3d.push(point_3d)
            }
            grid_3d.push(line_3d);
        }
        grid_3d
    }

    fn segment_grid(&self, grid_3d: Vec<Vec<Point3>>, angle: f32) -> Vec<Vec<Segment>> {
        let z_scale = 60.0;
        let z_coef = angle.sin();
        let y_coef = angle.cos();

        let mut segment_grid = Vec::new();
        for line in grid_3d.into_iter() {
            let mut segment_line = Vec::new();
            for (p0, p1) in line.into_iter().tuple_windows() {
                let y0 = p0.y * y_coef + p0.z * z_coef * z_scale;
                let y1 = p1.y * y_coef + p1.z * z_coef * z_scale;
                let q0 = pt2(p0.x, y0);
                let q1 = pt2(p1.x, y1);
                let segment = Segment { start: q0, end: q1 };
                segment_line.push(segment);
            }
            segment_grid.push(segment_line);
        }
        segment_grid
    }

    pub fn draw(&self, grid: &Grid, draw: &Draw, angle: f32, offset: Vec2) {
        let point_2d_grid = grid.grid_of_points();
        let point_3d_grid = self.project_grid_of_points(point_2d_grid, offset);
        let segment_grid = self.segment_grid(point_3d_grid, angle);

        let mut maximums = vec![f32::NEG_INFINITY; grid.resolution as usize];

        for line in segment_grid.into_iter() {
            for (pos, segment) in line.into_iter().enumerate() {
                let local_max = &mut maximums[pos];
                if segment.start.y > *local_max {
                    *local_max = segment.start.y;
                    segment.draw(&draw);
                }
            }
        }
    }
}
