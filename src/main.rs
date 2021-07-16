//! https://nb.paulbutler.org/surface-projection/
use nannou::prelude::*;

pub const WIDTH: u32 = 500;
pub const HEIGHT: u32 = 500;
pub const X_OFFSET: f32 = (WIDTH / 2) as f32;
pub const Y_OFFSET: f32 = (HEIGHT / 2) as f32;
const MARGIN: u32 = 50;

mod grid;
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
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());
    let _window = app
        .new_window()
        .size(WIDTH + MARGIN * 2, HEIGHT + MARGIN * 2)
        .view(view)
        .build()
        .expect("failed to build window");

    let grid = Grid::new(15, 50);
    let surface = Surface::new();
    let angle = PI / 2.5;
    Model {
        grid,
        surface,
        angle,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // model.angle = app.mouse.x;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    // model.grid.draw(&draw);

    model.surface.draw(&model.grid, &draw, model.angle);

    // debug angle info
    draw.text(&format!("angle: {} π", model.angle / PI))
        .color(BLACK)
        .y(MARGIN as f32 - 2.0 * HEIGHT as f32 / 3.0);
    draw.to_frame(app, &frame).unwrap();
}
