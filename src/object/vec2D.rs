pub struct Vec2D {
    pub x: f64,
    pub y: f64,
}

impl Vec2D {
    //constructor
    pub fn new(x: f64, y: f64) -> Vec2D {
        Vec2D {x: x, y: y}
    }
}

impl Clone for Vec2D {
    fn clone(&self) -> Vec2D {
        Vec2D::new(self.x, self.y)
    }
}

impl std::ops::Add<Vec2D> for Vec2D {
    type Output = Vec2D;
    fn add(self, other: Vec2D) -> Vec2D {
        Vec2D {x: self.x + other.x, y: self.y + other.y}
    }
}

impl std::ops::Sub<Vec2D> for Vec2D {
    type Output = Vec2D;
    fn sub(self, other: Vec2D) -> Vec2D {
        Vec2D {x: self.x - other.x, y: self.y - other.y}
    }
}

impl std::ops::Mul<f64> for Vec2D {
    type Output = Vec2D;
    fn mul(self, other: f64) -> Vec2D {
        Vec2D {x: self.x * other, y: self.y * other}
    }
}

impl std::ops::Mul<Vec2D> for Vec2D {
    type Output = f64;
    fn mul(self, other: Vec2D) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

impl std::ops::Div<f64> for Vec2D {
    type Output = Vec2D;
    fn div(self, other: f64) -> Vec2D {
        Vec2D {x: self.x / other, y: self.y / other}
    }
}

impl std::cmp::PartialEq for Vec2D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl std::cmp::PartialOrd for Vec2D {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.eq(other) {return Some(std::cmp::Ordering::Equal)}
        else if self.x > other.x && self.y > other.y {return Some(std::cmp::Ordering::Greater)}
        Some(std::cmp::Ordering::Less)
    }
}