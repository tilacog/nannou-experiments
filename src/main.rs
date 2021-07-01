use nannou::noise::*;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    r1: f32,
    r2: f32,
    noise: Perlin,
}

fn model(app: &App) -> Model {
    // app.set_loop_mode(LoopMode::loop_once());
    let _window = app
        .new_window()
        .size(700, 700)
        .view(view)
        .build()
        .expect("failed to build window");

    Model {
        r1: 12.5,
        r2: 25.0,
        noise: Perlin::new(),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for factor in 1..=12 {
        let f = factor as f32;
        let iter = star(model.r1 * f, model.r2 * f).map(|(p, c)| {
            let t = app.elapsed_frames() as f64 * 0.01;
            let noise = vec2(
                model.noise.get([p.x as f64, p.y as f64, 0.0 + t]) as f32,
                model.noise.get([p.x as f64, p.y as f64, 1.0 + t]) as f32,
            );
            (p + noise * factor.min(7) as f32, c)
        });

        draw.polyline().points_colored_closed(iter);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn star(r1: f32, r2: f32) -> impl Iterator<Item = (Point2, Rgb<u8>)> {
    star_points(Point2::ZERO, r1, r2, 5)
        .into_iter()
        .map(|p| (p, STEELBLUE))
}

fn star_points(center: Point2, radius1: f32, radius2: f32, npoints: usize) -> Vec<Point2> {
    let step = TAU / npoints as f32;
    let half_step = step / 2.0;
    let mut points = vec![];
    let mut cursor_angle = 0.0;
    while cursor_angle < TAU {
        let outer_x = center.x + cursor_angle.sin() * radius2;
        let outer_y = center.y + cursor_angle.cos() * radius2;
        let inner_x = center.x + (cursor_angle + half_step).sin() * radius1;
        let inner_y = center.x + (cursor_angle + half_step).cos() * radius1;
        points.push(vec2(outer_x, outer_y));
        points.push(vec2(inner_x, inner_y));
        cursor_angle += step;
    }
    points
}
