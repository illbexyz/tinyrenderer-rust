use std::fmt;
use std::ops::{Add, Mul, Sub};

// #[derive(Clone, Copy)]
// pub struct Vec3<T> {
//     pub x: T,
//     pub y: T,
//     pub z: T
// }

// impl<T> Vec3<T> {
//     pub fn new(x: T, y: T, z: T) -> Vec3<T> {
//         Vec3 { x, y, z }
//     }
// }

// impl<T: Mul<Output=T> + Sub<Output=T>> Vec3<T> {
//     pub fn cross(v1: &Vec3<T>, v2: &Vec3<T>) -> Vec3<T> {
//         Vec3::new(
//             v1.y * v2.z - v1.z * v2.y,
//             v1.z * v2.x - v1.x * v2.z,
//             v1.x * v2.y - v1.y * v2.x
//         )
//     }
// }

#[derive(Clone, Copy, Debug)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
        Vec3f { x, y, z }
    }

    pub fn cross(v1: &Vec3f, v2: &Vec3f) -> Vec3f {
        Vec3f::new(
            v1.y * v2.z - v1.z * v2.y,
            v1.z * v2.x - v1.x * v2.z,
            v1.x * v2.y - v1.y * v2.x
        )
    }

    pub fn sum(v1: &Vec3f, v2: &Vec3f) -> Vec3f {
        Vec3f::new(v1.x + v2.x, v1.y + v2.y, v1.z + v2.z)
    }

    pub fn sub(v1: &Vec3f, v2: &Vec3f) -> Vec3f {
        Vec3f::new(v1.x - v2.x, v1.y - v2.y, v1.z - v2.z)
    }

    pub fn dot(v1: &Vec3f, v2: &Vec3f) -> f32 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vec3f {
        let len = self.length();
        Vec3f::new(self.x / len, self.y / len, self.z / len)
    }
}

impl Add for Vec3f {
    type Output = Vec3f;

    fn add(self, rhs: Vec3f) -> Vec3f {
        Vec3f::sum(&self, &rhs)
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;

    fn sub(self, rhs: Vec3f) -> Vec3f {
        Vec3f::sub(&self, &rhs)
    }
}

impl Mul for Vec3f {
    type Output = f32;

    fn mul(self, rhs: Vec3f) -> f32 {
        Vec3f::dot(&self, &rhs)
    }
}

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