use std::fmt;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn swap(p1: &mut Point, p2: &mut Point) {
        let p3 = p1.clone();
        p1.x = p2.x;
        p1.y = p2.y;
        p2.x = p3.x;
        p2.y = p3.y;
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}