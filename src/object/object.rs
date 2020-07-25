extern crate sdl2;
use sdl2::render::Texture;
//use sdl2::event::Event;

pub use super::vec2d::Vec2D;

pub use super::shape::Shape;

pub struct Object<'a> {
    pub pos: Option<Vec2D>,

    pub shapes: Option<Vec<Shape>>,

    pub textures: Option<Vec<*const Texture<'a>>>,

    pub sfx: Option<Vec<*const sdl2::mixer::Chunk>>,
}

impl Object<'_> {
    //constructors
    pub fn new<'a>(pos: Vec2D, shape: Shape) -> Object<'a> {
        Object {pos: Some(pos), shapes: Some(vec!(shape)), textures: None, sfx: None}
    }
}