use sdl3::Sdl;
use specs::prelude::*;

use crate::components::{Drawable, DrawableType, Physics};

#[derive(SystemData)]
pub struct PhysicsAndDrawable<'a> {
    physics: ReadStorage<'a, Physics>,
    drawable: ReadStorage<'a, Drawable>,
}

pub struct SysRender {
    pub canvas: sdl3::render::Canvas<sdl3::video::Window>,
}

unsafe impl Send for SysRender {}
unsafe impl Sync for SysRender {}

impl<'a> System<'a> for SysRender {
    type SystemData = PhysicsAndDrawable<'a>;

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
    }

    fn run(&mut self, data: PhysicsAndDrawable) {
        print!("Rendering in thread: {:?}\n", std::thread::current().id());

        self.canvas.clear();

        for (physics, drawable) in (&data.physics, &data.drawable).join() {
            match drawable.drawable_type {
                DrawableType::Rectangle => {
                    let center = physics.world_space_position;
                    let rect = sdl3::rect::Rect::new(
                        center.x as i32,
                        center.y as i32,
                        drawable.width as u32,
                        drawable.height as u32,
                    );

                    self.canvas.set_draw_color(drawable.color);

                    self.canvas.set_draw_color(drawable.color);
                    self.canvas.draw_rect(rect.into()).unwrap();
                }
                DrawableType::Circle => {
                    todo!("Implement the circle drawing")
                }
            }
        }

        self.canvas.present();
    }
}
