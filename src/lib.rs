#![feature(alloc)]
#![no_std]


extern crate alloc;

extern crate vector;
extern crate stack;
extern crate remove;
extern crate material;
extern crate scene_graph;
extern crate shared;


mod sprite;
mod sprite_manager;

pub use sprite::Sprite;
pub use sprite_manager::SpriteManager;
