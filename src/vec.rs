use std::ops::{Add, Mul, Sub};
use num::{Num, Float};

#[derive(Clone, Copy, Debug)]
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

impl<T: Num + Copy> Vec3<T> {
    pub fn cross(v1: &Vec3<T>, v2: &Vec3<T>) -> Vec3<T> {
        Vec3::new(
            v1.y * v2.z - v1.z * v2.y,
            v1.z * v2.x - v1.x * v2.z,
            v1.x * v2.y - v1.y * v2.x
        )
    }

    pub fn sum(v1: &Vec3<T>, v2: &Vec3<T>) -> Vec3<T> {
        Vec3::new(v1.x + v2.x, v1.y + v2.y, v1.z + v2.z)
    }

    pub fn sub(v1: &Vec3<T>, v2: &Vec3<T>) -> Vec3<T> {
        Vec3::new(v1.x - v2.x, v1.y - v2.y, v1.z - v2.z)
    }

    pub fn dot(v1: &Vec3<T>, v2: &Vec3<T>) -> T {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

}

impl<T: Float + Copy> Vec3<T> {
    pub fn length(&self) -> T {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vec3<T> {
        let len = self.length();
        Vec3::new(self.x / len, self.y / len, self.z / len)
    }
}

impl<T: Num + Copy> Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3::sum(&self, &rhs)
    }
}

impl<T: Num + Copy> Sub for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3::sub(&self, &rhs)
    }
}

impl<T: Num + Copy> Mul for Vec3<T> {
    type Output = T;

    fn mul(self, rhs: Vec3<T>) -> T {
        Vec3::dot(&self, &rhs)
    }
}