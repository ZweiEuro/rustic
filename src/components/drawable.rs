use parry2d::shape::Shape;
use sdl3::pixels::Color;
use specs::Component;
use specs::prelude::*;

pub enum DrawableType {
    Rectangle,
    Circle,
}

pub struct Drawable {
    pub drawable_type: DrawableType,
    pub color: Color,

    // Rectangle
    pub width: f32,
    pub height: f32,

    // Circle
    pub radius: f32,
}

impl Component for Drawable {
    type Storage = VecStorage<Self>;
}
