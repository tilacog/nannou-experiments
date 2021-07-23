use itertools::Itertools;
use nannou::{noise::*, prelude::*};
use std::iter::successors;

const WIDTH: f32 = 1_000.0;
const HEIGHT: f32 = 1_000.0;
const SCALE: f32 = 5.0;
const HALF_SCALE: f32 = SCALE / 2.0;
const TWO_THIRDS_SCALE: f32 = SCALE * 2.0 / 3.0;
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
        let pre_value = {
            let x = x as f64 * NOISE_SCALE;
            let y = y as f64 * NOISE_SCALE;
            let z = (x * y).abs().sqrt() as f64 * NOISE_SCALE;
            model.noise.get([x, y, z]) as f32
        };
        let value = match (pre_value * 10.0) as u32 % 2 == 0 {
            true => 0.0,
            false => 1.0,
        };

        let color = hsv(0.0, 1.0, value);
        draw.rect()
            .x_y(x + HALF_SCALE, y + HALF_SCALE)
            .w_h(TWO_THIRDS_SCALE, TWO_THIRDS_SCALE)
            .color(color);
    }

    draw.to_frame(app, &frame).unwrap();
}
