use std::ops::{Add, Div, Mul, Sub};
#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}
impl GridPos {
    pub fn new(x: i32, y: i32) -> Self {
        GridPos { x: x, y: y }
    }
}
impl Add for GridPos {
    type Output = GridPos;

    fn add(self, other: GridPos) -> GridPos {
        GridPos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for GridPos {
    type Output = GridPos;

    fn sub(self, other: GridPos) -> GridPos {
        GridPos {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i32> for GridPos {
    type Output = GridPos;

    fn mul(self, scalar: i32) -> GridPos {
        GridPos {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Mul<f32> for GridPos {
    type Output = GridPos;
    fn mul(self, scalar: f32) -> GridPos {
        GridPos {
            x: (self.x as f32 * scalar) as i32,
            y: (self.y as f32 * scalar) as i32,
        }
    }
}

impl Div<i32> for GridPos {
    type Output = GridPos;

    fn div(self, scalar: i32) -> GridPos {
        GridPos {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SubGridPos {
    pub x: f32,
    pub y: f32,
}
