mod left_mover;
mod left_walker;
mod map;
mod player;
mod position;
mod renderable;
mod state;
mod tile;

use left_mover::LeftMover;
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
    game_state.ecs.register::<LeftMover>();
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
    for i in 0..10 {
        game_state.ecs
            .create_entity()
            .with(Position {x: i * 7, y: 20})
            .with(Renderable {
                glyph: rltk::to_cp437('☺'),
                foreground: RGB::named(rltk::RED),
                background: RGB::named(rltk::BLACK),
            })
            .with(LeftMover{})
            .build();
    }
    game_state.ecs.insert(new_map());
    rltk::main_loop(context, game_state)
}