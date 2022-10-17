use nannou::prelude::*;

const RATIO: f32 = 1.4142;
const HEIGHT: f32 = 500.0;
const WIDTH: f32 = HEIGHT * RATIO;
const _MARGIN: f32 = HEIGHT / 10.0;
const POINT_DISTANCE: f32 = 50.0;
const DOT_SIZE: f32 = 5.0;

// * Model
struct Model {}

impl Model {
    fn new(app: &App) -> Model {
        app.set_loop_mode(LoopMode::loop_once());
        let _window = app
            .new_window()
            .size(WIDTH as u32, HEIGHT as u32)
            .view(view)
            .build()
            .expect("failed to build window");

        Model {}
    }

    fn draw(&self, draw: &Draw) {
        draw.background().color(BLACK);
    }
}

// * core

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw(); //.translate(Vec3::new(-WIDTH / 2.0 + MARGIN, HEIGHT / 2.0 - MARGIN, 0.0));

    model.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(Model::new).update(update).run();
}

fn draw_triangle(origin: Point2, radius: f32) -> Vec<Point2> {
    (0..360)
        .step_by(120)
        .map(|angle| {
            let radian = deg_to_rad(angle as f32);
            let x = radian.sin() * radius;
            let y = radian.cos() * radius;
            origin + pt2(x, y)
        })
        .collect()
}
