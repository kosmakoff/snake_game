mod colors;
mod pickup;
mod snake;
mod snake_sprite;

use std::time::{Duration, Instant};

use graphics::*;
use opengl_graphics::{GlGraphics, Texture};
use piston::input::{Key, RenderArgs, UpdateArgs};

use pickup::Pickup;
use snake::{Direction, NewCell, Snake};
use snake_sprite::SpriteData;

const FRAME_DURATION: Duration = Duration::from_millis(300);

enum GameState {
    Playing,
    GameOver,
}

pub struct Game {
    sprites: SpriteData,
    scale: u32,
    game_size: [u32; 2],
    snake: Snake,
    last_move_instant: Instant,
    cherry_pickup: Pickup,
    score: u32,
    state: GameState,
}

fn draw_border(
    gl: &mut GlGraphics,
    context: &Context,
    width: u32,
    height: u32,
    scale: u32,
    texture: &Texture,
) {
    // draw the outer rectangle
    let image = Image::new();
    for i in 0..width {
        let transform = context.transform.trans_pos([(i * scale * 8) as f64, 0.0]);
        image.draw(texture, &context.draw_state, transform, gl);
        let transform = transform.trans_pos([0.0, ((height - 1) * scale * 8) as f64]);
        image.draw(texture, &context.draw_state, transform, gl);
    }

    for i in 1..height - 1 {
        let transform = context.transform.trans_pos([0.0, (i * scale * 8) as f64]);
        image.draw(texture, &context.draw_state, transform, gl);
        let transform = transform.trans_pos([((width - 1) * scale * 8) as f64, 0.0]);
        image.draw(texture, &context.draw_state, transform, gl);
    }
}

impl Game {
    pub fn new(scale: u32, game_size: [u32; 2]) -> Self {
        let [width, height] = game_size;
        let snake = Snake::new(6, 3);
        let cherry_pickup = Pickup::new_cherry(width - 3, height - 3, &snake.get_occupied_cells())
            .expect("Couldn't generate the first cherry");

        Game {
            sprites: SpriteData::new(scale as u32),
            scale,
            game_size,
            snake,
            cherry_pickup,
            last_move_instant: Instant::now(),
            score: 0,
            state: GameState::Playing,
        }
    }

    pub fn move_snake(&mut self) {}

    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        let viewport = args.viewport();
        let [width, height] = self.game_size;
        let scale = self.scale;

        let all_textures = &self.sprites;
        let brick_texture = &all_textures.brick;

        gl.draw(viewport, |c, gl| {
            clear(colors::BLACK, gl);

            draw_border(gl, &c, width, height, scale, brick_texture);

            self.snake.render(gl, &c, all_textures, scale as u32 * 8);

            self.cherry_pickup
                .render(gl, &c, all_textures, scale as u32 * 8);
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        // move or grow the snake
        if self.last_move_instant.elapsed() >= FRAME_DURATION {
            self.last_move_instant = Instant::now();
            let [max_x, max_y] = self.game_size;
            match self
                .snake
                .advance(max_x - 3, max_y - 3, &self.cherry_pickup)
            {
                Ok(cell) => match cell {
                    NewCell::Pickup => {
                        // recreate the pickup
                        let [width, height] = self.game_size;
                        self.cherry_pickup = Pickup::new_cherry(
                            width - 3,
                            height - 3,
                            &self.snake.get_occupied_cells(),
                        )
                        .expect("Couldn't generate the first cherry");
                    }
                    NewCell::Empty => {
                        // do nothing
                    }
                }, // we are good
                Err(_) => {
                    // we don't care yet about the collision matter
                    println!("We hit something!!!!!")
                }
            }
        }

        // handle the pickup
    }

    pub fn handle_key_press(&mut self, key: &Key) {
        match key {
            Key::Left | Key::A if self.snake.direction() != Direction::Right => {
                self.snake.set_next_direction(Direction::Left)
            }
            Key::Right | Key::D if self.snake.direction() != Direction::Left => {
                self.snake.set_next_direction(Direction::Right)
            }
            Key::Up | Key::W if self.snake.direction() != Direction::Down => {
                self.snake.set_next_direction(Direction::Up)
            }
            Key::Down | Key::S if self.snake.direction() != Direction::Up => {
                self.snake.set_next_direction(Direction::Down)
            }
            _ => (),
        };
    }
}
