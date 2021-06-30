use super::{HEIGHT, WIDTH};
use nannou::{color, geom::Vec2, rand::random_f32, Draw};
use std::collections::VecDeque;
const G: f32 = 10.0;
const MOVER_HISTORY: usize = 10_000;

const MAX_SPEED: f32 = 10.0;

pub struct Attractor {
    pub position: Vec2,
    pub mass: f32,
}

impl Attractor {
    pub fn new() -> Self {
        Attractor {
            position: Vec2::new(0.0, 0.0),
            mass: 10.0,
        }
    }

    pub fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .color(color::GREENYELLOW)
            .xy(self.position)
            .radius(10.0);
    }

    #[allow(dead_code)]
    pub fn update(&mut self) {}

    pub fn attract(&self, mover: &mut Mover) {
        let force = self.position - mover.position;
        let distance_squared = force.length_squared().clamp(1.0, 10_000.0);
        let strength = (G * (self.mass * mover.mass)) / distance_squared;
        let force_magnutide = force.normalize() * strength; // set magnitude
        mover.apply_force(force_magnutide);
    }
}

pub struct Mover {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
    trace: VecDeque<Vec2>,
    color: color::Hsla,
    trace_color: color::Hsla,
}

impl Mover {
    pub fn new() -> Self {
        MoverBuilder::new().build()
    }

    pub fn draw(&self, draw: &Draw) {
        let point_iterator = self.trace.iter().cloned().map(|p| (p, self.trace_color));
        draw.polyline()
            .weight(2.0 * self.mass / 10.0)
            .points_colored(point_iterator);

        draw.ellipse()
            .color(self.color)
            .xy(self.position)
            .radius(self.mass / 2.0);
    }

    pub fn update(&mut self) {
        // update physics
        self.velocity += self.acceleration;

        self.velocity = self.velocity.clamp(
            Vec2::new(-MAX_SPEED, -MAX_SPEED),
            Vec2::new(MAX_SPEED, MAX_SPEED),
        );

        self.position += self.velocity;
        self.acceleration = Vec2::ZERO;

        // update trace
        self.trace.push_front(self.position.clone());
        if self.trace.len() > MOVER_HISTORY {
            self.trace.pop_back();
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        let f = force / self.mass;
        self.acceleration += f;
    }
}

fn random_point() -> Vec2 {
    Vec2::new(random_f32(), random_f32())
}

fn random_color() -> color::Hsla {
    let random_hue = random_f32() * 360.0;
    color::Hsla::new(random_hue, 0.5, 0.5, 1.0)
}

#[derive(Default)]
pub struct MoverBuilder {
    position: Option<Vec2>,
    velocity: Option<Vec2>,
    color: Option<color::Hsla>,
    mass: Option<f32>,
}

impl MoverBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(self) -> Mover {
        let random_x = (WIDTH / 2) as f32 * random_f32();
        let random_y = (HEIGHT / 2) as f32 * random_f32();
        let position = self.position.unwrap_or(Vec2::new(random_x, random_y));
        let velocity = self.velocity.unwrap_or(random_point().normalize());
        let mass = self.mass.unwrap_or((10.0 * random_f32()).clamp(3.0, 10.0));
        let color = self.color.unwrap_or(random_color());
        let mut trace_color = color.clone();
        trace_color.lightness = trace_color.lightness * 0.1;
        Mover {
            position,
            velocity,
            acceleration: Vec2::ZERO,
            mass,
            trace: VecDeque::new(),
            color,
            trace_color,
        }
    }

    pub fn position(mut self, position: Vec2) -> Self {
        self.position = Some(position);
        self
    }
    pub fn velocity(mut self, velocity: Vec2) -> Self {
        self.velocity = Some(velocity);
        self
    }
    pub fn mass(mut self, mass: f32) -> Self {
        self.mass = Some(mass);
        self
    }
    pub fn color(mut self, color: color::Hsla) -> Self {
        self.color = Some(color);
        self
    }
}
