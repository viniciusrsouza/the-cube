use crate::{
    app::App,
    asset_to_str,
    model::{Entity, Renderable},
    resources::ShaderError,
};

#[rustfmt::skip]
pub fn make_cube(app: &mut App) {
    let mut cube = Entity::new();
    let vertices = vec![
         0.5, -0.5, -0.5,
         0.5, -0.5,  0.5,
        -0.5, -0.5,  0.5,
        -0.5, -0.5, -0.5,
         0.5,  0.5, -0.5,
         0.5,  0.5,  0.5,
        -0.5,  0.5,  0.5,
        -0.5,  0.5, -0.5
    ];
    let indices = vec![
        0, 1, 2, 
        0, 2, 3,
        4, 5, 6,
        4, 6, 7,
        0, 1, 5,
        0, 5, 4,
        3, 2, 6,
        3, 6, 7,
        1, 2, 6,
        1, 6, 5,
        0, 3, 7,
        0, 7, 4
    ];
    let mut renderable = Renderable::new(&app.gl, vertices, indices);
    renderable.shader = Some("debug".to_string());
    renderable.load_attributes(&app.gl, &app.assets);
    cube.add_renderable(renderable);
    app.entities.add(cube);
}

pub async fn load_shaders(app: &mut App) -> Result<(), ShaderError> {
    let debug_frag_src = asset_to_str!("shaders/debug-frag.glsl");
    let debug_vert_src = asset_to_str!("shaders/debug-vert.glsl");

    app.assets
        .load_shader(&app.gl, "debug", debug_vert_src, debug_frag_src)?;

    Ok(())
}
