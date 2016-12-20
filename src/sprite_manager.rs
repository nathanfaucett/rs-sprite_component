use alloc::boxed::Box;

use vector::Vector;
use stack::Stack;
use remove::Remove;
use shared::Shared;
use scene_graph::{Scene, Component, ComponentManager, Id};

use sprite::Sprite;


struct SpriteManagerData {
    scene: Option<Scene>,
    count: usize,
    layers: Vector<Vector<Sprite>>,
}


#[derive(Clone)]
pub struct SpriteManager {
    data: Shared<SpriteManagerData>,
}

impl SpriteManager {

    pub fn new() -> SpriteManager {
        let mut layers = Vector::new();
        layers.push(Vector::new());

        SpriteManager {
            data: Shared::new(SpriteManagerData {
                scene: None,
                count: 0usize,
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

    pub fn get_components(&self) -> Vector<&Sprite> {
        let mut out = Vector::with_capacity(self.data.count);
        let ref layers = self.data.layers;

        for layer in layers.iter() {
            for sprite in layer.iter() {
                out.push(sprite);
            }
        }

        out
    }
    pub fn get_components_mut(&mut self) -> Vector<&mut Sprite> {
        let mut out = Vector::with_capacity(self.data.count);
        let ref mut layers = self.data.layers;

        for layer in layers.iter_mut() {
            for sprite in layer.iter_mut() {
                out.push(sprite);
            }
        }

        out
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
                    layers.push(Vector::new());
                }
            }

            layers[layer].push(component.clone());
        }

        self.data.count += 1;
        self.sort_layers();
    }
    fn remove_component(&mut self, component: &mut Box<Component>) {
        let component = component.downcast_mut::<Sprite>().unwrap();
        let mut count = self.data.count;
        {
            let ref mut layer = self.data.layers[component.get_layer()];

            match layer.iter().position(|c| *c == *component) {
                Some(i) => {
                    count -= 1;
                    component.set_sprite_manager(None);
                    layer.remove(&i);
                },
                None => (),
            }
        }

        self.data.count = count;
    }
}
