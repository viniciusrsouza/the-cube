use std::collections::HashMap;

use web_sys::WebGl2RenderingContext;

use super::shader::{Shader, ShaderError};

pub struct Assets {
    shaders: HashMap<String, Shader>,
}

impl Assets {
    pub fn new() -> Assets {
        Assets {
            shaders: HashMap::new(),
        }
    }

    pub fn load_shader(
        &mut self,
        gl: &WebGl2RenderingContext,
        name: &str,
        vertex_src: &str,
        fragment_src: &str,
    ) -> Result<(), ShaderError> {
        let shader = Shader::new(gl, &vertex_src, &fragment_src);

        shader.compile(gl)?;
        self.shaders.insert(name.to_string(), shader);
        Ok(())
    }

    pub fn get_shader(&self, name: &str) -> Option<&Shader> {
        self.shaders.get(name)
    }
}

#[macro_export]
macro_rules! asset_to_str {
    ($asset:expr) => {
        include_str!(concat!("../../assets/", $asset))
    };
}
