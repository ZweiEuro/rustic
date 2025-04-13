use sdl3::{pixels::Color, render::FPoint};

use specs::prelude::*;

use crate::{
    DebugSysState,
    components::{DrawableComp, PhysicsComp, Shape},
};

#[derive(SystemData)]
pub struct Data<'a> {
    physics: ReadStorage<'a, PhysicsComp>,
    drawable: ReadStorage<'a, DrawableComp>,

    debug_sys_info: Read<'a, DebugSysState>,
}

pub struct SysRender {
    pub canvas: sdl3::render::Canvas<sdl3::video::Window>,
}

impl<'a> System<'a> for SysRender {
    type SystemData = Data<'a>;

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
    }

    fn run(&mut self, data: Data) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();

        for (physics, drawable) in (&data.physics, &data.drawable).join() {
            match drawable.shape {
                Shape::Rectangle { width, height } => {
                    let rect = sdl3::rect::Rect::new(
                        (physics.physics.world_space_position.x - width / 2.0) as i32,
                        (physics.physics.world_space_position.y - height / 2.0) as i32,
                        width as u32,
                        height as u32,
                    );

                    self.canvas.set_draw_color(drawable.color);

                    self.canvas.set_draw_color(drawable.color);
                    self.canvas.draw_rect(rect.into()).unwrap();
                }
                Shape::Circle { radius } => {
                    todo!("Implement the circle drawing")
                }
            }

            #[cfg(feature = "dbg_draw_direction")]
            if (DRAW_DIRECTION) {
                let start = FPoint::new(
                    physics.physics.world_space_position.x,
                    physics.physics.world_space_position.y,
                );

                let offset_direction = physics.physics.direction * 50.0;

                let end = FPoint::new(
                    physics.physics.world_space_position.x + offset_direction.x,
                    physics.physics.world_space_position.y + offset_direction.y,
                );

                self.canvas.draw_line(start, end).unwrap();
            }
        }

        self.canvas.present();
    }
}
