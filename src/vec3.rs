use rand::prelude::*;

use std::{cmp, ops};


// TODO: make this generic over f32 and f64
#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub type Point = Vec3;


impl Vec3 {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen::<f32>(),
            y: rng.gen::<f32>(),
            z: rng.gen::<f32>(),
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let v = 2.0 * Vec3::random() - 1.0;
            if v.length_squared() >= 1.0 { continue; } else { return v; }
        }
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let v = Vec3 {
                x: rng.gen::<f32>(),
                y: rng.gen::<f32>(),
                z: 0.0,
            };
            if v.length_squared() >= 1.0 { continue; } else { return v; }
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self{
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length_squared(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(&self) -> f32 {
       self.length_squared().sqrt()
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        let e = 1e-8;
        self.x.abs() < e && self.y.abs() < e && self.z.abs() < e
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Self{x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z}
    }
}

impl ops::Add<Vec3> for f32 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3{x: self + _rhs.x, y: self + _rhs.y, z: self + _rhs.z}
    }
}

impl ops::Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, _rhs: f32) -> Self {
        Self{x: self.x + _rhs, y: self.y + _rhs, z: self.z + _rhs}
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Self{x: self.x - _rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z}
    }
}

impl ops::Sub<f32> for Vec3 {
    type Output = Self;

    fn sub(self, _rhs: f32) -> Self {
        Self{x: self.x - _rhs, y: self.y - _rhs, z: self.z - _rhs}
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, _rhs: Self) -> Self {
        Self{x: self.x * _rhs.x, y: self.y * _rhs.y, z: self.z * _rhs.z}
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3{x: self * _rhs.x, y: self * _rhs.y, z: self * _rhs.z}
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, _rhs: f32) -> Self {
        Self{x: self.x * _rhs, y: self.y * _rhs, z: self.z * _rhs}
    }
}

impl ops::Div for Vec3 {
    type Output = Self;

    fn div(self, _rhs: Self) -> Self {
        Self{x: self.x / _rhs.x, y: self.y / _rhs.y, z: self.z / _rhs.z}
    }
}

impl ops::Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, _rhs: Vec3) -> Vec3 {
        Vec3{x: self / _rhs.x, y: self / _rhs.y, z: self / _rhs.z}
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, _rhs: f32) -> Self {
        Self{x: self.x / _rhs, y: self.y / _rhs, z: self.z / _rhs}
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self{x: -self.x, y: -self.y, z: -self.z}
    }
}

impl cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        let e = 1e-8;
        (self.x - other.x).abs() < e && (self.y - other.y).abs() < e && (self.z - other.z).abs() < e
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_const() {
        let v0 = Vec3{x: 1.0, y: 2.0, z: 3.0};
        let v1 = 1.0 + v0;
        // assert!(v1.x == 2.0 && v1.y == 3.0 && v1.z == 4.0);
        assert_eq!(v1, Vec3{x: 2.0, y: 3.0, z: 4.0});
    }
}
