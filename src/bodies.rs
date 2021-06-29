use nannou::{color, geom::Vec2, Draw};

pub struct Attractor {
    pub position: Vec2,
}

impl Attractor {
    pub fn new() -> Self {
        Attractor {
            position: Vec2::new(0.0, 0.0),
        }
    }

    pub fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .color(color::GREENYELLOW)
            .xy(self.position)
            .radius(10.0);
    }
    pub fn update(&mut self) {}
}

pub struct Mover {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}
impl Mover {
    pub fn new() -> Self {
        Mover {
            position: Vec2::new(100.0, 10.0),
            velocity: Vec2::new(0.0, 0.0),
            acceleration: Vec2::new(0.0, 0.0),
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
}
