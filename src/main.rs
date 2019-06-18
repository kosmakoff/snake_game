extern crate glutin_window;
extern crate graphics;
extern crate image;
extern crate opengl_graphics;
extern crate piston;

#[macro_use]
mod conv_macros;
mod game;
mod point;
mod size;
mod sprite_renderer;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::*;
use piston::window::WindowSettings;

use game::{Game, GameSettings};

const GAME_WIDTH: u32 = 24;
const GAME_HEIGHT: u32 = 16;
const SPRITE_SIZE: u32 = 8;
const SPRITE_SCALE: u32 = 3;

fn main() {
    let opengl = OpenGL::V2_0;

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

    let opengl = OpenGL::V2_1;

    let settings = GameSettings::new(opengl, SPRITE_SIZE, SPRITE_SCALE, (GAME_WIDTH, GAME_HEIGHT));
    let mut game = Game::new(settings);

    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {
        game.handle_event(&event);
    }
}
