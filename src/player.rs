use crate::{map::xy_idx, position::Position, tile::TileType, MAP_HEIGHT, MAP_WIDTH};
use specs_derive::Component;
use specs::prelude::*;
use std::cmp::{max, min};

#[derive(Component, Debug)]
pub struct Player {}

pub fn try_move_player(delta_x: isize, delta_y: isize, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[destination_idx] != TileType::Wall {
            pos.x = min(MAP_WIDTH - 1, max(0, pos.x + delta_x));
            pos.y = min(MAP_HEIGHT - 1, max(0, pos.y + delta_y));
        }
    }
}
