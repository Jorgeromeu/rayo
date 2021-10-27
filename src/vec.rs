use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x+_rhs.x, y: self.y+_rhs.y, z: self.z+_rhs.z }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x-_rhs.x, y: self.y-_rhs.y, z: self.z-_rhs.z }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3 { x: self.x*_rhs, y: self.y*_rhs, z: self.z*_rhs }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 { x: _rhs.x*self, y: _rhs.y*self, z: _rhs.z*self }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f64) -> Vec3 {
        Vec3 { x: self.x/_rhs, y: self.y/_rhs, z: self.z/_rhs }
    }
}

// vec3

impl Vec3 {
    pub fn norm(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    pub fn normalized(&self) -> Vec3 {
        Vec3 {x: self.x/self.norm(), y: self.y/self.norm(), z: self.z/self.norm()}
    }
}


pub fn dotprod(v1: Vec3, v2: Vec3) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}
