use sdl3::{Sdl, pixels::Color};

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

impl<'a> System<'a> for SysRender {
    type SystemData = PhysicsAndDrawable<'a>;

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
    }

    fn run(&mut self, data: PhysicsAndDrawable) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();

        for (physics, drawable) in (&data.physics, &data.drawable).join() {
            match drawable.drawable_type {
                DrawableType::Rectangle => {
                    let rect = sdl3::rect::Rect::new(
                        (physics.world_space_position.x - drawable.width / 2.0) as i32,
                        (physics.world_space_position.y - drawable.height / 2.0) as i32,
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
