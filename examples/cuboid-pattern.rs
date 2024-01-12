use nannou::{color::encoding::Srgb, prelude::*};
use nannou_experiments::hexagonal_grid::{pointy_hex_corner, CubeCoord};

const SIZE: f32 = 1.0;
const ARM_TIP_LENGTH: f32 = SIZE / 2.5;
const ARM_TIP_ROTATION: f32 = PI / 3.0;
const THICKNESS: f32 = SIZE / 30.0;
const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
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
    let draw = app.draw().scale(WIDTH as f32 / 5.0);
    draw.background().color(BACKGROUND_COLOR);
    draw_units(10, &draw);
    draw.to_frame(app, &frame).unwrap();
}

fn draw_units(rings: u16, draw: &Draw) {
    let center = CubeCoord::new(0, 0, 0);
    let unit = Unit::new(center.cartesian(SIZE));
    unit.draw(draw);
    // for hex in center.spiral(rings) {
    //     let unit = Unit::new(hex.cartesian(SIZE));
    //     unit.draw(draw);
    // }
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

        // let offset = PI / 2.0; // cool rotation
        // let step = TAU / 3.0;
        // for turn in 0..3 {
        //     let rotated_draw = draw.rotate(offset + step * turn as f32);
        //     self.draw_arm(&rotated_draw);
        // }

        for corner_ix in 0..3 {
            let corner = pointy_hex_corner(self.origin, SIZE, corner_ix * 2);
            let angle = corner.angle();
            let rotated_draw = draw.rotate(angle);
            self.draw_arm(&rotated_draw);
        }
    }

    fn draw_arm(&self, draw: &Draw) {
        let tip = self.origin + Point2::new(f32::zero(), SIZE);
        draw.line()
            .start(self.origin)
            .end(tip)
            .color(FOREGROUND_COLOR)
            .weight(THICKNESS);

        // draw tips
        self.draw_tip(draw, tip);
        let mirrored = draw.scale_x(-1.0);
        self.draw_tip(&mirrored, tip)
    }

    fn draw_tip(&self, draw: &Draw, tip: Vec2) {
        let draw = draw.xy(tip).rotate(ARM_TIP_ROTATION);
        let end = Point2::new(f32::zero(), -ARM_TIP_LENGTH);
        draw.line()
            .start(Point2::ZERO)
            .end(end)
            .color(FOREGROUND_COLOR)
            .weight(THICKNESS);
    }
}
