//! https://nb.paulbutler.org/surface-projection/
use itertools::Itertools;
use nannou::prelude::*;

const MARGIN: u32 = 50;
const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const X_OFFSET: f32 = (WIDTH / 2) as f32;
const Y_OFFSET: f32 = (HEIGHT / 2) as f32;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    grid: Grid,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());
    let _window = app
        .new_window()
        .size(WIDTH + MARGIN * 2, HEIGHT + MARGIN * 2)
        .view(view)
        .build()
        .expect("failed to build window");

    let grid = Grid {
        num_lines: 15,
        resolution: 50,
    };

    Model { grid }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    model.grid.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}

// ------------------

struct Grid {
    num_lines: u32,
    resolution: u32,
}

impl Grid {
    fn iter_point_pairs(&self) -> impl Iterator<Item = (Point2, Point2)> {
        let xs = (0..self.resolution).into_iter().map(|n| n as f32);
        let ys = (0..self.num_lines).into_iter().map(|n| n as f32);

        let scale_h: f32 = WIDTH as f32 / self.resolution as f32;
        let scale_v: f32 = HEIGHT as f32 / self.num_lines as f32;

        ys.cartesian_product(xs.tuple_windows::<(_, _)>())
            .map(move |(y, (x0, x1))| {
                let p0 = vec2(x0 * scale_h - X_OFFSET, y * scale_v - Y_OFFSET);
                let p1 = vec2(x1 * scale_h - X_OFFSET, y * scale_v - Y_OFFSET);
                (p0, p1)
            })
    }

    fn draw(&self, draw: &Draw) {
        for (p0, p1) in self.iter_point_pairs() {
            draw.line().start(p0).end(p1);
        }
    }
}
