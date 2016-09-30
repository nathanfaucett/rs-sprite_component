use alloc::boxed::Box;
use collections::vec::Vec;

use shared::Shared;
use scene_graph::{Scene, Component, ComponentManager, Id};

use sprite::Sprite;


struct SpriteManagerData {
    scene: Option<Scene>,
    layers: Vec<Vec<Sprite>>,
}


#[derive(Clone)]
pub struct SpriteManager {
    data: Shared<SpriteManagerData>,
}

impl SpriteManager {

    pub fn new() -> SpriteManager {
        let mut layers = Vec::new();
        layers.push(Vec::new());

        SpriteManager {
            data: Shared::new(SpriteManagerData {
                scene: None,
                layers: layers,
            })
        }
    }

    pub fn sort_layer(&mut self, layer: usize) {
        let ref mut layers = self.data.layers;

        if layer < layers.len() {
            layers[layer].sort_by(|a, b| a.get_z().cmp(&b.get_z()));
        }
    }
    pub fn sort_layers(&mut self) {
        let ref mut layers = self.data.layers;

        for layer in layers.iter_mut() {
            layer.sort_by(|a, b| a.get_z().cmp(&b.get_z()));
        }
    }
}

impl ComponentManager for SpriteManager {

    fn get_id(&self) -> Id { Id::of::<SpriteManager>() }

    fn get_scene(&self) -> Option<Scene> {
        match self.data.scene {
            Some(ref scene) => Some(scene.clone()),
            None => None,
        }
    }
    fn set_scene(&mut self, scene: Option<Scene>) {
        self.data.scene = scene;
    }

    fn get_order(&self) -> usize { 0 }
    fn is_empty(&self) -> bool {
        for layer in self.data.layers.iter() {
            if layer.is_empty() {
                return true;
            }
        }
        false
    }

    fn clear(&mut self) {}
    fn init(&mut self) {}
    fn update(&mut self) {}

    fn add_component(&mut self, component: &mut Box<Component>) {
        let component = component.downcast_mut::<Sprite>().unwrap();
        let layer = component.get_layer();

        component.set_sprite_manager(Some(self.clone()));

        {
            let ref mut layers = self.data.layers;
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
    fn remove_component(&mut self, component: &mut Box<Component>) {
        let component = component.downcast_mut::<Sprite>().unwrap();
        let ref mut layer = self.data.layers[component.get_layer()];

        match layer.iter().position(|c| *c == *component) {
            Some(i) => {
                component.set_sprite_manager(None);
                layer.remove(i);
            },
            None => (),
        }
    }
}
