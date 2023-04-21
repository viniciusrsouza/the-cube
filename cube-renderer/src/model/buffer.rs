use web_sys::WebGl2RenderingContext;

use super::{DrawableContext, Entity, Light};

pub struct EntityBuffer {
    last_id: u32,
    entities: Vec<Entity>,
}

#[allow(dead_code)]
impl EntityBuffer {
    pub fn new() -> Self {
        Self {
            last_id: 0,
            entities: Vec::new(),
        }
    }

    pub fn add(&mut self, mut entity: Entity) {
        self.last_id += 1;
        entity.register(self.last_id);
        self.entities.push(entity);
    }

    pub fn get(&self, id: u32) -> Option<&Entity> {
        self.entities.iter().find(|e| e.id == id)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut Entity> {
        self.entities.iter_mut().find(|e| e.id == id)
    }

    pub fn remove(&mut self, id: u32) {
        self.entities.retain(|e| e.id != id);
    }

    pub fn get_renderables(&self) -> Vec<&Entity> {
        self.entities.iter().filter(|e| e.is_renderable()).collect()
    }

    pub fn get_renderables_mut(&mut self) -> Vec<&mut Entity> {
        self.entities
            .iter_mut()
            .filter(|e| e.is_renderable())
            .collect()
    }

    pub fn get_lights(&mut self) -> Vec<Light> {
        self.entities
            .iter_mut()
            .filter(|e| e.is_light_source())
            .map(|e| e.get_light().unwrap())
            .collect()
    }

    pub fn draw<'a>(
        &'a mut self,
        gl: &WebGl2RenderingContext,
        ctx: &mut DrawableContext<'a>,
        dt: f32,
    ) {
        let lights = self.get_lights();
        ctx.lights = Some(lights);
        for entity in self.get_renderables_mut() {
            entity.draw(gl, ctx, dt);
        }
    }

    pub fn update(&mut self, dt: f32) {
        for entity in self.entities.iter_mut() {
            entity.update(dt);
        }
    }
}
