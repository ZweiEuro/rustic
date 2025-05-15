use sokol::gfx::Color;
use specs::Component;
use specs::prelude::*;

use super::physics::Shape;

pub struct DrawableComp {
    pub shape: Shape,
    pub color: Color,
}

impl Component for DrawableComp {
    type Storage = VecStorage<Self>;
}
