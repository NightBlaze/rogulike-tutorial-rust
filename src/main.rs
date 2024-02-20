use rltk::{GameState, RltkBuilder};

struct State { }

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
    let game_state = State { };
    rltk::main_loop(context, game_state)
}
