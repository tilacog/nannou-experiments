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
    let size = 0.4;
    draw_fractal(Vec2::ZERO, size, 1, &draw);
    draw.to_frame(app, &frame).unwrap();
}

fn draw_fractal(position: Vec2, size: f32, depth: usize, draw: &Draw) {
    // let draw = draw.translate(starting_point.extend(0.0));
    let sides = 3;
    let increment = 1.0 / sides as f32;

    for i in 0..sides {
        let fraction = i as f32 * increment;
        let angle = fraction;

        if depth > 0 {
            let scale = 0.5;
            let new_size = size * scale;
            let new_position = polar(angle, new_size);
            draw_fractal(new_position, new_size, depth - 1, &draw);
        } else {
            let pt1 = polar(angle, size) + position;
            let pt2 = polar(angle + increment, size) + position;
            draw.line().start(pt1).end(pt2).weight(0.002).color(WHITE);
        }
    }
}

fn polar(angle: f32, radius: f32) -> Point2 {
    let x = (angle * TAU).cos() * radius;
    let y = (angle * TAU).sin() * radius;
    pt2(x, y)
}

fn _normalized_mouse(app: &App) -> Vec2 {
    let half = SIZE * 0.5;
    let mouse_x = map_range(app.mouse.x, -half, half, 0.0, 1.0);
    let mouse_y = map_range(app.mouse.y, -half, half, 0.0, 1.0);
    vec2(mouse_x, mouse_y)
}
