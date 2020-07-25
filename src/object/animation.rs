extern crate sdl2;
use sdl2::render::Canvas;

pub use super::{
    vec2d::Vec2D,
    image::Image,
};

pub struct Animation<'a> {
    spr_sheet: Image<'a>,
    //size of texture in pixels
    texture_src_size: Vec2D,
    //size, in pixels, to pull from texture per frame
    frame_src_size: Vec2D,
    //size, in pixels, to render a frame to the screen
    frame_dst_size: Vec2D,
    repeat: bool,

    //how many frames are skipped when this animation plays, the higher the number, the slower the animation plays, but the more choppy it may seem
    delay: i32,
    delay_counter: i32,

    src_pos: Vec2D,
    dst_pos: Vec2D,

    sound: Option<*const sdl2::mixer::Chunk>,
}

impl Animation<'_> {
    //constructor
    pub fn new<'a> (spr_sheet: Image<'a>, texture_src_size: Vec2D, frame_src_size: Vec2D, frame_dst_size: Vec2D, repeat: bool, delay: i32, dst_pos: Vec2D, sound: Option<*const sdl2::mixer::Chunk>) -> Animation {
        Animation {spr_sheet: spr_sheet, texture_src_size: texture_src_size, frame_src_size: frame_src_size, frame_dst_size: frame_dst_size, repeat: repeat, delay: delay, delay_counter: 0, src_pos: Vec2D::new(0.,0.), dst_pos: dst_pos, sound: sound}
    }

    //draw the current frame, without advancing to the next
    pub fn draw (&self, canvas: &mut Canvas<sdl2::video::Window>) -> Result<(), String> {
        self.spr_sheet.draw_part(canvas, sdl2::rect::Rect::new(self.src_pos.x as i32, self.src_pos.y as i32, self.frame_src_size.x as u32, self.frame_src_size.y as u32), sdl2::rect::Rect::new(self.dst_pos.x as i32, self.dst_pos.y as i32, self.frame_dst_size.x as u32, self.frame_dst_size.y as u32))?;
        Ok(())
    }

    //draw the current frame, and advance to the next; returns false if the animation has ended
    pub fn update(&mut self, canvas: &mut Canvas<sdl2::video::Window>) -> bool {
        self.delay_counter = (self.delay_counter + 1) % (self.delay + 1);
        if self.delay_counter >= self.delay {
            self.src_pos.x += self.frame_src_size.x;
            if self.src_pos.x >= self.texture_src_size.x - self.frame_src_size.x {
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
        }
        if self.draw(canvas).is_err() {
            println!("Error in drawing animation");
        }
        true
    }

}