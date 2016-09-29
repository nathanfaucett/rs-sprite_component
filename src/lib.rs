#![feature(collections)]
#![no_std]


extern crate collections;

extern crate scene_graph;
extern crate shared;


mod sprite;
mod sprite_manager;

pub use sprite::Sprite;
pub use sprite_manager::SpriteManager;
