use std::mem::MaybeUninit;

use nannou::{color::encoding::Srgb, prelude::*};

const SIZE: u32 = 1;
const SPACING: f32 = 4.0 / 3.0;
const ARP_TIP_LENGTH: f32 = 2.5;
const ARM_TIP_ROTATION: f32 = PI / 3.0;
const THICKNESS: f32 = SIZE as f32 / 30.0;
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

fn draw_units(rounds: u32, draw: &Draw) {
    let unit = Unit::new(Point2::ZERO);
    draw_recursive(draw, &unit, f32::zero(), rounds);
}

fn draw_recursive(draw: &Draw, unit: &Unit, reserved: f32, steps_left: u32) {
    if steps_left == 0 {
        return;
    }
    // Don't draw or spawn new children if under the reserved circle
    if reserved > 0.0 && unit.origin.length() <= (reserved + SIZE as f32 * 0.1) {
        return;
    }
    unit.draw(draw);
    for child in &unit.spawn() {
        draw_recursive(draw, child, reserved + SIZE as f32, steps_left - 1)
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
        let offset = PI / 2.0; // cool rotation
        let step = TAU / 3.0;
        for turn in 0..3 {
            let rotated_draw = draw.rotate(offset + step * turn as f32);
            self.draw_arm(&rotated_draw);
        }
    }

    fn draw_arm(&self, draw: &Draw) {
        let tip = self.origin + Point2::new(f32::zero(), SIZE as f32);
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
        let end = Point2::new(f32::zero(), SIZE as f32 / -ARP_TIP_LENGTH);
        draw.line()
            .start(Point2::ZERO)
            .end(end)
            .color(FOREGROUND_COLOR)
            .weight(THICKNESS);
    }

    fn spawn(&self) -> [Self; 6] {
        const SIDES: usize = 6;
        let mut children: [MaybeUninit<Self>; SIDES] =
            unsafe { MaybeUninit::uninit().assume_init() };
        let offset = PI / 2.0;
        let step = TAU / SIDES as f32;
        for (turn, slot) in children.iter_mut().enumerate().take(SIDES) {
            let angle = offset + step * turn as f32;
            let origin = find_new_origin(self.origin, angle, SIZE as f32);
            let unit = Unit::new(origin);
            *slot = MaybeUninit::new(unit);
        }
        unsafe { std::mem::transmute::<_, [Self; SIDES]>(children) }
    }
}

fn find_new_origin(origin: Point2, angle: f32, radius: f32) -> Point2 {
    let radius = radius * SPACING;
    let x = radius * angle.cos();
    let y = radius * angle.sin();
    origin + Point2::new(x, y)
}
