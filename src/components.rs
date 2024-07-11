use specs::*;
use specs_derive::Component;

#[allow(unused)]
#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum BodyType {
    Static,
    Kinematic,
    Dynamic,
}

#[derive(Component, Clone)]
pub struct Tile {
    pub uv: [f32; 4],
    pub atlas: String,
}

#[derive(Component, Clone)]
pub struct Transform {
    pub position: [f32; 3],
    pub size: [f32; 2],
    pub rotation: f32,
}

#[derive(Component, Clone, Default)]
pub struct Cell {
    pub index: u32,
    pub alive: bool,
}
