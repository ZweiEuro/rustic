use parry2d::{na::Vector2, shape::Shape};
use sdl3::{pixels::Color, render::FPoint, video::Window};

/**
 * What makes you an object in this world.
 */
#[derive(Copy, Clone, Debug)]
pub struct Object<PhysicsShape: Shape> {
    pub world_space_position: Vector2<f32>,

    // physics
    pub physics_shape: PhysicsShape,
    pub speed: Vector2<f32>,

    // basic shape
    pub color: Color,
}

trait DebugPrintable {
    fn debug_draw(&self, canvas: &mut sdl3::render::Canvas<Window>);
}

impl<T> DebugPrintable for Object<T>
where
    T: Shape,
{
    fn debug_draw(&self, canvas: &mut sdl3::render::Canvas<Window>) {
        let center = self.world_space_position;
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.draw_point(FPoint::new(center.x, center.y)).unwrap();
    }
}

pub trait PhysicsUpdated: Send + Sync {
    fn physics_update(&mut self, delta_time_s: f32);
}

impl<T> PhysicsUpdated for Object<T>
where
    T: Shape + Send + Sync,
{
    fn physics_update(&mut self, delta_time: f32) {
        self.world_space_position += self.speed * delta_time;
    }
}

pub trait Drawable: Send + Sync {
    fn draw(&self, canvas: &mut sdl3::render::Canvas<Window>);
}

impl<T> Drawable for Object<T>
where
    T: Shape + Send + Sync,
{
    fn draw(&self, canvas: &mut sdl3::render::Canvas<Window>) {
        let center = self.world_space_position;
        let rect = sdl3::rect::Rect::new(center.x as i32, center.y as i32, 10, 10);

        canvas.set_draw_color(self.color);
        canvas.draw_rect(rect.into()).unwrap();
    }
}

pub trait IWorldObject: PhysicsUpdated + Drawable {}
