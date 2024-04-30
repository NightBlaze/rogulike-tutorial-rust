mod map;
mod player;
mod position;
mod renderable;
mod state;
mod tile;

use map::new_map;
use player::Player;
use position::Position;
use renderable::Renderable;
use rltk::{RltkBuilder, RGB};
use specs::prelude::*;
use state::State;

pub const MAP_WIDTH: isize = 80;
pub const MAP_HEIGHT: isize = 50;

fn main() -> rltk::BError {
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike game")
        .build()?;
    let mut game_state = State {
        ecs: World::new(),
    };
    game_state.ecs.register::<Position>();
    game_state.ecs.register::<Renderable>();
    game_state.ecs.register::<Player>();
    game_state.ecs
            .create_entity()
            .with(Position {x: 40, y: 25})
            .with(Renderable {
                glyph: rltk::to_cp437('@'),
                foreground: RGB::named(rltk::YELLOW),
                background: RGB::named(rltk::BLACK),
            })
            .with(Player{})
            .build();
    game_state.ecs.insert(new_map());
    rltk::main_loop(context, game_state)
}