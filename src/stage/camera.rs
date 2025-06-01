use glm::*;

pub struct Camera {
    pub camera_pos: Vec3,
    pub camera_front: Vec3, // where the camera is looking at
    pub camera_up: Vec3,    // relative 'up' for the camera

    pub pitch: f32,
    pub yaw: f32,
}

// implement camera movement
impl Camera {
    pub fn move_forward(&mut self, direction_scalar: f32) {
        self.camera_pos = self.camera_pos + (self.camera_front * direction_scalar);
    }

    pub fn move_backwards(&mut self, direction_scalar: f32) {
        self.camera_pos = self.camera_pos - (self.camera_front * direction_scalar);
    }

    pub fn move_left(&mut self, direction_scalar: f32) {
        self.camera_pos = self.camera_pos
            - glm::normalize(glm::cross(self.camera_front, self.camera_up)) * direction_scalar;
    }

    pub fn move_right(&mut self, direction_scalar: f32) {
        self.camera_pos = self.camera_pos
            + glm::normalize(glm::cross(self.camera_front, self.camera_up)) * direction_scalar;
    }

    pub fn move_up(&mut self, direction_scalar: f32) {
        self.camera_pos = self.camera_pos + (self.camera_up * direction_scalar);
    }

    pub fn move_down(&mut self, direction_scalar: f32) {
        self.camera_pos = self.camera_pos - (self.camera_up * direction_scalar);
    }
}
