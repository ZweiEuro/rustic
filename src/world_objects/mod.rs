pub mod cube;
use sdl3::{pixels::Color, render::FPoint, video::Window};

/**
 * Central information any object in the world needs to know about itself.
 */
#[derive(Copy, Clone, Debug)]
pub struct WorldInfo {
    pub center_pos: FPoint,
    pub speed: FPoint,
}

pub trait IWorldObject {
    fn get_world_info(&self) -> WorldInfo;
    fn set_position(&mut self, x: f32, y: f32);
    fn set_speed(&mut self, x: f32, y: f32);

    fn draw(&self, canvas: &mut sdl3::render::Canvas<Window>);

    /**
     * Apply speed from the world info to the center position.
     */
    fn physics_update(&mut self, delta_time: f32) {
        let world_info = self.get_world_info();
        let mut center = world_info.center_pos;
        let speed = world_info.speed;

        print!(
            "Updating object {} prev {:?} speed: {:?} now {} {} \n",
            delta_time,
            center,
            speed,
            center.x + speed.x * delta_time,
            center.y + speed.y * delta_time
        );
        center.x += speed.x * delta_time;
        center.y += speed.y * delta_time;

        // if the new position reaches out of the (for now fixed) screen size, reflect it off the wall

        if center.y < 0.0 {
            // colliding with upper wall
            self.set_speed(speed.x, -speed.y);
        } else if center.y > 600.0 {
            // colliding with lower wall
            self.set_speed(speed.x, -speed.y);
        } else if center.x < 0.0 {
            // colliding with left wall
            self.set_speed(-speed.x, speed.y);
        } else if center.x > 800.0 {
            // colliding with right wall
            self.set_speed(-speed.x, speed.y);
        } else {
            self.set_position(center.x, center.y);
        }
    }

    /**
     * A basic debug draw function that draws the center of the object.
     */
    fn draw_center(&self, canvas: &mut sdl3::render::Canvas<Window>) {
        let world_info = self.get_world_info();
        let center = world_info.center_pos;
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.draw_point(FPoint::new(center.x, center.y)).unwrap();
    }
}
