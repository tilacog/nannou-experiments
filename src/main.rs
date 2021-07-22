use nannou::prelude::*;

const DOT_RADIUS: f32 = 20.0;
const WIDTH: u32 = 1_000;
const HEIGHT: u32 = 1_000;
const MARGIN: (f32, f32) = (-DOT_RADIUS * 2.0, -DOT_RADIUS);
fn main() {
    nannou::app(model).update(update).run();
}

struct Model {}

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

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app
        .draw()
        .rotate(PI / -4.0)
        .xy(app.window_rect().bottom_left() + Vec2::from(MARGIN) * 8.5);

    draw.background().color(WHITE);

    let dots = Dots {
        start_radius: DOT_RADIUS,
        end_radius: 1.0,
        resize_factor: 0.965,
        direction: pt2(0.0, 1.0),
    };

    let dot_diameter = 2 * DOT_RADIUS as u32;
    let two_dots = 2 * dot_diameter;
    let num_dots = (WIDTH - two_dots) / dot_diameter;
    for i in 1..num_dots * 2 {
        let shift_y = if i % 2 == 0 { PI * -0.1 } else { PI * 0.1 };
        let step = vec2(
            i as f32 * dots.start_radius * 2.0,
            shift_y * dots.start_radius * 2.0,
        );
        let d = draw.xy(step);
        dots.draw(&d);
    }

    draw.to_frame(app, &frame).unwrap();
}

struct Dots {
    start_radius: f32,
    end_radius: f32,
    resize_factor: f32,
    direction: Vec2,
}

impl Dots {
    fn draw(&self, draw: &Draw) {
        let mut radius = self.start_radius;
        let mut position = vec2(0.0, 0.0);
        while radius > self.end_radius {
            let draw = draw.xy(position);
            draw.ellipse().w(radius * 2.0).h(radius * 2.0).color(BLACK);
            let step = radius * 2.5;
            position += self.direction.normalize() * step;
            radius *= self.resize_factor;
        }
    }
}
