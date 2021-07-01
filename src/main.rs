use nannou::prelude::*;
mod star;
use star::Star;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    star: Star,
}

fn model(app: &App) -> Model {
    // app.set_loop_mode(LoopMode::loop_once());
    let _window = app
        .new_window()
        .size(700, 700)
        .view(view)
        .build()
        .expect("failed to build window");

    Model { star: Star::new() }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.star.update()
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.star.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}
