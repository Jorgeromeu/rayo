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
    pub normal: vec::Vec3,
    pub t: f64,
    pub point: vec::Vec3,
    pub front_face: bool,
}

impl HitInfo {
    pub fn new() -> HitInfo {
        HitInfo {
            is_hit: false,
            front_face: false,
            point: vec::Vec3::zero(),
            t: 0.0,
            normal: vec::Vec3::zero()
        }
    }

    pub fn set_face_normal(&mut self, ray: &ray::Ray, outward_normal: vec::Vec3) {
        self.front_face = vec::dotprod(ray.dir, outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal }
    }
}

impl Scene {
    pub fn empty() -> Scene {
        Scene { spheres: vec![] }
    }
}

pub trait Hittable {
    fn intersect(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> HitInfo;
}

impl Hittable for Scene {
    fn intersect(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> HitInfo {
        for sphere in &(self.spheres) {
            let sphere_hit_info = sphere.intersect(ray,t_min, t_max);
            if sphere_hit_info.is_hit {
                return sphere_hit_info;
            }
        }
        return HitInfo::new();
    }
}

impl Hittable for Sphere {
    fn intersect(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> HitInfo {
        let oc = ray.origin - self.center;
        let a = ray.dir.norm_sqared();
        let half_b = vec::dotprod(oc, ray.dir);
        let c = oc.norm_sqared() - self.radius.powi(2);

        let discr = half_b * half_b - a * c;


        // no hit
        if discr < 0.0 {
            return HitInfo::new();
        }

        // yes hit
        let sqrtd = discr.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return HitInfo::new();
            }
        }

        // construct hitinfo
        let mut hit_info = HitInfo::new();
        hit_info.is_hit = true;
        hit_info.t = root;
        hit_info.point = ray.at(hit_info.t);

        let outward_normal = ray.at(hit_info.t) - vec::Vec3::new(0.0, 0.0, -1.0).normalized();
        hit_info.set_face_normal(ray, outward_normal);

        hit_info
    }
}
