use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

pub struct Shader {
    pub id: WebGlProgram,
    vertex_src: String,
    fragment_src: String,
}

impl Shader {
    pub fn new(gl: &WebGl2RenderingContext, vertex_src: &str, fragment_src: &str) -> Shader {
        let id = gl.create_program().unwrap();
        Shader {
            id,
            vertex_src: vertex_src.to_string(),
            fragment_src: fragment_src.to_string(),
        }
    }

    pub fn compile(&self, gl: &WebGl2RenderingContext) -> Result<(), ShaderError> {
        let vert_shader =
            self.compile_shader(gl, WebGl2RenderingContext::VERTEX_SHADER, &self.vertex_src)?;

        let frag_shader = self.compile_shader(
            gl,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            &self.fragment_src,
        )?;

        self.link_shaders(gl, &vert_shader, &frag_shader)?;

        gl.delete_shader(Some(&vert_shader));
        gl.delete_shader(Some(&frag_shader));

        Ok(())
    }

    fn compile_shader(
        &self,
        gl: &WebGl2RenderingContext,
        typ: u32,
        source: &str,
    ) -> Result<WebGlShader, ShaderError> {
        let shader = gl.create_shader(typ);
        if shader.is_none() {
            return Err(ShaderError::UnknownError);
        }
        let shader = shader.unwrap();

        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);

        if gl
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            return Ok(shader);
        }

        let error = gl.get_shader_info_log(&shader);
        if error.is_none() {
            return Err(ShaderError::UnknownError);
        }

        let error = error.unwrap();
        if typ == WebGl2RenderingContext::VERTEX_SHADER {
            return Err(ShaderError::CompileError(format!(
                "Vertex shader error: {}",
                error
            )));
        } else if typ == WebGl2RenderingContext::FRAGMENT_SHADER {
            return Err(ShaderError::CompileError(format!(
                "Fragment shader error: {}",
                error
            )));
        } else {
            return Err(ShaderError::UnknownError);
        }
    }

    fn link_shaders(
        &self,
        gl: &WebGl2RenderingContext,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<(), ShaderError> {
        gl.attach_shader(&self.id, vert_shader);
        gl.attach_shader(&self.id, frag_shader);
        gl.link_program(&self.id);

        if gl
            .get_program_parameter(&self.id, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            return Ok(());
        }

        let error = gl.get_program_info_log(&self.id);
        if error.is_none() {
            return Err(ShaderError::UnknownError);
        }

        let error = error.unwrap();
        return Err(ShaderError::LinkError(format!("Link error: {}", error)));
    }

    pub fn use_program(&self, gl: &WebGl2RenderingContext) {
        gl.use_program(Some(&self.id));
    }
}

#[derive(Debug)]
pub enum ShaderError {
    CompileError(String),
    LinkError(String),
    UnknownError,
}
