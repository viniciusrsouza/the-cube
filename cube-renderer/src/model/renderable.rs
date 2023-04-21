use web_sys::WebGl2RenderingContext;

use crate::{
    app::Viewport,
    camera::Camera,
    console,
    resources::{Assets, Shader},
};

use super::{mesh::Mesh, transition::Transition, Light, Material};

pub struct Renderable {
    mesh: Mesh,
    material: Material,
    pub light: Option<Light>,
    pub shader: Option<String>,
    rotation: glm::Vec3,
    position: glm::Vec3,
    rotation_transition: Option<Transition<glm::Vec3>>,
    position_transition: Option<Transition<glm::Vec3>>,
}

impl Renderable {
    pub fn new(gl: &WebGl2RenderingContext, vertices: Vec<f32>, material: Material) -> Renderable {
        let mesh = Mesh::new(gl, vertices);
        Renderable {
            mesh,
            material,
            shader: None,
            light: None,
            rotation: glm::vec3(0.0, 0.0, 0.0),
            position: glm::vec3(0.0, 0.0, 0.0),
            rotation_transition: None,
            position_transition: None,
        }
    }

    fn get_shader<'a>(&self, assets: &'a Assets) -> &'a Shader {
        let shader = self
            .shader
            .as_ref()
            .expect("Renderable has no shader attached");
        assets
            .get_shader(shader)
            .expect(format!("Shader '{}' not found in assets", shader).as_str())
    }

    #[rustfmt::skip]
    pub fn load_attributes(&self, gl: &WebGl2RenderingContext, assets: &Assets) {
        let shader = self.get_shader(assets);
        self.mesh.load_attributes(gl, &shader.id);
    }

    pub fn is_light_source(&self) -> bool {
        self.light.is_some()
    }

    pub fn set_light(&mut self, light: Option<Light>) {
        self.light = light;
    }

    pub fn translate(&mut self, position: glm::Vec3) {
        self.position = position;
    }

    pub fn smooth_translate(
        &mut self,
        position: glm::Vec3,
        duration: f32,
        function: fn(f32) -> f32,
    ) {
        self.position_transition =
            Some(Transition::new(self.position, position, duration, function));
    }

    pub fn rotate(&mut self, rotation: glm::Vec3) {
        self.rotation = rotation;
    }

    pub fn smooth_rotate(&mut self, rotation: glm::Vec3, duration: f32, function: fn(f32) -> f32) {}

    pub fn draw<'a>(
        &'a mut self,
        gl: &WebGl2RenderingContext,
        ctx: &mut DrawableContext<'a>,
        dt: f32,
    ) {
        self.apply_transitions(dt);
        ctx.rotation = self.rotation;
        ctx.position = self.position;
        let shader = self.get_shader(ctx.assets);
        ctx.shader = Some(shader);
        ctx.material = Some(&self.material);
        self.mesh.draw(gl, ctx);
    }

    fn apply_transitions(&mut self, dt: f32) {
        if let Some(transition) = &mut self.rotation_transition {
            self.rotation = transition.update(dt);
            if transition.is_finished() {
                self.rotation_transition = None;
            }
        }
        if let Some(transition) = &mut self.position_transition {
            self.position = transition.update(dt);
            if transition.is_finished() {
                self.position_transition = None;
            }
        }
    }
}

pub struct DrawableContext<'a> {
    pub gl: &'a WebGl2RenderingContext,
    pub camera: &'a Camera,
    pub assets: &'a Assets,
    pub viewport: &'a Viewport,
    pub shader: Option<&'a Shader>,
    pub material: Option<&'a Material>,
    pub lights: Option<Vec<Light>>,
    pub rotation: glm::Vec3,
    pub position: glm::Vec3,
}

impl<'a> DrawableContext<'a> {
    pub fn new(
        gl: &'a WebGl2RenderingContext,
        camera: &'a Camera,
        assets: &'a Assets,
        viewport: &'a Viewport,
    ) -> DrawableContext<'a> {
        DrawableContext {
            gl,
            camera,
            assets,
            viewport,
            shader: None,
            material: None,
            lights: None,
            rotation: glm::vec3(0.0, 0.0, 0.0),
            position: glm::vec3(0.0, 0.0, 0.0),
        }
    }

    pub(crate) fn get_model_matrix(&self) -> glm::Mat4 {
        let mut model = glm::identity();
        model = glm::translate(&model, &self.position);
        model = glm::rotate(&model, self.rotation.x, &glm::vec3(1.0, 0.0, 0.0));
        model = glm::rotate(&model, self.rotation.y, &glm::vec3(0.0, 1.0, 0.0));
        model = glm::rotate(&model, self.rotation.z, &glm::vec3(0.0, 0.0, 1.0));
        model
    }
}
