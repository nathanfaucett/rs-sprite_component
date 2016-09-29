use collections::boxed::Box;

use shared::Shared;
use scene_graph::{Entity, Component, ComponentManager, Id};
use sprite_manager::SpriteManager;


struct SpriteData {

    entity: Option<Entity>,
    sprite_manager: Option<SpriteManager>,

    layer: usize,
    z: isize,

    material: Option<usize>,

    width: f32,
    height: f32,

    x: f32,
    y: f32,
    w: f32,
    h: f32,
}


#[derive(Clone)]
pub struct Sprite {
    data: Shared<SpriteData>,
}

impl Sprite {
    pub fn new() -> Self {
        Sprite {
            data: Shared::new(SpriteData {

                entity: None,
                sprite_manager: None,

                layer: 0usize,
                z: 0isize,

                material: None,

                width: 1f32,
                height: 1f32,

                x: 0f32,
                y: 0f32,
                w: 1f32,
                h: 1f32,
            })
        }
    }

    pub fn get_sprite_manager(&self) -> Option<SpriteManager> {
        self.data.sprite_manager.clone()
    }
    pub fn set_sprite_manager(&mut self, sprite_manager: Option<SpriteManager>) {
        self.data.sprite_manager = sprite_manager;
    }

    pub fn get_layer(&self) -> usize { self.data.layer }
    pub fn set_layer(&mut self, layer: usize) -> &Self {
        let old_layer = self.data.layer;

        if old_layer != layer {
            let mut sprite_manager_ref = None;

            if let Some(ref sprite_manager) = self.data.sprite_manager {
                sprite_manager_ref = Some(sprite_manager.clone());
            }

            if let Some(ref mut sprite_manager) = sprite_manager_ref {
                let component = &mut (Box::new(self.clone()) as Box<Component>);
                sprite_manager.remove_component(component);
                self.data.layer = layer;
                sprite_manager.add_component(component);
            }
        }

        self
    }

    pub fn get_z(&self) -> isize { self.data.z }
    pub fn set_z(&mut self, z: isize) -> &Self {
        let old_z = self.data.z;

        if old_z != z {
            self.data.z = z;

            let mut sprite_manager_ref = None;

            if let Some(ref sprite_manager) = self.data.sprite_manager {
                sprite_manager_ref = Some(sprite_manager.clone());
            }

            if let Some(mut sprite_manager) = sprite_manager_ref {
                sprite_manager.sort_layer(self.get_layer());
            }
        }

        self
    }

    pub fn get_width(&self) -> f32 { self.data.width }
    pub fn set_width(&mut self, width: f32) -> &Self {
        self.data.width = width;
        self
    }
    pub fn get_height(&self) -> f32 { self.data.height }
    pub fn set_height(&mut self, height: f32) -> &Self {
        self.data.height = height;
        self
    }

    pub fn get_x(&self) -> f32 { self.data.x }
    pub fn set_x(&mut self, x: f32) -> &Self {
        self.data.x = x;
        self
    }
    pub fn get_y(&self) -> f32 { self.data.y }
    pub fn set_y(&mut self, y: f32) -> &Self {
        self.data.y = y;
        self
    }
    pub fn get_w(&self) -> f32 { self.data.w }
    pub fn set_w(&mut self, w: f32) -> &Self {
        self.data.w = w;
        self
    }
    pub fn get_h(&self) -> f32 { self.data.h }
    pub fn set_h(&mut self, h: f32) -> &Self {
        self.data.h = h;
        self
    }
}

impl Component for Sprite {
    fn get_id(&self) -> Id {
        Id::of::<Sprite>()
    }
    fn new_component_manager(&self) -> Box<ComponentManager> {
        Box::new(SpriteManager::new())
    }
    fn get_component_manager_id(&self) -> Id {
        Id::of::<SpriteManager>()
    }
    fn get_entity(&self) -> Option<Entity> {
        self.data.entity.clone()
    }
    fn set_entity(&mut self, entity: Option<Entity>) {
        self.data.entity = entity;
    }
}

impl PartialEq<Sprite> for Sprite {
    fn eq(&self, other: &Sprite) -> bool {
        (&*self.data as *const _) == (&*other.data as *const _)
    }
    fn ne(&self, other: &Sprite) -> bool {
        !self.eq(other)
    }
}
