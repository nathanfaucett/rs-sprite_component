use collections::boxed::Box;
use alloc::rc::Rc;
use core::cell::RefCell;

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
    data: Rc<RefCell<SpriteData>>,
}

impl Sprite {
    pub fn new() -> Self {
        Sprite {
            data: Rc::new(RefCell::new(SpriteData {

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
    pub fn set_layer(&self, layer: usize) -> &Self {
        let old_layer = self.data.borrow().layer;

        if old_layer != layer {
            let mut sprite_manager_ref = None;

            if let Some(ref sprite_manager) = self.data.borrow().sprite_manager {
                sprite_manager_ref = Some(sprite_manager.clone());
            }

            if let Some(sprite_manager) = sprite_manager_ref {
                let component = &(Box::new(self.clone()) as Box<Component>);
                sprite_manager.remove_component(component);
                self.data.borrow_mut().layer = layer;
                sprite_manager.add_component(component);
            }
        }

        self
    }

    pub fn z(&self) -> isize { self.data.borrow().z }
    pub fn set_z(&self, z: isize) -> &Self {
        let old_z = self.data.borrow().z;

        if old_z != z {
            self.data.borrow_mut().z = z;

            let mut sprite_manager_ref = None;

            if let Some(ref sprite_manager) = self.data.borrow().sprite_manager {
                sprite_manager_ref = Some(sprite_manager.clone());
            }

            if let Some(sprite_manager) = sprite_manager_ref {
                sprite_manager.sort_layer(self.layer());
            }
        }

        self
    }

    pub fn width(&self) -> f32 { self.data.borrow().width }
    pub fn set_width(&self, width: f32) -> &Self {
        self.data.borrow_mut().width = width;
        self
    }
    pub fn height(&self) -> f32 { self.data.borrow().height }
    pub fn set_height(&self, height: f32) -> &Self {
        self.data.borrow_mut().height = height;
        self
    }

    pub fn x(&self) -> f32 { self.data.borrow().x }
    pub fn set_x(&self, x: f32) -> &Self {
        self.data.borrow_mut().x = x;
        self
    }
    pub fn y(&self) -> f32 { self.data.borrow().y }
    pub fn set_y(&self, y: f32) -> &Self {
        self.data.borrow_mut().y = y;
        self
    }
    pub fn w(&self) -> f32 { self.data.borrow().w }
    pub fn set_w(&self, w: f32) -> &Self {
        self.data.borrow_mut().w = w;
        self
    }
    pub fn h(&self) -> f32 { self.data.borrow().h }
    pub fn set_h(&self, h: f32) -> &Self {
        self.data.borrow_mut().h = h;
        self
    }
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
