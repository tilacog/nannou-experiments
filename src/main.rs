use nannou::prelude::*;

const WIDTH: f32 = 500.0;
const HEIGHT: f32 = 500.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());
    let _window = app
        .new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .build()
        .expect("failed to build window");

    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw().scale_axes(vec3(WIDTH, HEIGHT, 1.0));
    draw.background().color(BLACK);

    let radius = 0.25;
    let count = 100;
    for i in 0..count {
        let fraction = i as f32 / count as f32;
        let angle = fraction;
        let x = (angle * TAU).cos() * radius;
        let y = (angle * TAU).sin() * radius;
        let circle_radius = 0.01 * Vec2::ONE;
        draw.ellipse().x_y(x, y).wh(circle_radius).resolution(20.0);
    }
    draw.to_frame(app, &frame).unwrap();
}
