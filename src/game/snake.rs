use crate::game::pickup::Pickup;
use crate::game::snake_sprite::SpriteData;
use crate::point::Point;
use crate::size::Size;
use crate::sprite_renderer::GenericContext;
use opengl_graphics::Texture;
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

type BodyElement = (Point, BodyPartKind, Direction);

pub struct Snake {
    body: LinkedList<BodyElement>,
    direction: Direction,
    next_direction: Direction,
    field_size: Size,
}

pub enum NewCell {
    Empty,
    Pickup,
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
    pub fn new<P: Into<Point>, S: Into<Size>>(head_start: P, length: u32, field_size: S) -> Self {
        let head_start: Point = head_start.into();

        let mut body = LinkedList::new();
        body.push_back((head_start, BodyPartKind::Head, Direction::Right));

        for i in 1..length - 1 {
            body.push_back((
                head_start.offset((-(i as i32), 0)),
                BodyPartKind::Middle,
                Direction::Right,
            ));
        }

        body.push_back((
            head_start.offset((1 - length as i32, 0)),
            BodyPartKind::Tail,
            Direction::Right,
        ));

        Snake {
            direction: Direction::Right,
            next_direction: Direction::Right,
            body,
            field_size: field_size.into(),
        }
    }

    pub fn render<C>(&self, context: &mut C, sprites: &SpriteData)
    where
        C: GenericContext,
    {
        for (point, kind, rotation) in &self.body {
            let texture = match kind {
                BodyPartKind::Head => get_rotated_texture_variant(&sprites.snake_head, rotation),
                BodyPartKind::Middle => get_rotated_texture_variant(&sprites.snake_body, rotation),
                BodyPartKind::Tail => get_rotated_texture_variant(&sprites.snake_tail, rotation),
            };

            context.draw_sprite(*point, texture);
        }
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn get_occupied_cells(&self) -> Vec<Point> {
        self.body.iter().map(|(point, _, _)| *point).collect()
    }

    /// Moves the snake one cell in selected direction.
    ///
    /// # Arguments
    ///
    /// * `max_x` - Maximum value for X coordinate, the boundary of level.
    /// * `max_y` - Maximum value for Y coordinate, the boundary of level.
    pub fn advance(&mut self, cherry_pickup: &Pickup) -> Result<NewCell, Collision> {
        let (head_position, _, _) = self.body.front().expect("Body is empty.");
        let new_head_position = match self.next_direction {
            Direction::Left => head_position.offset((-1, 0)),
            Direction::Right => head_position.offset((1, 0)),
            Direction::Up => head_position.offset((0, -1)),
            Direction::Down => head_position.offset((0, 1)),
        };

        // check for borders first
        if new_head_position.x < 0
            || new_head_position.x >= self.field_size.width as i32
            || new_head_position.y < 0
            || new_head_position.y >= self.field_size.height as i32
        {
            return Err(Collision::Border);
        }

        // check for own_body
        if self
            .body
            .iter()
            .rev()
            .skip(1)
            .any(|(part_position, _, _)| *part_position == new_head_position)
        {
            return Err(Collision::Body);
        }

        let picked_cherry = cherry_pickup.position == new_head_position;

        self.direction = self.next_direction;

        // change old head kind
        let (_, body_part_kind, _) = self.body.front_mut().expect("Body is empty.");
        *body_part_kind = BodyPartKind::Middle;

        // attach new head
        self.body
            .push_front((new_head_position, BodyPartKind::Head, self.direction));

        if !picked_cherry {
            // remove tail
            self.body.pop_back();

            // change new back kind
            let (previous_tail_position, _, _) =
                *self.body.iter().rev().nth(1).expect("Body is too short.");

            let (tail_position, tail_part_kind, tail_part_direction) =
                self.body.back_mut().expect("Body is empty.");
            *tail_part_kind = BodyPartKind::Tail;

            *tail_part_direction = match (
                tail_position.x - previous_tail_position.x,
                tail_position.y - previous_tail_position.y,
            ) {
                (1, _) => Direction::Left,
                (-1, _) => Direction::Right,
                (_, 1) => Direction::Up,
                (_, -1) => Direction::Down,
                _ => *tail_part_direction,
            };
        }

        match picked_cherry {
            true => Ok(NewCell::Pickup),
            false => Ok(NewCell::Empty),
        }
    }

    pub fn set_next_direction(&mut self, direction: Direction) {
        self.next_direction = direction;
    }
}
