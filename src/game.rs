mod colors;
mod font;
mod pickup;
mod snake;
mod snake_sprite;

use crate::game::font::draw_string;
use crate::size::*;
use crate::sprite_renderer::{
    GenericContext, SpriteRenderer, SpriteRendererSettings, SubSpriteRenderingContext,
};
use opengl_graphics::OpenGL;
use std::time::{Duration, Instant};

use opengl_graphics::Texture;
use piston::input::*;

use font::Font;
use pickup::Pickup;
use snake::{Direction, NewCell, Snake};
use snake_sprite::SpriteData;

const FRAME_DURATION: Duration = Duration::from_millis(300);

enum GameState {
    Playing(PlayingState),
    GameOver(u32),
}

enum GameFlow {
    StartNew,
    ShowGameOver(u32),
}

pub struct Game {
    sprite_renderer: SpriteRenderer,
    settings: GameSettings,
    sprites: SpriteData,
    state: GameState,
}

pub struct GameSettings {
    opengl: OpenGL,
    sprite_size: u32,
    sprite_scale: u32,
    game_size: Size,
}

impl GameSettings {
    pub fn new<S>(opengl: OpenGL, sprite_size: u32, sprite_scale: u32, game_size: S) -> GameSettings
    where
        S: Into<Size>,
    {
        GameSettings {
            opengl,
            sprite_size,
            sprite_scale,
            game_size: game_size.into(),
        }
    }
}

pub struct PlayingState {
    game_size: Size,
    snake: Snake,
    last_move_instant: Instant,
    cherry_pickup: Pickup,
    score: u32,
}

fn shrink_context<C>(
    context: &mut C,
    shrink_size: (u32, u32, u32, u32),
) -> SubSpriteRenderingContext<&mut C>
where
    C: GenericContext,
{
    let size = context.size();
    let (left, top, bottom, right) = shrink_size;
    context.view_mut(
        (left as i32, top as i32),
        (size.width - left - right, size.height - top - bottom),
    )
}

fn get_border_context<C>(context: &mut C) -> SubSpriteRenderingContext<&mut C>
where
    C: GenericContext,
{
    shrink_context(context, (0, 1, 0, 0))
}

fn get_playing_field_context<C>(context: &mut C) -> SubSpriteRenderingContext<&mut C>
where
    C: GenericContext,
{
    shrink_context(context, (1, 1, 1, 1))
}

fn draw_score<'a, C>(context: &'a mut C, score: u32, font: &Font)
where
    C: GenericContext,
{
    let text = format!("score: {}", score);
    draw_string(context, text, (0, 0), font);
}

fn draw_border<'a, C>(context: &'a mut C, texture: &Texture)
where
    C: GenericContext,
{
    let size = context.size();

    for x in 0..size.width {
        context.draw_sprite((x as i32, 0), texture);
        context.draw_sprite((x as i32, size.height as i32 - 1), texture);
    }

    for y in 1..size.height - 1 {
        context.draw_sprite((0, y), texture);
        context.draw_sprite((size.width - 1, y), texture);
    }
}

fn draw_game_over_screen<'a, C>(context: &'a mut C, score: u32, font: &Font)
where
    C: GenericContext,
{
    draw_string(context, "Game Over".to_uppercase(), (8, 2), font);
    draw_string(
        context,
        format!("You scored {}", score).to_uppercase(),
        (4, 4),
        font,
    );
    draw_string(context, "Press SPACE to restart", (1, 6), font);
}

fn new_cherry_pickup<S>(field_size: S, snake: &Snake) -> Pickup
where
    S: Into<Size>,
{
    Pickup::new_cherry(field_size, &snake.get_occupied_cells())
        .expect("Couldn't generate the first cherry")
}

impl GameState {
    fn new_playing<S: Into<Size>>(game_size: S) -> Self {
        let game_size: Size = game_size.into();
        let field_size = (game_size.width - 2, game_size.height - 3);
        let snake = Snake::new((4, 0), 5, field_size);

        let cherry_pickup = new_cherry_pickup(field_size, &snake);

        GameState::Playing(PlayingState {
            game_size,
            snake,
            cherry_pickup,
            last_move_instant: Instant::now(),
            score: 0,
        })
    }

    fn new_game_over(score: u32) -> Self {
        GameState::GameOver(score)
    }
}

