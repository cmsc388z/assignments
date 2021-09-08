extern crate piston_window;
extern crate rand;

mod game;
mod snake;
mod utils;

use piston_window::*;

use game::Game;

const BACK_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

fn main() {
    let (width, height) = (20, 20);

    let mut window: PistonWindow = WindowSettings::new(
        "CMSC388Z Snake Game",
        [
            ((width as f64) * 25.0) as u32,
            ((height as f64) * 25.0) as u32,
        ],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
