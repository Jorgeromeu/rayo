use crate::vec;

#[derive(Debug)]
pub struct Ray {
    pub dir: vec::Vec3,
    pub origin: vec::Vec3,
    pub t: f64,
}

impl Ray {

    pub fn new(origin: vec::Vec3, dir: vec::Vec3) -> Ray {
        Ray {origin: origin, dir: dir, t: 0.0}
    }

    pub fn at(&self, t: f64) -> vec::Vec3 {
        self.origin + self.dir * t
    }
}