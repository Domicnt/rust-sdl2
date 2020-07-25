extern crate sdl2;

pub use super::super::object::vec2d::Vec2D;
pub use super::button::Button;

pub struct Menu<'a> {
    //top-left corner position
    pub pos: Vec2D,
    pub dimensions: Vec2D,

    pub buttons: Vec<Button<'a>>,
}

impl Menu<'_> {
    //returns 0 if no button was pressed, returns button index + 1 if one was pressed
    pub fn update(self, e: &sdl2::EventPump) -> u32 {
        for i in 0..self.buttons.len() {
            if self.buttons[i].hover(e) && e.mouse_state().is_mouse_button_pressed(sdl2::mouse::MouseButton::Left) {
                return i as u32 + 1;
            }
        }
        0
    }

    pub fn draw () {

    }
}