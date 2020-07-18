extern crate sdl2;

pub struct Image<'a> {
    texture: sdl2::render::Texture<'a>,
}

impl Image {
    //load an image at a specified path
    pub fn load_image (mut self, &str path) -> Result<(), String)> {
        let temp_surface = sdl2::surface::Surface::load_bmp(std::path::Path::new(path))?;
        texture = texture_creator.create_texture_from_surface(&temp_surface).map_err(|e| e.to_string())?;
        Ok(())
    }

    //draw the image at a specified location
    pub fn draw (self, mut canvas: sdl2::render::Canvas<sdl2::video::Window>, dstrect: sdl2::rect::Rect) -> Result<(), String> {
        canvas.copy(&self.texture, None, Some(dstrect))?;
        Ok(())
    }
}