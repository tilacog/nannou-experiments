mod grid;

use grid::Grid;
use nannou::prelude::*;

const WIDTH: f32 = 500.0;
const HEIGHT: f32 = 500.0;
const SPACING: f32 = 20.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    grid: Grid,
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
        grid: Grid::new(WIDTH, HEIGHT, SPACING),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model.grid.draw(&draw);

    let first = model.grid.iter().next().unwrap();
    first.draw(&draw, Some(STEELBLUE));

    let last = model.grid.iter().last().unwrap();
    last.draw(&draw, None);

    draw.to_frame(app, &frame).unwrap();
}
