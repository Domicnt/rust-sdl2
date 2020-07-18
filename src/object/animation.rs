extern crate sdl2;
use sdl2::render::{Texture, Canvas};

pub use super::vec2D::Vec2D;

pub struct Animation<'a> {
    spr_sheet: Texture<'a>,
    //size, in pixels, to pull from texture per frame
    frame_src_size: Vec2D,
    //size of texture in pixels
    texture_src_size: Vec2D,
    //size, in pixels, to render a frame to the screen
    frame_dst_size: Vec2D,
    repeat: bool,

    src_pos: Vec2D,
    dst_pos: Vec2D,

    sound: Option<*const sdl2::mixer::Chunk>,
}

impl Animation<'_> {
    //draw the current frame, without advancing to the next
    pub fn draw (self, mut canvas: Canvas<sdl2::video::Window>) -> Result<(), String> {
        canvas.copy_ex(&self.spr_sheet, Some(sdl2::rect::Rect::new(self.src_pos.x as i32, self.src_pos.y as i32, self.frame_src_size.x as u32, self.frame_src_size.y as u32)), Some(sdl2::rect::Rect::new(self.dst_pos.x as i32, self.dst_pos.y as i32, self.frame_dst_size.x as u32, self.frame_dst_size.y as u32)), 0.0, None, false, false)?;
        Ok(())
    }

    //draw the current frame, and advance to the next; returns false if the animation has ended
    pub fn update(mut self, mut canvas: Canvas<sdl2::video::Window>) -> bool {
        self.src_pos.x += self.frame_src_size.x;
        if self.src_pos.x >= self.texture_src_size.x + self.frame_src_size.x {
            self.src_pos.x = 0.;
            self.src_pos.y += self.frame_src_size.y;
            if self.src_pos.y >= self.texture_src_size.y {
                if self.repeat {
                    self.src_pos = Vec2D {x: 0., y: 0.};
                } else {
                    return false;
                }
            }
        }
        if self.draw(canvas).is_err() {
            println!("Error in drawing animation");
        }
        true
    }

}