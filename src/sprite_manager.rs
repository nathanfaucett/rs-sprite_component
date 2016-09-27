use collections::vec::Vec;
use collections::boxed::Box;
use alloc::rc::Rc;
use core::cell::RefCell;

use scene_graph::{Scene, Component, ComponentManager, Id};
use sprite::Sprite;


struct SpriteManagerData {
    scene: Option<Scene>,
    layers: Vec<Vec<Sprite>>,
}


#[derive(Clone)]
pub struct SpriteManager {
    data: Rc<RefCell<SpriteManagerData>>,
}

impl SpriteManager {

    pub fn new() -> SpriteManager {
        let mut layers = Vec::new();
        layers.push(Vec::new());

        SpriteManager {
            data: Rc::new(RefCell::new(SpriteManagerData {
                scene: None,
                layers: layers,
            }))
        }
    }

    pub fn sort_layer(&self, layer: usize) {
        let ref mut layers = self.data.borrow_mut().layers;

        if layer < layers.len() {
            layers[layer].sort_by(|a, b| a.z().cmp(&b.z()));
        }
    }
    pub fn sort_layers(&self) {
        let ref mut layers = self.data.borrow_mut().layers;

        for layer in layers.iter_mut() {
            layer.sort_by(|a, b| a.z().cmp(&b.z()));
        }
    }
}

impl ComponentManager for SpriteManager {

    fn id(&self) -> Id { Id::of::<SpriteManager>() }

    fn scene(&self) -> Option<Scene> {
        match self.data.borrow().scene {
            Some(ref scene) => Some(scene.clone()),
            None => None,
        }
    }
    fn set_scene(&self, scene: Option<Scene>) {
        self.data.borrow_mut().scene = scene;
    }

    fn order(&self) -> usize { 0 }
    fn is_empty(&self) -> bool {
        for layer in self.data.borrow().layers.iter() {
            if layer.is_empty() {
                return true;
            }
        }
        false
    }

    fn destroy(&self) {}
    fn init(&self) {}
    fn update(&self) {}

    fn add_component(&self, component: &Box<Component>) {
        let component = component.downcast_ref::<Sprite>().unwrap();
        let layer = component.layer();

        component.set_sprite_manager(Some(self.clone()));

        {
            let ref mut layers = self.data.borrow_mut().layers;
            let len = layers.len();

            if layer > len {
                for _ in len..(layer + 1) {
                    layers.push(Vec::new());
                }
            }

            layers[layer].push(component.clone());
        }

        self.sort_layers();
    }
    fn remove_component(&self, component: &Box<Component>) {
        let component = component.downcast_ref::<Sprite>().unwrap();
        let ref mut layer = self.data.borrow_mut().layers[component.layer()];

        match layer.iter().position(|c| *c == *component) {
            Some(i) => {
                component.set_sprite_manager(None);
                layer.remove(i);
            },
            None => (),
        }
    }
}
