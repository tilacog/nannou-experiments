

use nannou::{color::encoding::Srgb, prelude::*};
use nannou_experiments::hexagonal_grid::{pointy_hex_corner, CubeCoord};

const SIZE: f32 = 30.0;
const ARM_RATIO: f32 = 21.0 / 16.0;
const ARM_TIP_RATIO: f32 = 10.0 / 20.0;
const ARM_TIP_ROTATION: f32 = PI / 3.0;
const THICKNESS: f32 = SIZE / 20.0;
const WINDOW_SCALE: u32 = 200;
const WIDTH: u32 = 16 * WINDOW_SCALE;
const HEIGHT: u32 = 9 * WINDOW_SCALE;
const FOREGROUND_COLOR: rgb::Rgb<Srgb, u8> = PLUM;
const BACKGROUND_COLOR: rgb::Rgb<Srgb, u8> = INDIGO;

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
    let draw = app.draw();
    draw.background().color(BACKGROUND_COLOR);
    draw_units(25, &draw);
    draw.to_frame(app, &frame).unwrap();
    app.main_window().capture_frame("/tmp/img.png");
}

fn draw_units(rings: u16, draw: &Draw) {
    let center = CubeCoord::new(0, 0, 0);
    let unit = Unit::new(center.cartesian(SIZE));
    unit.draw(draw);

    for hex in center.spiral(rings) {
        let unit = Unit::new(hex.cartesian(SIZE));
        unit.draw(draw);
    }
}

struct Unit {
    origin: Point2,
}

impl Unit {
    fn new(origin: Point2) -> Self {
        Self { origin }
    }

    fn draw(&self, draw: &Draw) {
        // draw arms
        let draw = draw.translate(self.origin.extend(f32::zero()));
        for corner_ix in 0..3 {
            let corner = pointy_hex_corner(Point2::ZERO, SIZE, corner_ix * 2);
            self.draw_arm(&draw, corner);
        }
    }

    fn draw_arm(&self, draw: &Draw, corner: Point2) {
        let draw = draw.rotate(corner.angle());
        let tip = Point2::ZERO + Point2::new(f32::zero(), SIZE * ARM_RATIO);
        draw.line()
            .start(Point2::ZERO)
            .end(tip)
            .color(FOREGROUND_COLOR)
            .weight(THICKNESS);

        // draw tips
        self.draw_tip(&draw, tip);
        let mirrored = &draw.scale_x(-1.0);
        self.draw_tip(&mirrored, tip)
    }

    fn draw_tip(&self, draw: &Draw, tip: Vec2) {
        let draw = draw.xy(tip).rotate(ARM_TIP_ROTATION);
        let size = SIZE * ARM_TIP_RATIO;
        let end = Point2::new(f32::zero(), -size);
        draw.line()
            .start(Point2::ZERO)
            .end(end)
            .color(FOREGROUND_COLOR)
            .weight(THICKNESS);
    }
}
