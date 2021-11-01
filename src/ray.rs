use crate::vec::Vec3;

#[derive(Debug)]
pub struct Ray {
    pub dir: Vec3,
    pub origin: Vec3,
    pub t: f64,
}

impl Ray {

    pub fn new(origin: Vec3, dir: Vec3) -> Ray {
        Ray {origin: origin, dir: dir, t: 0.0}
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.dir * t
    }
}