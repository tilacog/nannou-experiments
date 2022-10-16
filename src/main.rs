use nannou::prelude::*;
use petgraph::graph::UnGraph;

const RATIO: f32 = 1.4142;
const HEIGHT: f32 = 500.0;
const WIDTH: f32 = HEIGHT * RATIO;
const MARGIN: f32 = HEIGHT / 10.0;

// * Model
struct Model {
    grid: Grid,
}

impl Model {
    fn new(app: &App) -> Model {
        app.set_loop_mode(LoopMode::loop_once());
        let _window = app
            .new_window()
            .size(WIDTH as u32, HEIGHT as u32)
            .view(view)
            .build()
            .expect("failed to build window");

        Model {
            grid: Grid::default(),
        }
    }

    fn draw(&self, draw: &Draw) {
        draw.background().color(BLACK);
        self.grid.draw(draw)
    }
}

// * Grid
struct Grid {
    points: UnGraph<Point2, ()>,
}

impl Grid {
    fn new(num_points: usize, row_size: usize, horizontal_spacing: f32) -> Grid {
        // height of an equilateral triangle
        let vertical_spacing = horizontal_spacing * (3.0).sqrt() / 2.0;

        let mut points = UnGraph::<Point2, ()>::new_undirected();
        for p in 0..num_points {
            // vertical position is determined by division
            let y = (p / row_size) as f32 * vertical_spacing;
            let x_offset = (p / row_size % 2 == 0) as usize as f32 * horizontal_spacing / 2.0;
            // horizontal position is determined by modulo
            let x = (p % row_size) as f32 * horizontal_spacing + x_offset;

            points.add_node(Point2::new(x, -y));
        }
        Grid { points }
    }

    fn draw(&self, draw: &Draw) {
        for point in self.points.raw_nodes() {
            draw.ellipse()
                .color(STEELBLUE)
                .xy(point.weight)
                .w(5.0)
                .h(5.0);
        }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new(12 * 10, 12, 50.0)
    }
}

// * core

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app
        .draw()
        .translate(Vec3::new(-WIDTH / 2.0 + MARGIN, HEIGHT / 2.0 - MARGIN, 0.0));

    model.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(Model::new).update(update).run();
}
