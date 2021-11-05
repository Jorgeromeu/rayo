use crate::{material::Material, ray::Ray, vec::Vec3};
use super::{Hittable, hitinfo::HitInfo};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub radius: f64,
    pub center: Vec3,
    pub material: Material
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
