use web_sys::WebGl2RenderingContext;

use crate::resources::Shader;

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub ambient: glm::Vec4,
    pub diffuse: glm::Vec4,
    pub specular: glm::Vec4,
    position: Option<glm::Vec3>,
}

impl Light {
    pub fn new(color: glm::Vec4) -> Light {
        Light {
            ambient: color,
            diffuse: color,
            specular: color,
            position: None,
        }
    }

    #[allow(unused)]
    pub fn new_with_colors(ambient: glm::Vec4, diffuse: glm::Vec4, specular: glm::Vec4) -> Light {
        Light {
            ambient,
            diffuse,
            specular,
            position: None,
        }
    }

    pub fn with_position(&mut self, position: glm::Vec3) -> &Light {
        self.position = Some(position);
        self
    }

    pub fn apply_to_shader(&self, gl: &WebGl2RenderingContext, shader: &Shader, index: usize) {
        let position = self.position.unwrap();
        let light_name = format!("u_Lights[{}]", index);
        shader.set_uniform_vec3(gl, &format!("{}.position", light_name), &position);
        shader.set_uniform_vec4(gl, &format!("{}.ambient", light_name), &self.ambient);
        shader.set_uniform_vec4(gl, &format!("{}.diffuse", light_name), &self.diffuse);
        shader.set_uniform_vec4(gl, &format!("{}.specular", light_name), &self.specular);
    }
}
