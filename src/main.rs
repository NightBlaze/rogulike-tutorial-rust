use std::cmp::{max, min};

use rltk::{GameState, Rltk, RltkBuilder, VirtualKeyCode, RGB};
use specs::prelude::*;
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

#[derive(Component)]
struct LeftMover {}

struct LeftWalker {}

impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);
    
    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }
}

#[derive(Component, Debug)]
struct Player {}

struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
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

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.foreground, render.background, render.glyph);
        }
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
    rltk::main_loop(context, game_state)
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
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