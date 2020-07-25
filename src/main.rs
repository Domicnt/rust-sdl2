extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod object;
pub mod menu;

use object::{
    shape::Shape,
    vec2d::Vec2D,
    image::Image,
    animation::Animation,
};

fn main() -> Result<(), String> {
    
    //SDL2 setup
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(sdl2::image::InitFlag::PNG | sdl2::image::InitFlag::JPG)?;
    let window = video_subsystem.window("SDL2", 640, 480).position_centered().build().map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().accelerated().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,255));

    let poly = Shape::rectangle(Vec2D::new(100.0,100.0), Vec2D::new(50.0,50.0));
    let img = Image::new("default_assets/image.png", &texture_creator);
    let mut anim = Animation::new(Image::new("default_assets/animation.png", &texture_creator), Vec2D::new(48.0,48.0), Vec2D::new(16.0, 16.0), Vec2D::new(100.0, 100.0), true, 100, Vec2D::new(200.0, 200.0), None);

    'main: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'main,
                _ => {}
            }
        }
        if poly.draw(&mut canvas, true).is_err() {
            println!("Error drawing shape!");
        }
        if img.draw(&mut canvas, sdl2::rect::Rect::new(500,300,100,100)).is_err() {
            println!("Error drawing image!");
        }
        anim.update(&mut canvas);
        canvas.present();
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(255,255,255,255));
        canvas.clear();
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,255));
    }

    Ok(())
}