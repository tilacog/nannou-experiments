//! https://nb.paulbutler.org/surface-projection/
use itertools::Itertools;

use nannou::noise::*;
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
    surface: Surface,
}

fn model(app: &App) -> Model {
    // app.set_loop_mode(LoopMode::loop_once());
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

    let surface = Surface::new();
    Model { grid, surface }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    // model.grid.draw(&draw);
    let angle = app.mouse.x * 0.01;
    model.surface.draw(&model.grid, &draw, angle);

    // debug angle info
    draw.text(&format!("angle: {}", angle))
        .color(BLACK)
        .x(app.mouse.x)
        .y(app.mouse.y);
    draw.to_frame(app, &frame).unwrap();
}

// ------------------

struct Grid {
    num_lines: u32,
    resolution: u32,
}
impl Grid {
    fn iter_points(&self) -> impl Iterator<Item = Point2> {
        let xs = (0..=self.resolution).into_iter().map(|n| n as f32);
        let ys = (0..=self.num_lines).into_iter().map(|n| n as f32);

        let scale_h: f32 = WIDTH as f32 / self.resolution as f32;
        let scale_v: f32 = HEIGHT as f32 / self.num_lines as f32;

        ys.cartesian_product(xs)
            .map(move |(y, x)| vec2(x * scale_h - X_OFFSET, y * scale_v - Y_OFFSET))
    }

    fn _draw(&self, draw: &Draw) {
        self.iter_points()
            .tuple_windows()
            .filter(|(p0, p1)| p0.y == p1.y)
            .for_each(|(p0, p1)| {
                draw.line().start(p0).end(p1).weight(2.0);
            })
    }
}

struct Surface {
    noise: Perlin,
}

impl Surface {
    fn new() -> Surface {
        Surface {
            noise: Perlin::new(),
        }
    }

    fn project_points<'a, 'b>(&'a self, grid: &'b Grid) -> impl Iterator<Item = Point3> + 'a {
        let scl1 = 0.005;
        grid.iter_points().map(move |p| {
            let z = self.noise.get([p.x as f64 * scl1, p.y as f64 * scl1]) as f32;
            p.extend(z)
        })
    }

    fn draw(&self, grid: &Grid, draw: &Draw, angle: f32) {
        let line_color = hsva(0.0, 0.0, 0.0, 0.75);
        self.project_points(&grid)
            .tuple_windows()
            .filter(|(p0, p1)| p0.y == p1.y)
            .for_each(|(p0, p1)| {
                let z_scale = 60.0; // TODO: try not to hardcode this
                let z_coef = angle.sin();
                let y_coef = angle.cos();

                let y0 = p0.y * y_coef + p0.z * z_coef * z_scale;
                let y1 = p1.y * y_coef + p1.z * z_coef * z_scale;
                let q0 = vec2(p0.x, y0);
                let q1 = vec2(p1.x, y1);
                draw.line().start(q0).end(q1).weight(2.5).color(line_color);
            })
    }
}
