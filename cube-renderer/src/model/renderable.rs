use web_sys::WebGl2RenderingContext;

use crate::{
    app::Viewport,
    camera::Camera,
    resources::{Assets, Shader},
};

use super::{mesh::Mesh, Light, Material};

pub struct Renderable {
    mesh: Mesh,
    material: Material,
    transforms: Vec<Transform>,
    pub light: Option<Light>,
    pub shader: Option<String>,
}

impl Renderable {
    pub fn new(gl: &WebGl2RenderingContext, vertices: Vec<f32>, material: Material) -> Renderable {
        let mesh = Mesh::new(gl, vertices);
        Renderable {
            mesh,
            material,
            shader: None,
            light: None,
            transforms: Vec::new(),
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

    pub fn with_transform(&mut self, transform: Transform) -> &mut Self {
        self.transforms.push(transform);
        self
    }

    pub fn draw<'a>(&'a mut self, gl: &WebGl2RenderingContext, ctx: &mut DrawableContext<'a>) {
        let shader = self.get_shader(ctx.assets);
        ctx.shader = Some(shader);
        ctx.material = Some(&self.material);
        ctx.transforms = Some(&mut self.transforms);
        self.mesh.draw(gl, ctx);
    }
}

pub struct DrawableContext<'a> {
    pub gl: &'a WebGl2RenderingContext,
    pub camera: &'a Camera,
    pub assets: &'a Assets,
    pub viewport: &'a Viewport,
    pub shader: Option<&'a Shader>,
    pub material: Option<&'a Material>,
    pub transforms: Option<&'a mut Vec<Transform>>,
    pub lights: Option<Vec<Light>>,
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
            transforms: None,
            lights: None,
        }
    }
}

impl Drop for DrawableContext<'_> {
    fn drop(&mut self) {
        self.transforms.as_mut().and_then(|ts| {
            ts.clear();
            Some(())
        });
    }
}

#[allow(unused)]
pub enum Transform {
    Translate(glm::Vec3),
    Rotate(glm::Vec3),
    Scale(glm::Vec3),
}

pub trait Transforms<T> {
    fn apply(&self, value: &mut T);
}

impl Transforms<glm::Mat4> for Transform {
    fn apply(&self, value: &mut glm::Mat4) {
        match self {
            Transform::Translate(translation) => {
                *value = glm::translate(value, &translation);
            }
            Transform::Rotate(rotation) => {
                *value = glm::rotate(value, rotation.x, &glm::vec3(1.0, 0.0, 0.0));
                *value = glm::rotate(value, rotation.y, &glm::vec3(0.0, 1.0, 0.0));
                *value = glm::rotate(value, rotation.z, &glm::vec3(0.0, 0.0, 1.0));
            }
            Transform::Scale(scale) => {
                *value = glm::scale(value, &scale);
            }
        }
    }
}
