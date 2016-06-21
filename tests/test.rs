#![no_std]

extern crate sprite_component;
extern crate scene_graph;


use sprite_component::Sprite;
use scene_graph::{Scene, Entity};


#[test]
fn test_scene() {
    let scene = Scene::new();
    let entity = Entity::new();
    let sprite = Sprite::new();

    entity.add_component(sprite);
    scene.add_entity(entity.clone());

    let entity_sprite = entity.get_component::<Sprite>().unwrap();

    assert!(entity_sprite.layer() == 0);
    assert!(entity_sprite.z() == 0);
}
