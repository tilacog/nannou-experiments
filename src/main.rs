use nannou::prelude::*;

const SCALE: u32 = 3;
const WIDTH: u32 = 900 * SCALE;
const HEIGHT: u32 = 400 * SCALE;

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());
    let _window = app
        .new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .expect("failed to build window");

    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let draw = draw.xy(pt2(WIDTH as f32 / 3.5, 0.0));

    let gap = (WIDTH / 28) as f32;
    let mut radius = gap * 3.0;

    loop {
        model.draw(&draw, radius, gap);
        if radius > app.window_rect().top_left().distance(Point2::ZERO) * 2.0 {
            break;
        }
        radius += gap * 2.0;
    }
    app.main_window().capture_frame("/tmp/red-circles.png");
    draw.to_frame(app, &frame).unwrap();
}

struct Model {}

impl Model {
    fn draw(&self, draw: &Draw, radius: f32, stroke_weight: f32) {
        let points = self.points(radius).map(|p| (p, CRIMSON));
        draw.polyline()
            .stroke_weight(stroke_weight)
            .points_colored_closed(points);
    }

    fn points(&self, radius: f32) -> impl Iterator<Item = Point2> {
        itertools::unfold(0.0, move |radian| {
            *radian += 0.025;
            if *radian < TAU {
                let x = radian.cos() * radius;
                let y = radian.sin() * radius;
                let p = pt2(x, y);
                Some(p)
            } else {
                None
            }
        })
    }
}
