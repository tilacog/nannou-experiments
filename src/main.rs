use nannou::prelude::*;

const SIZE: f32 = 1_000.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(SIZE as u32, SIZE as u32)
        .view(view)
        .build()
        .expect("failed to build window");

    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw().scale_axes(vec3(SIZE, SIZE, 1.0));
    draw.background().color(BLACK);

    let circle_size = 1.0;
    draw.ellipse()
        .color(WHITE)
        .w_h(circle_size, circle_size)
        .resolution(100.0);

    if app.elapsed_frames() % 60 == 0 {
        dbg!(normalized_mouse(app));
    }

    draw.to_frame(app, &frame).unwrap();
}

fn normalized_mouse(app: &App) -> Vec2 {
    let half = SIZE * 0.5;
    let mouse_x = map_range(app.mouse.x, -half, half, 0.0, 1.0);
    let mouse_y = map_range(app.mouse.y, -half, half, 0.0, 1.0);
    vec2(mouse_x, mouse_y)
}
