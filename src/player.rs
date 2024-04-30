use crate::{position::Position, MAP_HEIGHT, MAP_WIDTH};
use specs_derive::Component;
use specs::prelude::*;
use std::cmp::{max, min};

#[derive(Component, Debug)]
pub struct Player {}

pub fn try_move_player(delta_x: isize, delta_y: isize, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = min(MAP_WIDTH - 1, max(0, pos.x + delta_x));
        pos.y = min(MAP_HEIGHT - 1, max(0, pos.y + delta_y));
    }
}
