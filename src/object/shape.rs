extern crate sdl2;

pub use super::vec2D::Vec2D;

pub struct Shape {
    points: Vec<sdl2::rect::Point>,
}

impl Shape {
    //constructor
    pub fn new (points: Vec<Vec2D>) -> Shape {
        let mut temp = Shape {points: vec!()};
        for i in points {
            temp.points.push(sdl2::rect::Point::new(i.x as i32, i.y as i32));
        }
        temp
    }

    //create a square shape
    pub fn square (pos: Vec2D, dimensions: Vec2D) -> Shape {
        Shape::new(vec!(pos.clone(), pos.clone() + Vec2D::new(dimensions.x, 0.0), pos.clone() + dimensions.clone(), pos.clone() + Vec2D::new(0.0, dimensions.y)))
    }

    //draw all of the lines making up this shape
    pub fn draw (&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, connects: bool) -> Result<(), String> {
        for i in 0..self.points.len() {
            canvas.draw_line(self.points[i], self.points[(i+1)%self.points.len()])?;
            if !connects && i == self.points.len() - 1 {
                break;
            }
        }
        Ok(())
    }
}