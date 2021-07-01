use nannou::prelude::*;

pub struct Star {
    inner_radius: f32,
    outer_radius_factor: f32,
}

impl Star {
    pub fn new() -> Star {
        Star {
            inner_radius: 100.0,
            outer_radius_factor: 2.0,
        }
    }

    pub fn draw(&self, draw: &Draw) {
        draw.polyline().points_colored_closed(star_points(
            self.inner_radius,
            self.inner_radius * self.outer_radius_factor,
        ));
    }

    pub fn update(&mut self) {
        self.shrink(0.1);
    }

    fn shrink(&mut self, amount: f32) {
        let new_radius = (self.inner_radius - amount.abs()).max(0.0);
        self.inner_radius = new_radius
    }
}

fn star_points(r1: f32, r2: f32) -> impl Iterator<Item = (Point2, Rgb<u8>)> {
    create_star_points(r1, r2, 5).map(|p| (p, STEELBLUE))
}

fn create_star_points(radius1: f32, radius2: f32, npoints: u32) -> impl Iterator<Item = Point2> {
    let step = TAU / npoints as f32;
    let half_step = step / 2.0;

    (0..npoints).flat_map(move |i| {
        let cursor_angle = step * i as f32;

        let outer_x = cursor_angle.sin() * radius2;
        let outer_y = cursor_angle.cos() * radius2;
        let inner_x = (cursor_angle + half_step).sin() * radius1;
        let inner_y = (cursor_angle + half_step).cos() * radius1;

        [vec2(outer_x, outer_y), vec2(inner_x, inner_y)]
    })
}