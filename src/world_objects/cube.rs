use sdl3::{pixels::Color, render::FPoint, video::Window};

use super::{IWorldObject, WorldInfo};

pub struct Cube {
    speed: FPoint,

    rect: sdl3::rect::Rect,
}

impl Cube {
    pub fn new(center_pos: FPoint, speed: FPoint) -> Cube {
        Cube {
            speed,
            rect: sdl3::rect::Rect::new(center_pos.x as i32, center_pos.y as i32, 50, 50),
        }
    }
}

impl IWorldObject for Cube {
    fn get_world_info(&self) -> WorldInfo {
        return {
            WorldInfo {
                center_pos: FPoint::new(self.rect.center().x as f32, self.rect.center().y as f32),
                speed: self.speed,
            }
        };
    }

    fn set_position(&mut self, x: f32, y: f32) {
        // the position set sets the _center_ of the cube.
        // SDL considers the position to be the top left corner.
        // So we need to adjust the position to be the top left corner.

        self.rect.set_x(x as i32 - self.rect.width() as i32 / 2);
        self.rect.set_y(y as i32 - self.rect.height() as i32 / 2);
    }

    fn set_speed(&mut self, x: f32, y: f32) {
        self.speed = FPoint::new(x, y);
    }

    fn draw(&self, canvas: &mut sdl3::render::Canvas<Window>) {
        // draw the cube outline

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.draw_rect(self.rect.into()).unwrap();
    }
}
