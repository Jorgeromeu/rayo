use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Scene {
    pub spheres: Vec<Sphere>,
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub radius: f64,
    pub center: Vec3,
}

#[derive(Debug, Clone, Copy)]
pub struct HitInfo {
    pub normal: Vec3,
    pub t: f64,
    pub point: Vec3,
    pub front_face: bool,
}

impl HitInfo {
    pub fn new(t: f64, hit_ray: &Ray, outward_normal: Vec3) -> HitInfo {
        let mut hit = HitInfo {
            front_face: false,
            point: hit_ray.at(t),
            t: t,
            normal: outward_normal
        };

        hit.set_face_normal(&hit_ray, outward_normal);

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
            let sphere_hit = sphere.intersect(ray,t_min, t_max);

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
        
        let outward_normal = ray.at(root) - Vec3::new(0.0, 0.0, -1.0).normalized();
        let hit = HitInfo::new(root, ray, outward_normal);

        Some(hit)
    }
}
