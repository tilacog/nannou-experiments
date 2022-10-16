use nannou::prelude::*;

const RATIO: f64 = 1.4142;
const SHORT: f64 = 500.0;
const LONG: f64 = SHORT * RATIO;

// * Model
struct Model {
    grid: Grid,
}

impl Model {
    fn new(app: &App) -> Model {
        app.set_loop_mode(LoopMode::loop_once());
        let _window = app
            .new_window()
            .size(LONG as u32, SHORT as u32)
            .view(view)
            .build()
            .expect("failed to build window");

        Model { grid: Grid {} }
    }

    fn draw(&self, draw: &Draw) {
        draw.background().color(BLACK);
        draw.ellipse().color(STEELBLUE);
    }
}

// * Grid
struct Grid {}

// * core

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    model.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(Model::new).update(update).run();
}
