use crate::resources::Assets;

use super::{renderable::Drawable, Entity};

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
}

impl Drawable for EntityBuffer {
    fn draw(&self, gl: &web_sys::WebGl2RenderingContext, assets: &Assets) {
        for entity in self.get_renderables() {
            entity.draw(gl, assets);
        }
    }
}
