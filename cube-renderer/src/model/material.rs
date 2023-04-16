use web_sys::WebGl2RenderingContext;

use crate::resources::Shader;

pub struct Material {
    pub ambient: glm::Vec4,
    pub diffuse: glm::Vec4,
    pub specular: glm::Vec4,
    pub shininess: f32,
}

impl Material {
    pub fn new(color: glm::Vec4, shininess: f32) -> Material {
        Material {
            ambient: color,
            diffuse: color,
            specular: color,
            shininess,
        }
    }

    #[allow(unused)]
    pub fn new_with_colors(
        ambient: glm::Vec4,
        diffuse: glm::Vec4,
        specular: glm::Vec4,
        shininess: f32,
    ) -> Material {
        Material {
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn apply_to_shader(&self, gl: &WebGl2RenderingContext, shader: &Shader) {
        shader.set_uniform_vec4(gl, "u_Material.ambient", &self.ambient);
        shader.set_uniform_vec4(gl, "u_Material.diffuse", &self.diffuse);
        shader.set_uniform_vec4(gl, "u_Material.specular", &self.specular);
        shader.set_uniform_float(gl, "u_Material.shininess", self.shininess);
    }
}
