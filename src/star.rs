use nannou::prelude::*;

pub struct Star {
    inner_radius: f32,
    outer_radius_factor: f32,
    dark: bool,
}

impl Star {
    pub fn new(inner_radius: f32, outer_radius_factor: f32, dark: bool) -> Star {
        Star {
            inner_radius,
            outer_radius_factor,
            dark,
        }
    }

    pub fn draw(&self, draw: &Draw) {
        let points = star_points(
            self.inner_radius,
            self.inner_radius * self.outer_radius_factor,
        )
        .map(|point| (point, star_color(self.dark)));
        // draw.polyline().points_colored_closed(points);
        draw.polygon().points_colored(points);
    }

    fn shrink(&mut self) {
        let decrement = 0.5;
        let new_radius = (self.inner_radius - decrement).max(0.0);
        self.inner_radius = new_radius
    }
}

fn star_points(radius1: f32, radius2: f32) -> impl Iterator<Item = Point2> {
    let n_points = 5;
    let step = TAU / n_points as f32;
    let half_step = step / 2.0;

    (0..n_points).flat_map(move |i| {
        let cursor_angle = step * i as f32;

        let outer_x = cursor_angle.sin() * radius2;
        let outer_y = cursor_angle.cos() * radius2;
        let inner_x = (cursor_angle + half_step).sin() * radius1;
        let inner_y = (cursor_angle + half_step).cos() * radius1;

        [vec2(outer_x, outer_y), vec2(inner_x, inner_y)]
    })
}

pub struct StarGroup {
    stars: Vec<Star>,
    size: f32,
    num_stars: usize,
    fixed_star: Star,
}

impl StarGroup {
    pub fn new(num_stars: usize, size: f32) -> StarGroup {
        let step = size / num_stars as f32;
        let stars: Vec<Star> = (1..=num_stars)
            .map(|i| {
                let star_inner_radius = i as f32 * step;
                Star::new(star_inner_radius, 2.0, i % 2 == 0)
            })
            .collect();
        StarGroup {
            stars,
            size,
            num_stars,
            fixed_star: Star::new(size, 2.0, true),
        }
    }

    pub fn draw(&self, draw: &Draw) {
        self.fixed_star.draw(&draw);
        self.stars.iter().rev().for_each(|star| star.draw(&draw));
    }

    pub fn update(&mut self) {
        self.stars.iter_mut().for_each(|star| star.shrink());
        self.stars.retain(|star| star.inner_radius > 0.0);
        while self.stars.len() < self.num_stars {
            let mut new_star = Star::new(self.size, 2.0, !self.fixed_star.dark);
            std::mem::swap(&mut self.fixed_star, &mut new_star);
            self.stars.push(new_star)
        }
        assert!(self.stars.len() == self.num_stars);
    }
}

fn star_color(dark: bool) -> Hsv {
    if dark {
        hsv(0.0, 0.0, 0.075)
    } else {
        hsv(0.0, 0.0, 0.0)
    }
}
