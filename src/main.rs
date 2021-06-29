mod bodies;

use bodies::{Attractor, Mover};
use nannou::prelude::*;
use std::collections::VecDeque;
const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;

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
        mover: Mover::new(),
        trace: VecDeque::new(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.update()
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // if app.elapsed_frames() <= 1 {
    draw.background().color(BLACK);
    // }
    // draw.rect()
    //     .w_h(WIDTH as f32, HEIGHT as f32)
    //     .color(srgba(0.0, 0.0, 0.0, 0.1));

    model.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}

struct Model {
    attractor: Attractor,
    mover: Mover,
    trace: VecDeque<Vec2>,
}

impl Model {
    fn draw(&self, draw: &Draw) {
        self.attractor.draw(&draw);
        self.mover.draw(&draw);

        let point_iterator = self.trace.iter().cloned().map(|p| (p, PLUM));
        draw.polyline().weight(2.0).points_colored(point_iterator);
    }

    fn update(&mut self) {
        // self.attractor.update();
        self.attractor.attract(&mut self.mover);
        self.mover.update();

        self.trace.push_front(self.mover.position.clone());
        if self.trace.len() > 10_00 {
            self.trace.pop_back();
        }
    }
}
