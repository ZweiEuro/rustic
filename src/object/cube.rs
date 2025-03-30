use parry2d::na::Vector2;
use sdl3::{pixels::Color, video::Window};

use super::object::{Drawable, IWorldObject, Object, PhysicsUpdated};

pub struct Cube {
    pub object: Object<parry2d::shape::Cuboid>,

    pub width: f32,
    pub height: f32,

    // physics
    pub is_static: bool,
}

impl Cube {
    pub fn new(
        position: Vector2<f32>,
        size: Vector2<f32>,
        initial_speed: Vector2<f32>,
        is_static: bool,
    ) -> Cube {
        Cube {
            object: Object {
                world_space_position: position,
                physics_shape: parry2d::shape::Cuboid::new(size),
                speed: initial_speed,
                color: Color::RGB(0, 0, 0),
            },
            width: size.x,
            height: size.y,

            //physics
            is_static: is_static,
        }
    }
}

impl PhysicsUpdated for Cube {
    fn physics_update(&mut self, delta_time_s: f32) {
        if !self.is_static {
            self.object.physics_update(delta_time_s);
        }
    }
}

impl Drawable for Cube {
    fn draw(&self, canvas: &mut sdl3::render::Canvas<Window>) {
        let center = self.object.world_space_position;
        let rect = sdl3::rect::Rect::new(
            center.x as i32,
            center.y as i32,
            self.width as u32,
            self.height as u32,
        );

        canvas.set_draw_color(self.object.color);
        canvas.draw_rect(rect.into()).unwrap();
    }
}

impl IWorldObject for Cube {}
