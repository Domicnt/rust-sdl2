extern crate sdl2;
use sdl2::render::Texture;
//use sdl2::event::Event;

pub use super::vec2D::Vec2D;

pub use super::shape::Shape;

pub struct Object<'a> {
    pos: Option<Vec2D>,
    aabb: Option<Vec2D>,

    shapes: Option<Vec<Shape>>,

    textures: Option<Vec<*const Texture<'a>>>,

    sfx: Option<Vec<*const sdl2::mixer::Chunk>>,
}

impl Object<'_> {

}