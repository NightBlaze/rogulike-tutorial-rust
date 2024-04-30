use crate::{left_walker::LeftWalker, map::draw_map, player::Player, position::Position, renderable::Renderable, tile::TileType, MAP_HEIGHT, MAP_WIDTH};
use rltk::{GameState, Rltk, VirtualKeyCode};
use specs::{Join, RunNow, World, WorldExt};
use std::cmp::{max, min};

pub struct State {
    pub ecs: World,
}

impl State {
    pub fn run_systems(&mut self) {
        let mut left_walker = LeftWalker{};
        left_walker.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut rltk::prelude::Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.foreground, render.background, render.glyph);
        }
    }
}

fn try_move_player(delta_x: isize, delta_y: isize, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = min(MAP_WIDTH - 1, max(0, pos.x + delta_x));
        pos.y = min(MAP_HEIGHT - 1, max(0, pos.y + delta_y));
    }
}

fn player_input(game_state: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {},
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut game_state.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut game_state.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut game_state.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut game_state.ecs),
            _ => {},
        },
    }
}