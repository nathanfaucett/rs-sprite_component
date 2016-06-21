use collections::vec::Vec;
use collections::boxed::Box;
use alloc::arc::Arc;
use core::cell::RefCell;

use scene_graph::{Component, ComponentManager, Id};
use sprite::Sprite;


struct SpriteManagerData {
    components: Vec<Sprite>,
}


#[derive(Clone)]
pub struct SpriteManager {
    data: Arc<RefCell<SpriteManagerData>>,
}

impl SpriteManager {

    pub fn new() -> SpriteManager {
        SpriteManager {
            data: Arc::new(RefCell::new(SpriteManagerData {
                components: Vec::new(),
            }))
        }
    }
}

impl ComponentManager for SpriteManager {

    fn id(&self) -> Id { Id::of::<SpriteManager>() }

    fn order(&self) -> usize { 0 }
    fn is_empty(&self) -> bool {
        self.data.borrow().components.len() == 0
    }

    fn destroy(&self) {}
    fn init(&self) {}
    fn update(&self) {}

    fn add_component(&self, component: &Box<Component>) {
        let component = component.downcast_ref::<Sprite>().unwrap();
        component.set_sprite_manager(Some(self.clone()));
        self.data.borrow_mut().components.push(component.clone());
    }
    fn remove_component(&self, component: &Box<Component>) {
        let component = component.downcast_ref::<Sprite>().unwrap();
        let ref mut components = self.data.borrow_mut().components;

        match components.iter().position(|c| *c == *component) {
            Some(i) => {
                component.set_sprite_manager(None);
                components.remove(i);
            },
            None => (),
        }
    }
}
