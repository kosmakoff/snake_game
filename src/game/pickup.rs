use crate::game::snake_sprite::SpriteData;
use crate::point::Point;
use crate::size::Size;
use crate::sprite_renderer::GenericContext;

use rand::{thread_rng, Rng};
use std::collections::HashSet;

fn get_hashcode(x: u32, y: u32) -> u32 {
    (17 * 31 + x) * 31 + y
}

fn find_non_occupied_cell<'a, S, P>(field_size: S, occupied_cells: &'a [P]) -> Option<Point>
where
    S: Into<Size>,
    P: Into<Point>,
    &'a P: Into<Point>,
{
    let field_size: Size = field_size.into();
    let space_size = field_size.width * field_size.height;
    let rand_range = space_size - occupied_cells.len() as u32;

    if rand_range <= 0 {
        return None;
    }

    let occupied_cells_hashset: HashSet<u32> = occupied_cells
        .iter()
        .map(|point| {
            let point: Point = point.into();
            get_hashcode(point.x as u32, point.y as u32)
        })
        .collect();

    let mut rng = thread_rng();
    let empty_cell_index: u32 = rng.gen_range(0, rand_range);

    let mut target_cell_index = 0;

    for i in 0..=empty_cell_index {
        // loop until we find the empty cell and increase the
        loop {
            let target_cell_y = target_cell_index / field_size.width;
            let target_cell_x = target_cell_index % field_size.width;
            let target_cell_hashcode = get_hashcode(target_cell_x, target_cell_y);
            if !occupied_cells_hashset.contains(&target_cell_hashcode) {
                break;
            }
            target_cell_index += 1;
        }

        if i == empty_cell_index {
            let target_cell_y = target_cell_index / field_size.width;
            let target_cell_x = target_cell_index % field_size.width;
            return Some((target_cell_x as i32, target_cell_y as i32).into());
        }

        target_cell_index += 1;
    }

    None
}

pub enum PickupKind {
    Cherry,
    Apple,
}

pub struct Pickup {
    pub pickup_kind: PickupKind,
    pub position: Point,
}

impl Pickup {
    pub fn new_cherry<'a, S, P>(
        field_size: S,
        occupied_cells: &'a [P],
    ) -> Option<Self>
    where
        S: Into<Size>,
        P: Into<Point>,
        &'a P: Into<Point>
    {
        find_non_occupied_cell(field_size, occupied_cells).map(|position| Pickup {
            pickup_kind: PickupKind::Cherry,
            position,
        })
    }

    pub fn new_apple<'a, S, P>(
        field_size: S,
        occupied_cells: &'a [P],
    ) -> Option<Self>
    where
        S: Into<Size>,
        P: Into<Point>,
        &'a P: Into<Point>,
    {
        find_non_occupied_cell(field_size, occupied_cells).map(|position| Pickup {
            pickup_kind: PickupKind::Apple,
            position,
        })
    }

    pub fn render<C>(&self, context: &mut C, all_sprites: &SpriteData)
    where
        C: GenericContext,
    {
        let texture = match &self.pickup_kind {
            PickupKind::Cherry => &all_sprites.cherry,
            PickupKind::Apple => &all_sprites.apple,
        };

        context.draw_sprite((self.position.x, self.position.y), texture);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_non_occupied_cell() {
        let cells: Vec<(i32, i32)> = vec![(0, 0), (1, 0), (1, 1)];
        assert_eq!(
            find_non_occupied_cell((2, 2), &cells),
            Some((0_i32, 1_i32).into())
        );

        let cells: Vec<(i32, i32)> = vec![(0, 0), (1, 0), (0, 1)];
        assert_eq!(find_non_occupied_cell((2, 2), &cells), Some((1, 1).into()));

        let cells: Vec<(i32, i32)> = vec![(0, 0), (1, 0), (0, 1), (1, 1)];
        assert_eq!(find_non_occupied_cell((2, 2), &cells), None);
    }
}
