mod cube_behaviour;

use crate::{
    app::App,
    asset_to_str,
    model::{Entity, Light, Material, Renderable},
    resources::ShaderError,
};

use self::cube_behaviour::CubeBehaviour;

fn cube_material() -> Material {
    Material::new(glm::vec4(0.4, 0.4, 0.4, 1.0), 32.)
}

fn light_material() -> Material {
    Material::new(glm::vec4(1.0, 1.0, 1.0, 1.0), 32.)
}

fn make_light() -> Light {
    Light::new(glm::vec4(1., 0., 1., 1.))
}

#[rustfmt::skip]
fn cube_renderable(app: &mut App, material: Material) -> Renderable {
    let vertices = vec![
        // positions       // normals
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
         0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
        -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,

        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,
         0.5, -0.5,  0.5,  0.0,  0.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
        -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,

        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,
        -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
        -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,
        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,

         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,
         0.5,  0.5, -0.5,  1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
         0.5, -0.5,  0.5,  1.0,  0.0,  0.0,
         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,

        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
         0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
        -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,

        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
         0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
        -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0
    ]; 

    let mut renderable = Renderable::new(&app.gl, vertices, material);
    renderable.shader = Some("debug".to_string());
    renderable.load_attributes(&app.gl, &app.assets);
    renderable
}

pub fn make_cube(app: &mut App) {
    let mut cube = Entity::new(glm::vec3(0., 0., 0.));
    let renderable = cube_renderable(app, cube_material());
    cube.add_renderable(renderable);
    cube.add_behaviour(Box::new(CubeBehaviour::new()));
    app.entities.add(cube);
}

pub fn make_lights(app: &mut App) {
    let positions = vec![glm::vec3(-3., 2., -5.), glm::vec3(3., 2., -5.)];
    for position in positions {
        let mut light = Entity::new(position);
        let mut renderable = cube_renderable(app, light_material());
        renderable.set_light(Some(make_light()));
        light.add_renderable(renderable);
        app.entities.add(light);
    }
}

pub async fn load_shaders(app: &mut App) -> Result<(), ShaderError> {
    let debug_frag_src = asset_to_str!("shaders/debug-frag.glsl");
    let debug_vert_src = asset_to_str!("shaders/debug-vert.glsl");

    app.assets
        .load_shader(&app.gl, "debug", debug_vert_src, debug_frag_src)?;

    Ok(())
}
