use rltk::{Rltk, RGB};

use crate::{tile::TileType, MAP_HEIGHT, MAP_WIDTH};

pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; (MAP_WIDTH*MAP_HEIGHT) as usize];

    // Make the boundaries walls
    for x in 0..MAP_WIDTH {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, MAP_HEIGHT - 1)] = TileType::Wall;
    }
    for y in 0..MAP_HEIGHT {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(MAP_WIDTH - 1, y)] = TileType::Wall;
    }

    // Create random walls
    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..350 {
        let x = rng.roll_dice(1, (MAP_WIDTH - 1) as i32);
        let y = rng.roll_dice(1, (MAP_HEIGHT - 1) as i32);
        let idx = xy_idx(x as isize, y as isize);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0.0, 0.0, 0.0),
                    rltk::to_cp437('.'),
                )
            },
            TileType::Wall => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0.0, 0.0, 0.0),
                    rltk::to_cp437('#'),
                )
            },
        }

        x += 1;
        if x > MAP_WIDTH - 1 {
            x = 0;
            y += 1;
        }
    }
}

pub fn xy_idx(x: isize, y: isize) -> usize {
    ((y * MAP_WIDTH) + x) as usize
}