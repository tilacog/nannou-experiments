use itertools::Itertools;
use nannou::{noise::*, prelude::*};
use std::iter::successors;

const WIDTH: f32 = 1_000.0;
const HEIGHT: f32 = 1_000.0;
const SCALE: f32 = 1.25;
const HALF_SCALE: f32 = SCALE / 2.0;
const NOISE_SCALE: f64 = 0.01;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    noise: OpenSimplex,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());
    let _window = app
        .new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .build()
        .expect("failed to build window");

    Model {
        noise: OpenSimplex::new(),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let left = app.window_rect().left();
    let right = app.window_rect().right();
    let top = app.window_rect().top();
    let bottom = app.window_rect().bottom();

    let xs = successors(Some(left), |n| {
        let next = n + SCALE;
        if next < right {
            Some(next)
        } else {
            None
        }
    });

    let ys = successors(Some(bottom), |n| {
        let next = n + SCALE;
        if next < top {
            Some(next)
        } else {
            None
        }
    });

    for (x, y) in xs.cartesian_product(ys) {
        let noise = {
            let x = x as f64 * NOISE_SCALE;
            let y = y as f64 * NOISE_SCALE;
            let z = (x * x + y * y).sqrt() as f64 * NOISE_SCALE;
            model.noise.get([x, y, z]) as f32
        };

        let hue = {
            let hue: f32 = map_range(noise, -0.5, 0.5, 0.0, 1.0);
            hue.clamp(0.0, 1.0)
        };

        let value = if in_range(hue) { 1.0 } else { 0.0 };

        let color = hsv(hue.clamp(0.0, 1.0), 1.0, value);
        draw.rect()
            .x_y(x + HALF_SCALE, y + HALF_SCALE)
            .w_h(SCALE, SCALE)
            .color(color);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn in_range(n: f32) -> bool {
    if n < 0.1 {
        true
    } else if n > 0.1 && n < 0.2 {
        false
    } else if n > 0.2 && n < 0.3 {
        true
    } else if n > 0.3 && n < 0.4 {
        false
    } else if n > 0.4 && n < 0.5 {
        true
    } else if n > 0.5 && n < 0.6 {
        false
    } else if n > 0.7 && n < 0.8 {
        true
    } else if n > 0.8 && n < 0.9 {
        false
    } else if n > 0.9 && n < 1.0 {
        true
    } else {
        false
    }
}
