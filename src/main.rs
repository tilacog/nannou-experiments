use itertools::Itertools;
use nannou::prelude::*;

const MARGIN: u32 = 50;
const NUM_LINES: usize = 15;
const RESOLUTION: usize = 50;
const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const SCALE_H: f32 = WIDTH as f32 / RESOLUTION as f32;
const SCALE_V: f32 = HEIGHT as f32 / NUM_LINES as f32;
const X_OFFSET: f32 = (WIDTH / 2) as f32;
const Y_OFFSET: f32 = (HEIGHT / 2) as f32;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());
    let _window = app
        .new_window()
        .size(WIDTH + MARGIN * 2, HEIGHT + MARGIN * 2)
        .view(view)
        .build()
        .expect("failed to build window");

    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    let xs: Vec<f32> = (0..RESOLUTION).into_iter().map(|n| n as f32).collect();
    let ys: Vec<f32> = (0..NUM_LINES).into_iter().map(|n| n as f32).collect();
    for &y in &ys {
        for (&x0, &x1) in xs.iter().tuple_windows() {
            let p0 = vec2(x0 * SCALE_H - X_OFFSET, y * SCALE_V - Y_OFFSET);
            let p1 = vec2(x1 * SCALE_H - X_OFFSET, y * SCALE_V - Y_OFFSET);
            draw.line().start(p0).end(p1);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

// ------------------
