use nannou::color::Lab;
use nannou::{color::encoding::Srgb, prelude::*};
use nannou_experiments::hexagonal_grid::{pointy_hex_corner, CubeCoord};

const SIZE: f32 = 50.0;
const ARM_RATIO: f32 = 21.0 / 16.0;
const ARM_TIP_RATIO: f32 = 10.0 / 20.0;
// const ARM_TIP_RATIO: f32 = std::f32::consts::E / PI; // nice pattern too
const ARM_TIP_ROTATION: f32 = PI / 3.0;
const THICKNESS: f32 = SIZE / 15.0;
const WINDOW_SCALE: u32 = 100;
const WIDTH: u32 = 16 * WINDOW_SCALE;
const HEIGHT: u32 = 9 * WINDOW_SCALE;
const BACKGROUND_COLOR: rgb::Rgb<Srgb, u8> = INDIGO;
const RINGS: u16 = 50;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    mouse: Point2,
    frame: Rect,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .expect("failed to build window");

    Model {
        mouse: app.mouse.position(),
        frame: app.window_rect(),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.mouse = app.mouse.position();
    model.frame = app.window_rect();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let scale = map_range(
        model.mouse.x,
        model.frame.left(),
        model.frame.right(),
        0.01,
        1.0,
    );
    let draw = app.draw().scale(scale);
    draw.background().color(BACKGROUND_COLOR);
    draw_units(RINGS, &draw, model);
    draw.to_frame(app, &frame).unwrap();
    // app.main_window().capture_frame("/tmp/img.png"); // uncomment to capture the frame
}

fn draw_units(rings: u16, draw: &Draw, model: &Model) {
    let center = CubeCoord::new(0, 0, 0);
    let unit = Unit::new(center, model);
    unit.draw(draw);

    for hex in center.spiral(rings) {
        let unit = Unit::new(hex, model);
        unit.draw(draw);
    }
}

struct Unit<'a> {
    coord: CubeCoord,
    model: &'a Model,
}

impl<'a> Unit<'a> {
    fn new(coord: CubeCoord, model: &'a Model) -> Self {
        Self { coord, model }
    }

    fn origin(&self) -> Point2 {
        self.coord.cartesian(SIZE)
    }

    fn color(&self) -> Lab {
        let (q, r, s) = self.coord.into_parts();
        let scale = map_range(
            self.model.mouse.y,
            self.model.frame.top(),
            self.model.frame.bottom(),
            0,
            RINGS as i32,
        );

        let l = map_range(s, scale, -scale, 0.0, 100.0);
        let a = map_range(q, scale, -scale, -128.0, 127.0);
        let b = map_range(r, -scale, scale, -128.0, 127.0);

        Lab::new(l, a, b)
    }

    fn draw(&self, draw: &Draw) {
        // draw arms
        let draw = draw.translate(self.origin().extend(f32::zero()));
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
            .color(self.color())
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
            .color(self.color())
            .weight(THICKNESS);
    }
}
