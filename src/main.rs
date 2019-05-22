extern crate glutin_window;
extern crate graphics;
extern crate image;
extern crate opengl_graphics;
extern crate piston;

mod game;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use game::Game;

const GAME_WIDTH: u32 = 24;
const GAME_HEIGHT: u32 = 16;
const SPRITE_SIZE: u32 = 8;
const SPRITE_SCALE: u32 = 3;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "Snake Game",
        [
            GAME_WIDTH * SPRITE_SIZE * SPRITE_SCALE,
            GAME_HEIGHT * SPRITE_SIZE * SPRITE_SCALE,
        ],
    )
    .opengl(opengl)
    .exit_on_esc(false)
    .resizable(false)
    .vsync(true)
    .build()
    .unwrap();

    let mut game = Game::new(SPRITE_SCALE, [GAME_WIDTH, GAME_HEIGHT]);
    let mut gl = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {
        game.move_snake();

        if let Some(render_args) = event.render_args() {
            game.render(&mut gl, &render_args);
        }

        if let Some(update_args) = event.update_args() {
            game.update(&update_args);
        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.handle_key_press(&key);
        }
    }
}
