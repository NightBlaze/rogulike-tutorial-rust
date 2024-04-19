use crate::{left_mover::LeftMover, position::Position, MAP_WIDTH};
use specs::{Join, ReadStorage, System, WriteStorage};

pub struct LeftWalker {}

impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);
    
    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = MAP_WIDTH - 1;
            }
        }
    }
}