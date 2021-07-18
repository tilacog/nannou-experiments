use crate::grid::Grid;
use crate::segment::Segment;
use itertools::Itertools;
use nannou::{noise::*, prelude::*};

pub struct Surface {
    noise: Perlin,
}

impl Surface {
    pub fn new() -> Surface {
        Surface {
            noise: Perlin::new(),
        }
    }

    fn project_points<'a, 'b>(&'a self, grid: &'b Grid) -> impl Iterator<Item = Point3> + 'a {
        let scl1 = 0.005;
        grid.iter_points().map(move |p| {
            let z = self.noise.get([p.x as f64 * scl1, p.y as f64 * scl1]) as f32;
            p.extend(z)
        })
    }

    fn project_grid_of_points(&self, grid_2d: Vec<Vec<Point2>>) -> Vec<Vec<Point3>> {
        let scl1 = 0.005;
        let mut grid_3d = Vec::new();
        for line_2d in grid_2d.into_iter() {
            let mut line_3d = Vec::new();
            for point_2d in line_2d.into_iter() {
                let z = self
                    .noise
                    .get([point_2d.x as f64 * scl1, point_2d.y as f64 * scl1])
                    as f32;
                let point_3d = point_2d.extend(z);
                line_3d.push(point_3d)
            }
            grid_3d.push(line_3d);
        }
        grid_3d
    }

    fn segments<'a, 'b>(
        &'a self,
        grid: &'b Grid,
        angle: f32,
    ) -> impl Iterator<Item = Segment> + 'a {
        self.project_points(&grid)
            .tuple_windows()
            .filter(|(p0, p1)| p0.y == p1.y)
            .map(move |(p0, p1)| {
                let z_scale = 60.0; // TODO: try not to hardcode this
                let z_coef = angle.sin();
                let y_coef = angle.cos();
                let y0 = p0.y * y_coef + p0.z * z_coef * z_scale;
                let y1 = p1.y * y_coef + p1.z * z_coef * z_scale;
                let q0 = pt2(p0.x, y0);
                let q1 = pt2(p1.x, y1);
                Segment { start: q0, end: q1 }
            })
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

    pub fn draw(&self, grid: &Grid, draw: &Draw, angle: f32) {
        let point_2d_grid = grid.grid_of_points();
        let point_3d_grid = self.project_grid_of_points(point_2d_grid);
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
