use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    r1: f32,
    r2: f32,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());
    let _window = app
        .new_window()
        .size(700, 700)
        .view(view)
        .build()
        .expect("failed to build window");

    Model { r1: 12.5, r2: 25.0 }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // model.r1 -= 1.0;
    // model.r2 -= 1.0;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for factor in 1..=12 {
        let f = factor as f32;
        draw.xy(Point2::ZERO).polyline()
            .points_colored_closed(star(model.r1 * f, model.r2 * f));
    }

    draw.to_frame(app, &frame).unwrap();
}

fn star(r1: f32, r2: f32) -> impl Iterator<Item = (Point2, Rgb<u8>)> {
    star_points(r1, r2, 5)
        .map(|p| (p, STEELBLUE))
}

fn star_points(radius1: f32, radius2: f32, npoints: u32) -> impl Iterator<Item = Point2> {
    use std::f32::consts::TAU;

    let step = TAU / npoints as f32;
    let half_step = step / 2.0;

    (0..npoints).flat_map(move |i| {
        let cursor_angle = step * i as f32;

        let outer_x = cursor_angle.sin() * radius2;
        let outer_y = cursor_angle.cos() * radius2;
        let inner_x = (cursor_angle + half_step).sin() * radius1;
        let inner_y = (cursor_angle + half_step).cos() * radius1;

        [vec2(outer_x, outer_y), vec2(inner_x, inner_y)]
    })
}
