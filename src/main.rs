use nannou::prelude::*;
mod star;
use star::StarGroup;

const WIDTH: u32 = 3000;
const HEIGHT: u32 = 3000;
const LEFT: f32 = WIDTH as f32 * -0.5;
const RIGHT: f32 = WIDTH as f32 * 0.5;
const TOP: f32 = HEIGHT as f32 * 0.5;
const BOTTOM: f32 = HEIGHT as f32 * -0.5;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    star_group: StarGroup,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());
    let _window = app
        .new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .expect("failed to build window");

    let mut star_group = StarGroup::new(3, 50.0);
    star_group.random_phase();
    Model { star_group }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.star_group.update()
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let star_radius = 60.0;
    let mut points: Vec<Point2> = vec![];
    let mut attempts = 0;

    while points.len() < 50_000 && attempts < 100_000 {
        let random_x = map_range(random_f32(), 0.0, 1.0, LEFT, RIGHT);
        let random_y = map_range(random_f32(), 0.0, 1.0, BOTTOM, TOP);
        let random_point = pt2(random_x, random_y);
        let mut should_insert = true;
        for point in &points {
            if point.distance(random_point) < star_radius * 1.0 {
                should_insert = false;
                break;
            }
        }
        if should_insert {
            points.push(random_point)
        }
        attempts += 1;
    }
    dbg!(points.len(), attempts);

    for (count, point) in points.into_iter().enumerate() {
        let mut star_group = StarGroup::new(5, star_radius);
        star_group.random_phase();
        star_group.xy(point);
        star_group.rotate(random_f32() * TAU);
        star_group.update();
        star_group.draw(&draw);

        if count % 100 == 0 {
            black_layer(&draw, &app)
        }
    }
    black_layer(&draw, &app);

    draw.to_frame(app, &frame).unwrap();
}

fn black_layer(draw: &Draw, app: &App) {
    draw.rect()
        .xy(app.window_rect().xy())
        .wh(app.window_rect().wh())
        .color(hsla(0.0, 0.0, 0.0, 0.3));
}
