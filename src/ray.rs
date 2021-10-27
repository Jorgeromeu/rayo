use crate::vec;

#[derive(Debug)]
pub struct Ray {
    pub dir: vec::Vec3,
    pub origin: vec::Vec3,
    pub t: f64,
}


impl Ray {
    pub fn at(&self, t: f64) -> vec::Vec3 {
        self.origin + self.dir * t
    }
}