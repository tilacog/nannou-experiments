use itertools::Itertools;
use nannou::prelude::*;
mod star;
use star::StarGroup;

const WIDTH: u32 = 700;
const HEIGHT: u32 = 700;
const GRID_LENGTH: u32 = 5;
const SEGMENT: u32 = HEIGHT / GRID_LENGTH;

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
    draw.background().color(CRIMSON);
    draw_grid(&draw);
    // model.star_group.draw(&draw);

    let xs = (0..GRID_LENGTH).into_iter().map(|x| x * SEGMENT);
    let ys = (0..GRID_LENGTH).into_iter().map(|y| y * SEGMENT);
    let d = origin_at_top_left(&draw);
    for (x, y) in xs.cartesian_product(ys) {
        let point = vec2((x + SEGMENT / 2) as f32, (y + SEGMENT / 2) as f32);
        // d.ellipse().xy(dbg!(point)).color(BLACK);
        let mut star_group = StarGroup::new(5, 30.0);
        star_group.random_phase();
        star_group.update();
        star_group.xy(point);
        star_group.draw(&d)
    }
    draw.to_frame(app, &frame).unwrap();
}

fn origin_at_top_left(draw: &Draw) -> Draw {
    draw.x(WIDTH as f32 / -2.0).y(HEIGHT as f32 / -2.0)
}

fn draw_grid(draw: &Draw) {
    let draw = origin_at_top_left(&draw);

    // origin
    // draw.ellipse().w(20.0).h(20.0).color(BLACK);

    let xs = (1..GRID_LENGTH).into_iter().map(|x| x * SEGMENT);
    let ys = (1..GRID_LENGTH).into_iter().map(|y| y * SEGMENT);
    for (x, y) in xs.zip(ys) {
        // draw a cross
        let v1 = vec2(x as f32, 0.0);
        let v2 = vec2(x as f32, HEIGHT as f32);

        let h1 = vec2(0.0, y as f32);
        let h2 = vec2(WIDTH as f32, y as f32);

        draw.line().start(v1).end(v2);
        draw.line().start(h1).end(h2);
    }
}
