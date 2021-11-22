use nannou::prelude::*;

mod ball;
use crate::ball::Ball;

fn main() {
    nannou::app(model)
        .size(1920, 1080)
        .update(update)
        .run();
}

struct Model {
    _window: window::Id,
    ball_list: [Ball; 50],
}

fn model(app: &App) -> Model {
    let _window = app.
                            new_window()
                            .title("Bouncing Ball")
                            .key_pressed(key_pressed)
                            .view(view)
                            .build()
                            .unwrap();
    let win = app.window_rect().pad(30.0);

    let mut ball_list = [Ball::new(win.x(), win.y(), 100.0, 100.0, -500.0, 30.0, BLACK); 50];
    for i in 0..50 {
        ball_list[i] = Ball::new(win.x(), win.y(), 300.0 + (i as f32 * 10.0), 300.0 + (i as f32 * 5.0), -500.0, 30.0, BLACK)
    }
 
    
    Model { _window, ball_list }
}

fn key_pressed(_app: &App, _model: &mut Model, _key: Key) {
    //Balls
    for i in 0..50 {
        _model.ball_list[i].key_logic(_key);
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    //Balls
    for i in 0..50 {
        _model.ball_list[i].logic(_app, _update)
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    //Balls
    for i in 0..50 {
        _model.ball_list[i].display(&draw)
    }

    draw.to_frame(app, &frame).unwrap();
}