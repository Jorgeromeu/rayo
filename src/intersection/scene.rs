use crate::{material::Material, ray::Ray, texture::Texture, vec::Vec3};
use super::{Hittable, aab::AxisAlignedBox, hitinfo::HitInfo, sphere::Sphere};

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub aabs: Vec<AxisAlignedBox>,
}

impl Scene {
    pub fn empty() -> Scene {
        Scene { spheres: vec![], aabs: vec![] }
    }
}

impl Hittable for Scene {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {

        let mut is_hit = false;
        let mut closest_so_far = HitInfo {
            normal: Vec3::zero(),
            t: t_max,
            point: Vec3::zero(),
            front_face: false,
            material: Material::Lambertian { albedo: Texture::new_constant(0.0, 0.0, 0.0)},
            u: 0.0,
            v: 0.0
        };

        // intersect with all spheres
        for sphere in &(self.spheres) {
            let sphere_hit = sphere.intersect(ray, t_min.clone(), t_max.clone());

            // if we hit the sphere check if its the closest
            if let Some(hit) = sphere_hit {
                is_hit = true;
                if hit.t < closest_so_far.t {
                    closest_so_far = hit;
                }
            }
        }
       
        // intersect all aabs
        for aab in &(self.aabs) {
            let sphere_hit = aab.intersect(ray, t_min.clone(), t_max.clone());

            // if we hit the box, check if its the closest one
            if let Some(hit) = sphere_hit {
                is_hit = true;
                if hit.t < closest_so_far.t {
                    closest_so_far = hit;
                }
            }
        }

        if is_hit {
            Some(closest_so_far)
        } else {
            None
        }
    }
}
