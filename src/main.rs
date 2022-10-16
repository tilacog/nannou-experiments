use nannou::prelude::*;
use petgraph::graph::UnGraph;

const RATIO: f32 = 1.4142;
const HEIGHT: f32 = 500.0;
const WIDTH: f32 = HEIGHT * RATIO;
const MARGIN: f32 = HEIGHT / 10.0;
const ROW_LENGTH: usize = 12;
const NUM_POINTS: usize = ROW_LENGTH * 10;
const POINT_DISTANCE: f32 = 50.0;
const GRID_SIZE: f32 = 5.0;

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
    fn new(num_points: usize, row_length: usize, horizontal_spacing: f32) -> Grid {
        // height of an equilateral triangle
        let vertical_spacing = horizontal_spacing * (3.0).sqrt() / 2.0;

        let mut graph = UnGraph::<Point2, ()>::new_undirected();
        let mut indices = vec![];

        // create points and add them to the graph
        for p in 0..num_points {
            // vertical position is determined by division
            let y = (p / row_length) as f32 * vertical_spacing;
            let x_offset = (p / row_length % 2 == 0) as usize as f32 * horizontal_spacing / 2.0;
            // horizontal position is determined by modulo
            let x = (p % row_length) as f32 * horizontal_spacing + x_offset;

            let index = graph.add_node(Point2::new(x, -y));
            indices.push(index);
        }

        // link nodes
        for position in 0..indices.len() {
            let current = &indices[position];
            let is_last_column = (position + 1) % row_length == 0;
            let is_last_row = position + row_length >= num_points;

            // (0, 1): right neighbor
            if !is_last_column {
                let coordinate = position + 1;
                let index = indices[coordinate];
                graph.add_edge(*current, index, ());
            }

            // (1, 0): bottom neighbor
            if !is_last_row {
                let coordinate = position + row_length;
                let index = indices[coordinate];
                graph.add_edge(*current, index, ());
            }

            // (1, 1): bottom right neighbor
            if !is_last_column && !is_last_row {
                let coordinate = position + row_length + 1;
                let index = indices[coordinate];
                graph.add_edge(*current, index, ());
            }
        }

        Grid { points: graph }
    }

    fn draw(&self, draw: &Draw) {
        for point in self.points.raw_nodes() {
            draw.ellipse()
                .color(STEELBLUE)
                .xy(point.weight)
                .w_h(GRID_SIZE, GRID_SIZE);
        }


        {
            // debug a random point and its neighbors
            //
            // There's currently a bug where edges are being made for non-adjacent points. This is
            // most likely because the algorithm was conceived for a square grid and we are using a
            // triangular one.
            let index = petgraph::graph::NodeIndex::new(NUM_POINTS / 2);
            let point = self.points[index];
            draw.ellipse()
                .color(TOMATO)
                .xy(point)
                .w_h(GRID_SIZE * 2.0, GRID_SIZE * 2.0);

            for neighbor in self.points.neighbors(index) {
                let point = self.points[neighbor];
                draw.ellipse()
                    .color(PLUM)
                    .xy(point)
                    .w_h(GRID_SIZE * 2.0, GRID_SIZE * 2.0);
            }
        }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new(NUM_POINTS, ROW_LENGTH, POINT_DISTANCE)
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
