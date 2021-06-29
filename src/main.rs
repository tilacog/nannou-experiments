mod bodies;

use bodies::{Attractor, Mover};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(500, 500)
        .view(view)
        .build()
        .expect("failed to build window");

    Model {
        attractor: Attractor::new(),
        mover: Mover::new(),
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
    mover: Mover,
}

impl Model {
    fn draw(&self, draw: &Draw) {
        self.attractor.draw(&draw);
        self.mover.draw(&draw);
    }

    fn update(&mut self) {
        // self.attractor.update();
        self.attractor.attract(&mut self.mover);
        self.mover.update();
    }
}
