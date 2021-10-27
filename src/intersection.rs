use crate::ray;
use crate::vec;

pub struct Scene {
    pub spheres: Vec<Sphere>,
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub radius: f64,
    pub center: vec::Vec3,
}

#[derive(Debug, Clone, Copy)]
pub struct HitInfo {
    pub is_hit: bool,
    pub front_face: bool,
    pub t: f64,
    pub normal: vec::Vec3,
}

pub fn empty_hit_info() -> HitInfo {
    HitInfo {
        is_hit: false,
        front_face: false,
        t: 0.0,
        normal: vec::Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    }
}

pub fn empty_scene() -> Scene {
    Scene {
        spheres: vec![]        
    }
}

pub trait Hittable {
    fn intersect(&self, ray: &ray::Ray) -> HitInfo;
}

impl Hittable for Scene {
    fn intersect(&self, ray: &ray::Ray) -> HitInfo {

        for sphere in &(self.spheres) {
            let sphere_hit_info = sphere.intersect(ray);
            if sphere_hit_info.is_hit {
                return sphere_hit_info;
            }
        }

        return empty_hit_info();
    }
}

impl Hittable for Sphere {
    fn intersect(&self, ray: &ray::Ray) -> HitInfo {
        let oc = ray.origin - self.center;
        let a = ray.dir.norm_sqared();
        let half_b = vec::dotprod(oc, ray.dir);
        let c = oc.norm_sqared() - self.radius.powi(2);
        let discr = half_b * half_b - a * c;

        let mut hit_info = empty_hit_info();

        // if discriminant is less than zero we have a hit
        if discr > 0.0 {
            hit_info.is_hit = true;
            hit_info.t = (-half_b - discr.sqrt()) / a;
            hit_info.normal = (ray.at(hit_info.t)
                - vec::Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                })
            .normalized();
        }

        hit_info
    }
}