// playing state stuff
fn render_playing(
    sprite_data: &SpriteData,
    sprite_renderer: &mut SpriteRenderer,
    playing_state: &PlayingState,
    args: &RenderArgs,
) {
    let viewport = args.viewport();

    let brick_texture = &sprite_data.brick;
    let score = playing_state.score;

    sprite_renderer.draw(viewport, |context| {
        context.clear(colors::BLACK);

        draw_score(context, score, &sprite_data.font);

        let mut border_context = get_border_context(context);
        draw_border(&mut border_context, brick_texture);

        let mut playing_field_context = get_playing_field_context(&mut border_context);

        playing_state
            .snake
            .render(&mut playing_field_context, &sprite_data);
        playing_state
            .cherry_pickup
            .render(&mut playing_field_context, &sprite_data);
    });
}

fn render_game_over(
    sprite_data: &SpriteData,
    sprite_renderer: &mut SpriteRenderer,
    score: u32,
    args: &RenderArgs,
) {
    let viewport = args.viewport();

    let font = &sprite_data.font;

    sprite_renderer.draw(viewport, |context| {
        context.clear(colors::BLACK);

        draw_game_over_screen(context, score, &font);
    });
}

fn update_playing(playing_state: &mut PlayingState) -> Option<GameFlow> {
    // move or grow the snake
    if playing_state.last_move_instant.elapsed() >= FRAME_DURATION {
        playing_state.last_move_instant = Instant::now();
        match playing_state.snake.advance(&playing_state.cherry_pickup) {
            Ok(cell) => {
                if let NewCell::Pickup = cell {
                    playing_state.score += 1;
                    // recreate the pickup
                    let field_size = (
                        playing_state.game_size.width - 2,
                        playing_state.game_size.height - 3,
                    );
                    playing_state.cherry_pickup =
                        new_cherry_pickup(field_size, &playing_state.snake);
                }
            }
            Err(_) => {
                // smashed the head
                return Some(GameFlow::ShowGameOver(playing_state.score));
            }
        }
    }

    None
}

fn handle_key_press_playing(playing_state: &mut PlayingState, key: &Key) -> Option<GameFlow> {
    match key {
        Key::Left | Key::A if playing_state.snake.direction() != Direction::Right => {
            playing_state.snake.set_next_direction(Direction::Left)
        }
        Key::Right | Key::D if playing_state.snake.direction() != Direction::Left => {
            playing_state.snake.set_next_direction(Direction::Right)
        }
        Key::Up | Key::W if playing_state.snake.direction() != Direction::Down => {
            playing_state.snake.set_next_direction(Direction::Up)
        }
        Key::Down | Key::S if playing_state.snake.direction() != Direction::Up => {
            playing_state.snake.set_next_direction(Direction::Down)
        }
        _ => (),
    }

    None
}

fn handle_key_press_game_over(key: &Key) -> Option<GameFlow> {
    match key {
        Key::Space => Some(GameFlow::StartNew),
        _ => None,
    }
}

impl Game {
    pub fn new(settings: GameSettings) -> Self {
        let sprite_renderer = SpriteRenderer::new(
            settings.opengl,
            SpriteRendererSettings::new(
                settings.game_size,
                settings.sprite_size * settings.sprite_scale,
            ),
        );

        let game_size = settings.game_size;
        let sprite_scale = settings.sprite_scale;

        Game {
            sprite_renderer,
            settings,
            sprites: SpriteData::new(sprite_scale),
            state: GameState::new_playing(game_size),
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        if let Some(render_args) = event.render_args() {
            self.render(&render_args);
        }

        if let Some(_) = event.update_args() {
            self.update().map(|f| self.handle_game_flow(f));
        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            self.handle_key_press(&key)
                .map(|f| self.handle_game_flow(f));
        }
    }

    fn handle_game_flow(&mut self, game_flow: GameFlow) {
        match game_flow {
            GameFlow::StartNew => {
                self.state = GameState::new_playing(self.settings.game_size);
            }
            GameFlow::ShowGameOver(score) => {
                self.state = GameState::new_game_over(score);
            }
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        match &self.state {
            GameState::Playing(playing_state) => {
                render_playing(
                    &self.sprites,
                    &mut self.sprite_renderer,
                    &playing_state,
                    args,
                );
            }
            GameState::GameOver(score) => {
                render_game_over(&self.sprites, &mut self.sprite_renderer, *score, args);
            }
        }
    }

    fn update(&mut self) -> Option<GameFlow> {
        match &mut self.state {
            GameState::Playing(playing_state) => update_playing(playing_state),
            _ => None,
        }
    }

    fn handle_key_press(&mut self, key: &Key) -> Option<GameFlow> {
        match &mut self.state {
            GameState::Playing(playing_state) => handle_key_press_playing(playing_state, key),
            GameState::GameOver(_) => handle_key_press_game_over(key),
        }
    }
}
