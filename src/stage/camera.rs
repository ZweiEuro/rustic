use glam::*;

pub struct Camera {
    pub camera_pos: Vec3,
    pub camera_front: Vec3, // where the camera is looking at
    pub camera_up: Vec3,    // relative 'up' for the camera

    pub pitch: f32,
    pub yaw: f32,

    // multiplied with the direction the camera is moving to
    // most likly applied per second
    pub camera_speed: f32,

    // the current projection matrix of the camera, may be orthogonal
    // or perspective
    pub fov_y_deg: f32,
    pub aspect_ratio: f32,
    pub z_near: f32,
    pub z_far: f32,
}

// implement camera movement
impl Camera {
    pub fn move_forward(&mut self, time_delta: f32) {
        self.camera_pos = self.camera_pos + (self.camera_front * self.camera_speed * time_delta);
    }

    pub fn move_backwards(&mut self, time_delta: f32) {
        self.camera_pos = self.camera_pos - (self.camera_front * self.camera_speed * time_delta);
    }

    pub fn move_left(&mut self, time_delta: f32) {
        self.camera_pos = self.camera_pos
            - self.camera_front.cross(self.camera_up).normalize() * self.camera_speed * time_delta;
    }

    pub fn move_right(&mut self, time_delta: f32) {
        self.camera_pos = self.camera_pos
            + self.camera_front.cross(self.camera_up).normalize() * self.camera_speed * time_delta;
    }

    pub fn move_up(&mut self, time_delta: f32) {
        self.camera_pos = self.camera_pos + (self.camera_up * self.camera_speed * time_delta);
    }

    pub fn move_down(&mut self, time_delta: f32) {
        self.camera_pos = self.camera_pos - (self.camera_up * self.camera_speed * time_delta);
    }

    /**
     * Add delta to actual pitch values and recalculate the "camera_front" vector
     * Which effectively means "rotate the current view by pitch/yaw"
     * This is relative to the coordinate system, the camera has setup, so its relative to its
     * front/up vectors.
     * pitch is clamped at -89 and 89 in order to not infinitely flip the camera
     */
    pub fn change_pitch_yaw(&mut self, delta_yaw: f32, delta_pitch: f32) {
        if delta_yaw == 0.0 && delta_pitch == 0.0 {
            return;
        }
        self.yaw += delta_yaw;
        self.pitch += delta_pitch;

        self.pitch = self.pitch.clamp(-89.0, 89.0);

        let direction = Vec3 {
            x: self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            y: self.pitch.to_radians().sin(),
            z: self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        };

        self.camera_front = direction.normalize();
    }
}

impl Camera {
    pub fn get_view_matrix(&self) -> glam::f32::Mat4 {
        glam::f32::Mat4::look_at_rh(
            self.camera_pos,
            self.camera_pos + self.camera_front,
            self.camera_up,
        )
    }

    pub fn get_perspective_matrix(&self) -> glam::f32::Mat4 {
        glam::f32::Mat4::perspective_rh(
            self.fov_y_deg.to_radians(),
            self.aspect_ratio,
            self.z_near,
            self.z_far,
        )
    }
}
