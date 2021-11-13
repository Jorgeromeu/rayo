use crate::vec::Vec3;
use crate::material::Material;
use crate::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct HitInfo {
    pub normal: Vec3,
    pub t: f64,
    pub point: Vec3,
    pub front_face: bool,
    pub material: Material,
    pub u: f64,
    pub v: f64
}

impl HitInfo {
    pub fn new(t: f64, hit_ray: &Ray, outward_normal: Vec3, material: Material, u: f64, v: f64) -> HitInfo {
        let mut hit = HitInfo {
            front_face: false,
            point: hit_ray.at(t),
            t: t.clone(),
            normal: outward_normal,
            material,
            u, // u and v are texture coordinates
            v
        };

        hit.set_face_normal(&hit_ray, outward_normal.clone());

        hit
    }

    fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(&ray.dir, &outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal }
    }
}