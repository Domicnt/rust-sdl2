extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod object;

use object::{
    shape::Shape,
    vec2D::Vec2D,
};

fn main() -> Result<(), String> {
    //SDL2 setup
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("SDL2", 640, 480).position_centered().build().map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().accelerated().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,255));

    let poly = Shape::square(Vec2D::new(100.0,100.0), Vec2D::new(50.0,50.0));

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
        canvas.present();
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(255,255,255,255));
        canvas.clear();
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,255));
    }

    Ok(())
}