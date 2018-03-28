use std::fmt;
// use std::ops::{Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }
}

// impl<T: Mul> Vec3<T> {
//     pub fn cross(v1: &mut Vec3<T>, v2: &mut Vec3<T>) -> Vec3<T> {
//         Vec3::new(
//             v1.y * v2.z ,
//             v1.z * v2.x ,
//             v1.x * v2.y 
//         )
//     }
// }

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