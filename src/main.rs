use nannou::prelude::*;

const SIZE: f32 = 1_000.0;
const FRAMES: u64 = 2_000;

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
    let mouse = normalized_mouse(&app);
    let draw = app.draw().scale_axes(vec3(SIZE, SIZE, 1.0));
    draw.background().color(BLACK);
    let size = 0.4;
    let time = (app.elapsed_frames() as f32 / FRAMES as f32).fract();
    // let depth = (5.0 * mouse.y).floor() as usize;
    let depth = 5.0 * inverse_cos(time * 4.0);
    let sides = 3 + (4.0 * mouse.x).floor() as usize;

    draw_fractal(Vec2::ZERO, size, depth, sides, &draw);
    draw.to_frame(app, &frame).unwrap();
}

fn draw_fractal(position: Vec2, size: f32, depth: f32, sides: usize, draw: &Draw) {
    let increment = 1.0 / sides as f32;
    let scale = 0.5;
    for i in 0..sides {
        let fraction = i as f32 * increment;
        let angle = fraction - 0.25;
        if depth > 0.0 {
            let new_size = size * scale;
            let new_position = position + polar(angle, new_size);
            draw_fractal(new_position, new_size, depth - 1.0, sides, &draw);
        } else {
            let pt1 = position + polar(angle, size);
            let pt2 = position + polar(angle + increment, size);
            draw.line().start(pt1).end(pt2).weight(0.002).color(WHITE);
        }
    }
}

fn polar(angle: f32, radius: f32) -> Point2 {
    let x = (angle * TAU).cos() * radius;
    let y = (angle * TAU).sin() * radius;
    pt2(x, y)
}

fn normalized_mouse(app: &App) -> Vec2 {
    let half = SIZE * 0.5;
    let mouse_x = map_range(app.mouse.x, -half, half, 0.0, 1.0);
    let mouse_y = map_range(app.mouse.y, -half, half, 0.0, 1.0);
    vec2(mouse_x, mouse_y)
}

fn inverse_cos(value: f32) -> f32 {
    1.0 - ((value * TAU).cos() * 0.5 + 0.5)
}
