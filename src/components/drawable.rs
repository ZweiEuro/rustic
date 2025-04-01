use sdl3::pixels::Color;
use specs::Component;
use specs::prelude::*;

#[derive(Debug, Clone)]
pub enum DrawableType {
    Rectangle { width: f32, height: f32 },
    Circle { radius: f32 },
}

pub struct Drawable {
    pub shape: DrawableType,
    pub color: Color,
}

impl Component for Drawable {
    type Storage = VecStorage<Self>;
}
