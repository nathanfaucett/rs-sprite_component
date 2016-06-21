use collections::boxed::Box;
use alloc::arc::Arc;
use core::cell::RefCell;

use scene_graph::{Entity, Component, ComponentManager, Id};
use sprite_manager::SpriteManager;


struct SpriteData {

    entity: Option<Entity>,
    sprite_manager: Option<SpriteManager>,

    visible: bool,

    layer: usize,
    z: usize,

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
    data: Arc<RefCell<SpriteData>>,
}

impl Sprite {
    pub fn new() -> Self {
        Sprite {
            data: Arc::new(RefCell::new(SpriteData {

                entity: None,
                sprite_manager: None,

                visible: true,

                layer: 0usize,
                z: 0usize,

                material: None,

                width: 1f32,
                height: 1f32,

                x: 0f32,
                y: 0f32,
                w: 1f32,
                h: 1f32,
            }))
        }
    }

    pub fn sprite_manager(&self) -> Option<SpriteManager> {
        self.data.borrow().sprite_manager.clone()
    }
    pub fn set_sprite_manager(&self, sprite_manager: Option<SpriteManager>) {
        self.data.borrow_mut().sprite_manager = sprite_manager;
    }

    pub fn layer(&self) -> usize { self.data.borrow().layer }

    pub fn z(&self) -> usize { self.data.borrow().z }

    pub fn width(&self) -> f32 { self.data.borrow().width }
    pub fn height(&self) -> f32 { self.data.borrow().height }

    pub fn x(&self) -> f32 { self.data.borrow().x }
    pub fn y(&self) -> f32 { self.data.borrow().y }
    pub fn w(&self) -> f32 { self.data.borrow().w }
    pub fn h(&self) -> f32 { self.data.borrow().h }
}

impl Component for Sprite {
    fn id(&self) -> Id {
        Id::of::<Sprite>()
    }
    fn new_component_manager(&self) -> Box<ComponentManager> {
        Box::new(SpriteManager::new())
    }
    fn component_manager_id(&self) -> Id {
        Id::of::<SpriteManager>()
    }
    fn entity(&self) -> Option<Entity> {
        self.data.borrow().entity.clone()
    }
    fn set_entity(&self, entity: Option<Entity>) {
        self.data.borrow_mut().entity = entity;
    }
}

impl PartialEq<Sprite> for Sprite {
    fn eq(&self, other: &Sprite) -> bool {
        (&*self.data.borrow() as *const _) == (&*other.data.borrow() as *const _)
    }
    fn ne(&self, other: &Sprite) -> bool {
        !self.eq(other)
    }
}
