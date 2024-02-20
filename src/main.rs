use rltk::{GameState, Rltk, RltkBuilder, VirtualKeyCode, RGB};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    foreground: RGB,
    background: RGB,
}

struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut rltk::prelude::Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Hello, Roguelike!");
    }
}

fn main() -> rltk::BError {
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike game")
        .build()?;
    let mut game_state = State {
        ecs: World::new(),
    };
    game_state.ecs.register::<Position>();
    game_state.ecs.register::<Renderable>();
    rltk::main_loop(context, game_state)
}
