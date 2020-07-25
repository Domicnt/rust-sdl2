extern crate sdl2;
use sdl2::image::LoadTexture;

pub struct Image<'a> {
    pub texture: sdl2::render::Texture<'a>,
}

impl Image<'_> {
    //constructor
    pub fn new<'a> (path: &str, texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Image<'a> {
        match &path[path.len()-4..] {
            ".bmp" | ".png" | ".jpg" => return Image {texture: texture_creator.load_texture(std::path::Path::new(path)).expect(format!("Couldn't load texture: {}", path).as_str())},
            _ => {
                println!("Unrecognized file type: {}, loading default image.", &path[path.len()-4..]);
                return Image {texture: texture_creator.load_texture(std::path::Path::new("default_assets/image.png")).expect("Default image could not be found!")};
            }
        }
    }

    //draw the image at a specified location
    pub fn draw (&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, dst_rect: sdl2::rect::Rect) -> Result<(), String> {
        canvas.copy(&self.texture, None, Some(dst_rect))?;
        Ok(())
    }

    //draw a specified part of the image at a specified location
    pub fn draw_part (&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, src_rect: sdl2::rect::Rect, dst_rect: sdl2::rect::Rect) -> Result<(), String> {
        canvas.copy(&self.texture, Some(src_rect), Some(dst_rect))?;
        Ok(())
    }
}