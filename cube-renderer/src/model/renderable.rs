use js_sys::{Float32Array, Uint16Array};
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlVertexArrayObject};

use crate::resources::{Assets, Shader};

pub struct Renderable {
    pub vao: WebGlVertexArrayObject,
    pub vbo: WebGlBuffer,
    pub ebo: WebGlBuffer,

    pub shader: Option<String>,
    indices_count: usize,
}

impl Renderable {
    pub fn new(gl: &WebGl2RenderingContext, vertices: Vec<f32>, indices: Vec<u16>) -> Renderable {
        let vao = gl.create_vertex_array().unwrap();
        let vbo = gl.create_buffer().unwrap();
        let ebo = gl.create_buffer().unwrap();
        gl.bind_vertex_array(Some(&vao));

        unsafe {
            gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));
            let vert_array = Float32Array::view(&vertices);
            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGl2RenderingContext::STATIC_DRAW,
            )
        };

        unsafe {
            gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&ebo));
            let indices_array = Uint16Array::view(&indices);
            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &indices_array,
                WebGl2RenderingContext::STATIC_DRAW,
            )
        };

        Renderable {
            vao,
            vbo,
            ebo,
            shader: None,
            indices_count: indices.len(),
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
        let stride = 3 * 4;
        let position_offset = 0;

        gl.vertex_attrib_pointer_with_i32(
            0, 3,
            WebGl2RenderingContext::FLOAT,
            false, stride,
            position_offset,
        );
        let shader = self.get_shader(assets);
        let position_attribute_location = gl.get_attrib_location(&shader.id, "a_Position");
        gl.enable_vertex_attrib_array(position_attribute_location as u32);

        gl.bind_vertex_array(None);
    }
}

pub trait Drawable: Sized {
    fn draw(&self, gl: &WebGl2RenderingContext, assets: &Assets);
}

impl Drawable for Renderable {
    fn draw(&self, gl: &WebGl2RenderingContext, assets: &Assets) {
        let shader = self.get_shader(assets);
        shader.use_program(gl);

        gl.bind_vertex_array(Some(&self.vao));
        gl.draw_elements_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            self.indices_count as i32,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            0,
        );
        gl.bind_vertex_array(None);
    }
}
