use glam::{Mat4, Vec3};

pub struct OrbitCamera {
    pub target: Vec3,
    pub distance: f32,
    pub yaw: f32,   // Horizontal orbit angle in radians
    pub pitch: f32, // Vertical elevation angle in radians
    pub fov_y_rad: f32,
    pub aspect: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            target: Vec3::ZERO,
            distance: 3.2,
            yaw: 0.8,
            pitch: 0.45,
            fov_y_rad: 50.0f32.to_radians(),
            aspect: 16.0 / 9.0,
            z_near: 0.1,
            z_far: 100.0,
        }
    }
}

impl OrbitCamera {
    pub fn eye_position(&self) -> Vec3 {
        let x = self.distance * self.pitch.cos() * self.yaw.sin();
        let y = self.distance * self.pitch.sin();
        let z = self.distance * self.pitch.cos() * self.yaw.cos();
        self.target + Vec3::new(x, y, z)
    }

    pub fn build_view_proj_matrix(&self) -> Mat4 {
        let eye = self.eye_position();
        let view = Mat4::look_at_rh(eye, self.target, Vec3::Y);
        let proj = Mat4::perspective_rh(self.fov_y_rad, self.aspect, self.z_near, self.z_far);
        proj * view
    }

    pub fn rotate(&mut self, delta_x: f32, delta_y: f32) {
        let sensitivity = 0.006;
        self.yaw -= delta_x * sensitivity;
        self.pitch += delta_y * sensitivity;

        // Clamp pitch to prevent gimbal flip
        let limit = 85.0f32.to_radians();
        self.pitch = self.pitch.clamp(-limit, limit);
    }

    pub fn zoom(&mut self, delta: f32) {
        let zoom_factor = 1.0 - delta * 0.0015;
        self.distance = (self.distance * zoom_factor).clamp(1.0, 15.0);
    }

    pub fn update_aspect(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.aspect = (width as f32) / (height as f32);
        }
    }
}
