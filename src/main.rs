mod bodies;

use bodies::{Attractor, Mover};
use nannou::prelude::*;

pub const WIDTH: u32 = 700;
pub const HEIGHT: u32 = 700;

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    // app.set_loop_mode(LoopMode::rate_fps(30.0));
    let _window = app
        .new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .expect("failed to build window");

    Model {
        attractor: Attractor::new(),
        movers: vec![Mover::new(), Mover::new()],
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.update()
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}

struct Model {
    attractor: Attractor,
    movers: Vec<Mover>,
}

impl Model {
    fn draw(&self, draw: &Draw) {
        self.attractor.draw(&draw);
        for mover in &self.movers {
            mover.draw(&draw);
        }
    }

    fn update(&mut self) {
        // self.attractor.update();
        for mover in self.movers.iter_mut() {
            self.attractor.attract(mover);
        }
        for mover in self.movers.iter_mut() {
            mover.update();
        }
    }
}
