use js_sys::Float32Array;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlVertexArrayObject};

use crate::model::{DrawableContext, Transforms};

pub struct Mesh {
    pub vao: WebGlVertexArrayObject,
    pub vbo: WebGlBuffer,
    triangles: usize,
}

impl Mesh {
    pub fn new(gl: &WebGl2RenderingContext, vertices: Vec<f32>) -> Mesh {
        let vao = gl.create_vertex_array().unwrap();
        let vbo = gl.create_buffer().unwrap();
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

        Mesh {
            vao,
            vbo,
            triangles: vertices.len() / 6,
        }
    }

    #[rustfmt::skip]
    pub fn load_attributes(&self, gl: &WebGl2RenderingContext, program: &WebGlProgram) {
        gl.bind_vertex_array(Some(&self.vao));
        let stride = 6 * 4;
        let position_offset = 0;
        let normal_offset = 3 * 4;

        gl.vertex_attrib_pointer_with_i32(
            0, 3,
            WebGl2RenderingContext::FLOAT,
            false, stride,
            position_offset,
        );
        gl.vertex_attrib_pointer_with_i32(
            1, 3,
            WebGl2RenderingContext::FLOAT,
            false, stride,
            normal_offset,
        );

        let position_attribute_location = gl.get_attrib_location(program, "a_Position");
        gl.enable_vertex_attrib_array(position_attribute_location as u32);

        let position_attribute_location = gl.get_attrib_location(program, "a_Normal");
        gl.enable_vertex_attrib_array(position_attribute_location as u32);

        gl.bind_vertex_array(None);
    }

    pub fn draw<'a>(&self, gl: &WebGl2RenderingContext, ctx: &DrawableContext<'a>) {
        let mut model = glm::Mat4::identity();
        let view = ctx.camera.view();
        let projection = ctx.camera.projection(ctx.viewport);
        let view_pos = ctx.camera.position;

        ctx.transforms.as_ref().and_then(|transforms| {
            for transform in transforms.iter() {
                transform.apply(&mut model);
            }
            Some(())
        });

        if let Some(shader) = ctx.shader {
            shader.use_program(gl);
            shader.set_uniform_mat4(gl, "u_Model", &model);
            shader.set_uniform_mat4(gl, "u_View", &view);
            shader.set_uniform_mat4(gl, "u_Projection", &projection);
            shader.set_uniform_vec3(gl, "u_ViewPos", &view_pos);

            if let Some(material) = ctx.material {
                material.apply_to_shader(gl, shader);
            }

            if let Some(lights) = &ctx.lights {
                for (i, light) in lights.iter().enumerate() {
                    light.apply_to_shader(gl, shader, i);
                }
            }
        }

        ctx.gl.bind_vertex_array(Some(&self.vao));
        ctx.gl
            .draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, self.triangles as i32);
        ctx.gl.bind_vertex_array(None);
    }
}
