#![crate_name = "snowmew-render-data"]
#![license = "ASL2"]
#![crate_type = "lib"]
#![comment = "A game engine in rust"]
#![allow(dead_code)]


extern crate position = "snowmew-position";
extern crate graphics = "snowmew-graphics";

pub trait RenderData: graphics::Graphics + position::Positions {}