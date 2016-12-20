#![no_std]


extern crate scene_graph;
extern crate material;

extern crate sprite_component;


use scene_graph::{Scene, Entity};
use material::Material;

use sprite_component::Sprite;


#[test]
fn test_scene() {
    let mut scene = Scene::new();
    let mut entity = Entity::new();
    let sprite = Sprite::new(Material::new());

    entity.add_component(sprite);
    scene.add_entity(&mut entity);

    let mut entity_sprite = entity.get_component::<Sprite>().unwrap();

    assert_eq!(entity_sprite.get_layer(), 0);
    assert_eq!(entity_sprite.get_z(), 0);

    entity_sprite.set_layer(5);
    entity_sprite.set_z(-5);

    assert_eq!(entity_sprite.get_layer(), 5);
    assert_eq!(entity_sprite.get_z(), -5);
}
