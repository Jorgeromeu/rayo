use crate::color::Color;
use crate::intersection::HitInfo;
use crate::ray::Ray;
use crate::vec::Vec3;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
}

impl Material {
    pub fn scatter(self, ray_in: &Ray, hit: HitInfo) -> (Color, Ray, bool) {
        match self {
            Material::Lambertian {albedo} => {
                let mut scatter_dir = hit.normal + Vec3::random_unit();

                // catch degenerate scatter direction
                if scatter_dir.near_zero() {
                    scatter_dir = hit.normal;
                }

                let scattered_ray = Ray::new(hit.point, scatter_dir);
                let attenuation = albedo;
                (attenuation, scattered_ray, true)
            }
            Material::Metal {albedo, fuzz} => {
                
                let mut reflected = reflect(&ray_in.dir, &hit.normal).normalized();

                // fuzz the reflection
                reflected += fuzz * Vec3::random_unit();

                let scattered_ray = Ray::new(hit.point, reflected);
                let attenuation = albedo;

                let should_scatter = Vec3::dot(&scattered_ray.dir, &hit.normal) > 0.0;

                (attenuation, scattered_ray, should_scatter)
            }
        }
    }
}

fn reflect(vec: &Vec3, normal: &Vec3) -> Vec3{
    *vec - 2.0 * Vec3::dot(vec, normal) * *normal
}
