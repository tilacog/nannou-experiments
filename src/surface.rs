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

    pub fn draw(&self, grid: &Grid, draw: &Draw, angle: f32) {
        self.segments(grid, angle)
            .for_each(|segment| segment.draw(&draw, &grid, angle))
    }
}
