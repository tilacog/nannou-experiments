use nannou::prelude::*;
mod star;
use star::StarGroup;

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
        .size(700, 700)
        .view(view)
        .build()
        .expect("failed to build window");

    let mut star_group = StarGroup::new(5, 150.0);
    star_group.random_phase();
    Model { star_group }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.star_group.update()
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(CRIMSON);
    model.star_group.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}
