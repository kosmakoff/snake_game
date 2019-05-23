use crate::game::snake_sprite::SpriteData;
use graphics::{Context, Image, Transformed};
use opengl_graphics::{GlGraphics, Texture};
use piston::input::RenderArgs;
use std::collections::LinkedList;

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

enum BodyPartKind {
    Head,
    Middle,
    Tail,
}

type BodyElement = ([u32; 2], BodyPartKind, Direction);

pub struct Snake {
    body: LinkedList<BodyElement>,
    direction: Direction,
    next_direction: Direction,
}

pub enum Collision {
    Body,
    Border,
}

fn get_rotated_texture_variant<'a>(
    textures: &'a [Texture; 4],
    direction: &Direction,
) -> &'a Texture {
    use Direction::*;
    let [right, down, left, up] = textures;

    match direction {
        Right => right,
        Down => down,
        Left => left,
        Up => up,
    }
}

impl Snake {
    pub fn new(x: u32, y: u32) -> Self {
        let mut body = LinkedList::new();
        body.push_back(([x, y], BodyPartKind::Head, Direction::Right));
        body.push_back(([x - 1, y], BodyPartKind::Middle, Direction::Right));
        body.push_back(([x - 2, y], BodyPartKind::Middle, Direction::Right));
        body.push_back(([x - 3, y], BodyPartKind::Middle, Direction::Right));
        body.push_back(([x - 4, y], BodyPartKind::Tail, Direction::Right));

        Snake {
            direction: Direction::Right,
            next_direction: Direction::Right,
            body,
        }
    }

    pub fn render(
        &self,
        gl: &mut GlGraphics,
        context: &Context,
        sprites: &SpriteData,
        sprite_size: u32,
    ) {
        let image = Image::new();

        // offset one cell for border
        let transform = context
            .transform
            .trans_pos([sprite_size as f64, sprite_size as f64]);

        for ([x, y], kind, rotation) in &self.body {
            let transform =
                transform.trans_pos([(x * sprite_size) as f64, (y * sprite_size) as f64]);

            let texture = match kind {
                BodyPartKind::Head => get_rotated_texture_variant(&sprites.snake_head, rotation),
                BodyPartKind::Middle => get_rotated_texture_variant(&sprites.snake_body, rotation),
                BodyPartKind::Tail => get_rotated_texture_variant(&sprites.snake_tail, rotation),
            };
            image.draw(texture, &context.draw_state, transform, gl);
        }
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn get_occupied_cells(&self) -> Vec<(u32, u32)> {
        self.body.iter().map(|([x, y], _, _)| (*x, *y)).collect()
    }

    /// Moves the snake one cell in selected direction.
    ///
    /// # Arguments
    ///
    /// * `max_x` - Maximum value for X coordinate, the boundary of level.
    /// * `max_y` - Maximum value for Y coordinate, the boundary of level.
    pub fn advance(&mut self, max_x: u32, max_y: u32) -> Result<(), Collision> {
        let ([head_x, head_y], _, _) = self.body.front().expect("Body is empty.");
        let (new_head_x, new_head_y) = match self.next_direction {
            Direction::Left => (*head_x as i32 - 1, *head_y as i32),
            Direction::Right => (*head_x as i32 + 1, *head_y as i32),
            Direction::Up => (*head_x as i32, *head_y as i32 - 1),
            Direction::Down => (*head_x as i32, *head_y as i32 + 1),
        };

        // check for borders first
        if new_head_x < 0
            || new_head_x > max_x as i32
            || new_head_y < 0
            || new_head_y > max_y as i32
        {
            return Err(Collision::Border);
        }

        // check for own_body
        if self
            .body
            .iter()
            .rev()
            .skip(1)
            .any(|([part_x, part_y], _, _)| {
                *part_x as i32 == new_head_x && *part_y as i32 == new_head_y
            })
        {
            return Err(Collision::Body);
        }

        self.direction = self.next_direction;

        // change old head kind
        let (_, body_part_kind, _) = self.body.front_mut().expect("Body is empty.");
        *body_part_kind = BodyPartKind::Middle;

        // attach new head
        self.body.push_front((
            [new_head_x as u32, new_head_y as u32],
            BodyPartKind::Head,
            self.direction,
        ));

        // remove tail
        self.body.pop_back();

        // change new back kind

        let ([pre_tail_x, pre_tail_y], _, _) =
            self.body.iter().rev().nth(1).expect("Body is too short.");

        let (pre_tail_x, pre_tail_y) = (*pre_tail_x, *pre_tail_y);

        let ([tail_x, tail_y], tail_part_kind, tail_part_direction) =
            self.body.back_mut().expect("Body is empty.");
        *tail_part_kind = BodyPartKind::Tail;

        *tail_part_direction = match (
            *tail_x as i32 - pre_tail_x as i32,
            *tail_y as i32 - pre_tail_y as i32,
        ) {
            (1, _) => Direction::Left,
            (-1, _) => Direction::Right,
            (_, 1) => Direction::Up,
            (_, -1) => Direction::Down,
            _ => *tail_part_direction,
        };

        Ok(())
    }

    /// Grows the snake in selected direction.
    pub fn grow(&mut self) {}

    pub fn set_next_direction(&mut self, direction: Direction) {
        self.next_direction = direction;
    }
}
