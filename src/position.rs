use specs_derive::Component;
use specs::prelude::*;

#[derive(Component)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}