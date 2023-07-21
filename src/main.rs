use std::char::TryFromCharError;

use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Verlet Integration")
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        update(&mut rl, &thread);
    }
}

fn update(rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = rl.begin_drawing(thread);

    d.clear_background(Color::GRAY);
    d.draw_circle(320, 240, 25.0, Color::BLACK);
}