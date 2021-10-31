use std::ops;

use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// Elementwise addition, subtraction and multiplication
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x+rhs.x, y: self.y+rhs.y, z: self.z+rhs.z }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x-rhs.x, y: self.y-rhs.y, z: self.z-rhs.z }
    }
}

impl ops::Mul<Vec3> for Vec3 {

    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {x: self.x*rhs.x, y: self.y*rhs.y, z: self.z*rhs.z}
    }  
}

// Vector - float product and division
impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 { x: self.x*rhs, y: self.y*rhs, z: self.z*rhs }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 { x: rhs.x*self, y: rhs.y*self, z: rhs.z*self }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3 { x: self.x/rhs, y: self.y/rhs, z: self.z/rhs }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        Vec3 {x: -self.x, y: -self.y, z: -self.z}
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }    
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }    
}

impl Vec3 {

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {x, y, z}
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        let rx: f64 = rng.gen_range(min..max);
        let ry: f64 = rng.gen_range(min..max);
        let rz: f64 = rng.gen_range(min..max);
        Vec3 {x: rx, y: ry, z: rz}
    }
    
    pub fn random_unit() -> Vec3 {
        let rx: f64 = rand::random();
        let ry: f64 = rand::random();
        let rz: f64 = rand::random();
        Vec3 {x: rx, y: ry, z: rz}
    }

    pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    pub fn zero() -> Vec3 {
        Vec3 {x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn norm(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
    
    pub fn norm_sqared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn normalized(&self) -> Vec3 {
        Vec3 {x: self.x/self.norm(), y: self.y/self.norm(), z: self.z/self.norm()}
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        return (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s);
    }

    pub fn normalize(&mut self) {
        let norm = self.norm();
        *self /= norm;
    }
}