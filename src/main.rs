//! https://nb.paulbutler.org/surface-projection/
use nannou::prelude::*;

pub const WIDTH: u32 = 3000;
pub const HEIGHT: u32 = 2000;
pub const X_OFFSET: f32 = (WIDTH / 2) as f32;
pub const Y_OFFSET: f32 = (HEIGHT / 2) as f32;
const MARGIN: u32 = 50;

mod grid;
mod segment;
mod surface;

use grid::Grid;
use surface::Surface;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    grid: Grid,
    surface: Surface,
    angle: f32,
    offset: Vec2,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());
    let _window = app
        .new_window()
        .size(WIDTH + MARGIN * 2, HEIGHT + MARGIN * 2)
        .view(view)
        .build()
        .expect("failed to build window");

    let grid = Grid::new(100, 1000);
    let surface = Surface::new();
    let angle = PI / 2.4;
    Model {
        grid,
        surface,
        angle,
        offset: Vec2::ZERO,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model
        .surface
        .draw(&model.grid, &draw, model.angle, model.offset);

    draw.to_frame(app, &frame).unwrap();
}

fn vinny_map(n: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
    (n - start1) / (stop1 - start1) * (stop2 - start2) + start2
}
