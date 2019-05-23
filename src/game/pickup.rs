use crate::game::snake_sprite::SpriteData;
use graphics::{Context, Image, Transformed};
use opengl_graphics::{GlGraphics, Texture};
use rand::{thread_rng, Rng};
use std::collections::HashSet;

fn get_hashcode(x: u32, y: u32) -> u32 {
    (17 * 31 + x) * 31 + y
}

fn find_non_occupied_cell(
    max_x: u32,
    max_y: u32,
    occupied_cells: &[(u32, u32)],
) -> Option<(u32, u32)> {
    let space_size = (max_x + 1) * (max_y + 1);
    let rand_range = space_size - occupied_cells.len() as u32;

    if rand_range <= 0 {
        return None;
    }

    let occupied_cells_hashset: HashSet<u32> = occupied_cells
        .iter()
        .map(|(x, y)| get_hashcode(*x, *y))
        .collect();

    let mut rng = thread_rng();
    let empty_cell_index: u32 = rng.gen_range(0, rand_range);

    let mut target_cell_index = 0;

    for i in 0..=empty_cell_index {
        // loop until we find the empty cell and increase the
        loop {
            let target_cell_y = target_cell_index / (max_x + 1);
            let target_cell_x = target_cell_index % (max_x + 1);
            let target_cell_hashcode = get_hashcode(target_cell_x, target_cell_y);
            if !occupied_cells_hashset.contains(&target_cell_hashcode) {
                break;
            }
            target_cell_index += 1;
        }

        if i == empty_cell_index {
            let target_cell_y = target_cell_index / (max_x + 1);
            let target_cell_x = target_cell_index % (max_x + 1);
            return Some((target_cell_x, target_cell_y));
        }

        target_cell_index += 1;
    }

    None
}

pub enum Pickup {
    Cherry(u32, u32),
    Apple(u32, u32),
}

impl Pickup {
    pub fn new_cherry(max_x: u32, max_y: u32, occupied_cells: &[(u32, u32)]) -> Option<Self> {
        find_non_occupied_cell(max_x, max_y, occupied_cells).map(|(x, y)| Pickup::Cherry(x, y))
    }

    pub fn new_apple(max_x: u32, max_y: u32, occupied_cells: &[(u32, u32)]) -> Option<Self> {
        find_non_occupied_cell(max_x, max_y, occupied_cells).map(|(x, y)| Pickup::Apple(x, y))
    }

    pub fn render(
        &self,
        gl: &mut GlGraphics,
        context: &Context,
        all_sprites: &SpriteData,
        sprite_size: u32,
    ) {
        let (texture, x, y) = match &self {
            Pickup::Cherry(x, y) => (&all_sprites.cherry, x, y),
            Pickup::Apple(x, y) => (&all_sprites.apple, x, y),
        };

        let image = Image::new();

        // offset one cell for border
        let transform = context
            .transform
            .trans_pos([sprite_size as f64, sprite_size as f64]);

        let transform = transform.trans_pos([(*x * sprite_size) as f64, (*y * sprite_size) as f64]);

        image.draw(texture, &context.draw_state, transform, gl);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_find_non_occupied_cell() {
        let cells: Vec<(u32, u32)> = vec![(0, 0), (1, 0), (1, 1)];
        assert_eq!(find_non_occupied_cell(1, 1, &cells), Some((0, 1)));

        let cells: Vec<(u32, u32)> = vec![(0, 0), (1, 0), (0, 1)];
        assert_eq!(find_non_occupied_cell(1, 1, &cells), Some((1, 1)));

        let cells: Vec<(u32, u32)> = vec![(0, 0), (1, 0), (0, 1), (1, 1)];
        assert_eq!(find_non_occupied_cell(1, 1, &cells), None);
    }
}
