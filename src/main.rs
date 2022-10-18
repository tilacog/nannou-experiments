use nannou::prelude::*;
use rand::Rng;
use std::{
    collections::{HashSet, VecDeque},
    iter::successors,
};

const RATIO: f32 = 1.4142;
const HEIGHT: f32 = 500.0;
const WIDTH: f32 = HEIGHT * RATIO;
const _MARGIN: f32 = HEIGHT / 10.0;

const DOT_COLOR: Rgb<u8> = PLUM;
const DOT_SIZE: f32 = 1.0;
const LINE_COLOR: Rgb<u8> = STEELBLUE;
const LINE_WEIGHT: f32 = 1.0;

const SQRT_3: f32 = 1.73205080757;
const SIDE_LENGTH: f32 = 80.0;
const CIRCUMRADIUS: f32 = SIDE_LENGTH * SQRT_3 / 3.0;
const INRADIUS: f32 = CIRCUMRADIUS / 2.0;

// * Model
struct Model {
    triangle: Triangle,
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
            triangle: Triangle::new(Point2::new(0.0, 0.0), INRADIUS, CIRCUMRADIUS, 0.0),
        }
    }

    fn draw(&self, draw: &Draw) {
        draw.background().color(BLACK);
        self.draw_triangles(draw);
    }

    fn draw_triangles(&self, draw: &Draw) {
        let mut triangles_left = 10;
        let mut queue: VecDeque<Triangle> = VecDeque::new();
        queue.push_back(self.triangle.clone()); // initialize queue
        let mut rendered: Vec<Point2> = Vec::new();

        while let Some(triangle) = queue.pop_front() {
            triangle.draw(draw);
            rendered.push(triangle.origin);

            triangles_left -= 1;
            if triangles_left == 0 {
                break;
            }

            // Don't enqueue new triangles queue if they are close (the same) as previous triangles
            'outer: for new_triangle in triangle.project() {
                for existing in &rendered {
                    if existing.distance(new_triangle.origin) < 1.0 {
                        continue 'outer;
                    }
                }
                // randomize enqueing of new triangles
                if rand::thread_rng().gen_bool(0.6) {
                    queue.push_back(new_triangle);
                }
            }
        }
    }
}

// * agent
#[derive(Clone)]
struct Triangle {
    origin: Point2,
    inradius: f32,
    circumradius: f32,
    orientation: f32,
}

impl std::fmt::Debug for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Triangle")
            .field("origin", &self.origin)
            .finish()
    }
}

impl Triangle {
    fn new(origin: Point2, inradius: f32, circumradius: f32, orientation: f32) -> Self {
        Self {
            origin,
            inradius,
            circumradius,
            orientation,
        }
    }

    fn draw(&self, draw: &Draw) {
        self.draw_triangle(draw);
    }

    fn stroke(a: Point2, b: Point2, color: Rgb<u8>, draw: &Draw) {
        draw.line().start(a).end(b).weight(LINE_WEIGHT).color(color);
    }

    fn triangle_points(origin: Point2, orientation: f32, radius: f32) -> Vec<Point2> {
        successors(Some(orientation), |radian| Some(radian + TAU / 3.0))
            .take(3)
            .map(|radian| {
                let x = radian.sin() * radius;
                let y = radian.cos() * radius;
                origin + pt2(x, y)
            })
            .collect()
    }
    fn draw_triangle(&self, draw: &Draw) {
        let points = Self::triangle_points(self.origin, self.orientation, self.circumradius);

        // draw sides
        Self::stroke(points[0], points[1], LINE_COLOR, &draw);
        Self::stroke(points[1], points[2], LINE_COLOR, &draw);
        Self::stroke(points[2], points[0], LINE_COLOR, &draw);

        // draw vertices
        for point in &points {
            draw.ellipse()
                .color(DOT_COLOR)
                .w_h(DOT_SIZE, DOT_SIZE)
                .xy(*point);
        }
    }
    fn project(&self) -> impl Iterator<Item = Self> + '_ {
        const ONE_THIRD_ROTATION: f32 = TAU / 3.0;
        const ONE_SIXTH__ROTATION: f32 = TAU / 6.0;
        // rotate by 30 degrees and then take three one-third turns...
        successors(Some(self.orientation + ONE_SIXTH__ROTATION), |radians| {
            Some(radians + ONE_THIRD_ROTATION)
        })
        .take(3)
        // for each turn, create a new triangle
        .map(|rotation| {
            let orientation = rotation;
            let radius = self.inradius * 2.0;
            let x = orientation.sin() * radius;
            let y = orientation.cos() * radius;
            let origin = self.origin + pt2(x, y);
            Triangle::new(origin, self.inradius, self.circumradius, orientation)
        })
    }
}

// * core

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw(); //.translate(Vec3::new(-WIDTH / 2.0 + MARGIN, HEIGHT / 2.0 - MARGIN, 0.0));
    model.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(Model::new).update(update).run();
}
