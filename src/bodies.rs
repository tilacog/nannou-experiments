use nannou::{color, geom::Vec2, rand::random_f32, Draw};
use std::collections::VecDeque;
const G: f32 = 5.0;
const MOVER_HISTORY: usize = 10_000;

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
}
impl Mover {
    pub fn new() -> Self {
        Mover {
            position: Vec2::new(0.0, 30.0),
            velocity: Vec2::new(2.0, 0.0),
            acceleration: Vec2::new(0.0, 0.0),
            mass: 10.0,
            trace: VecDeque::new(),
        }
    }

    pub fn draw(&self, draw: &Draw) {
        let point_iterator = self.trace.iter().cloned().map(|p| (p, color::PLUM));
        draw.polyline().weight(2.0).points_colored(point_iterator);

        draw.ellipse()
            .color(color::STEELBLUE)
            .xy(self.position)
            .radius(10.0);
    }

    pub fn update(&mut self) {
        // update physics
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration = Vec2::ZERO;

        // update trace
        self.trace.push_front(self.position.clone());
        if self.trace.len() > MOVER_HISTORY {
            self.trace.pop_back();
        }
    }

    fn apply_force(&mut self, force: Vec2) {
        let f = force / self.mass;
        self.acceleration += f;
    }
}
