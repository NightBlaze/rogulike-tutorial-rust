use rltk::{self, RGB};
use specs_derive::Component;
use specs::prelude::*;

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub foreground: RGB,
    pub background: RGB,
}