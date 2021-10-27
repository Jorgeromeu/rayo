use crate::vec;

#[derive(Debug)]
pub struct Ray {
    pub direction: vec::Vec3,
    pub origin: vec::Vec3,
    pub t: f64,
}
