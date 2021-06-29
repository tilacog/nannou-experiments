use nannou::{color, geom::Vec2, Draw, rand::random_f32};

const G: f32 = 5.0;

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
        let distance_squared = force.length_squared();
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
}
impl Mover {
    pub fn new() -> Self {
        Mover {
            position: Vec2::new(100.0, 10.0),
            velocity: Vec2::new(random_f32(), random_f32()),
            acceleration: Vec2::new(0.0, 0.0),
            mass: 10.0,
        }
    }

    pub fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .color(color::STEELBLUE)
            .xy(self.position)
            .radius(10.0);
    }

    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration = Vec2::ZERO;
    }

    fn apply_force(&mut self, force: Vec2) {
        let f = force / self.mass;
        self.acceleration += f;
    }
}
