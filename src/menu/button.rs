extern crate sdl2;

pub use super::super::object::{
    vec2d::Vec2D,
    image::Image,
};

pub struct Button<'a> {
    //top-left corner position
    pub pos: Vec2D,
    pub dimensions: Vec2D,

    pub img: Option<Image<'a>>,
}

impl Button<'_> {
    pub fn hover(&self, e: &sdl2::EventPump) -> bool {
        let pos = Vec2D::new(e.mouse_state().x() as f64, e.mouse_state().y() as f64);
        if pos > self.pos.clone() && pos < self.pos.clone() + self.dimensions.clone() {
            return true;
        }
        false
    }
    pub fn draw(self, mut canvas: sdl2::render::Canvas<sdl2::video::Window>) {
        if self.img.is_none() {
            if canvas.draw_rect(sdl2::rect::Rect::new(self.pos.x as i32, self.pos.y as i32, self.dimensions.x as u32, self.dimensions.y as u32)).is_err() {
                println!("Error drawing button!");
            }
        } else {
            if self.img.expect("").draw( &mut canvas, sdl2::rect::Rect::new(self.pos.x as i32, self.pos.y as i32, self.dimensions.x as u32, self.dimensions.y as u32)).is_err() {
                println!("Error drawing button!");
            }
        }
    }
}