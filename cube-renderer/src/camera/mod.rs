use crate::app::Viewport;

pub struct Camera {
    pub position: glm::Vec3,
    pub front: glm::Vec3,
    pub up: glm::Vec3,
    pub right: glm::Vec3,
    pub world_up: glm::Vec3,

    pub yaw: f32,
    pub pitch: f32,

    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32,
}

impl Camera {
    pub fn new(position: glm::Vec3, up: glm::Vec3, yaw: f32, pitch: f32) -> Camera {
        let mut camera = Camera {
            position,
            front: glm::vec3(0.0, 0.0, -1.0),
            up,
            right: glm::vec3(0.0, 0.0, 0.0),
            world_up: up,

            yaw,
            pitch,

            movement_speed: 2.5,
            mouse_sensitivity: 0.1,
            zoom: 45.0,
        };

        camera.update_camera_vectors();

        camera
    }

    pub fn view(&self) -> glm::Mat4 {
        glm::look_at(&self.position, &(&self.position + &self.front), &self.up)
    }

    pub fn projection(&self, viewport: &Viewport) -> glm::Mat4 {
        glm::perspective(viewport.aspect_ratio(), self.zoom.to_radians(), 0.1, 100.0)
    }

    fn update_camera_vectors(&mut self) {
        let front = glm::vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );

        self.front = glm::normalize(&front);
        self.right = glm::normalize(&glm::cross(&self.front, &self.world_up));
        self.up = glm::normalize(&glm::cross(&self.right, &self.front));
    }
}
