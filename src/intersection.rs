use crate::ray::Ray;
use crate::vec::Vec3;
use crate::material::Material;

pub struct Scene {
    pub spheres: Vec<Sphere>,
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub radius: f64,
    pub center: Vec3,
    pub material: Material
}

#[derive(Debug, Clone, Copy)]
pub struct HitInfo {
    pub normal: Vec3,
    pub t: f64,
    pub point: Vec3,
    pub front_face: bool,
    pub material: Material
}

impl HitInfo {
    pub fn new(t: f64, hit_ray: &Ray, outward_normal: Vec3, material: Material) -> HitInfo {
        let mut hit = HitInfo {
            front_face: false,
            point: hit_ray.at(t),
            t: t.clone(),
            normal: outward_normal,
            material: material
        };

        hit.set_face_normal(&hit_ray, outward_normal.clone());

        hit
    }

    fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(&ray.dir, &outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal }
    }
}

impl Scene {
    pub fn empty() -> Scene {
        Scene { spheres: vec![] }
    }
}

pub trait Hittable {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo>;
}

impl Hittable for Scene {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {
        for sphere in &(self.spheres) {
            let sphere_hit = sphere.intersect(ray, t_min.clone(), t_max.clone());

            // if we hit the sphere return its hit info
            match sphere_hit {
                Some(_hit) => return sphere_hit,
                None => (),
            }
        }
        None
    }
}

impl Hittable for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {
        let oc = ray.origin - self.center;
        let a = ray.dir.norm_sqared();
        let half_b = Vec3::dot(&oc, &ray.dir);
        let c = oc.norm_sqared() - self.radius.powi(2);

        // compute discriminant
        let discr = half_b * half_b - a * c;

        // no hit
        if discr < 0.0 {
            return None;
        } 

        // yes hit
        let sqrtd = discr.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        
        let outward_normal = (ray.at(root) - self.center).normalized();
        let hit = HitInfo::new(root, ray, outward_normal, self.material);

        Some(hit)
    }
}

mod tests {
    use crate::{color::Color, material::Material};
    use super::*;

    #[test]
    fn test_sphere_hit() {
        let s = Sphere {
            radius: 0.5,
            center: Vec3::new(0.0, 0.0, -1.0),
            material: Material {albedo: Color::black()}
        };

        let ray = Ray::new(Vec3::zero(), Vec3::new(0.0, 0.0, -1.0));
        let hit = s.intersect(&ray, 0.001, f64::MAX).unwrap();
  
        // normal should be normalized
        assert_eq!(hit.normal.norm(), 1.0);

        // normal should face towards camera
        assert!(hit.normal.is_close(&Vec3::new(0.0, 0.0, 1.0)));

        // intersectionpoint should be at -0.5
        assert!(hit.point.is_close(&Vec3::new(0.0, 0.0, -0.5)));
    }
}
