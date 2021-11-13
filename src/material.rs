use image::buffer::EnumeratePixels;

use crate::color::Color;
use crate::intersection::hitinfo::HitInfo;
use crate::ray::Ray;
use crate::vec::Vec3;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { ior: f64, color: Color },
    Checkered { odd: Color, even: Color}
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
            Material::Dielectric { ior, color} => {
                let refraction_ratio = if hit.front_face { 1.0/ior } else { ior };
                let unit_dir = ray_in.dir.normalized();

                // check for total internal reflection
                let costheta = f64::min(Vec3::dot(&(-unit_dir), &hit.normal), 1.0);
                let sintheta = (1.0 - costheta*costheta).sqrt();

                let cannot_refract = refraction_ratio * sintheta > 1.0;
                let reflectance_high = reflectance(costheta, refraction_ratio) > rand::random();

                let scatter_dir = if cannot_refract || reflectance_high {
                    reflect(&unit_dir, &hit.normal)
                } else {
                    refract(&unit_dir, &hit.normal, refraction_ratio)
                };

                let scattered = Ray::new(hit.point, scatter_dir);

                (color, scattered, true) 
            },
            Material::Checkered { odd, even } => {

                let mut scatter_dir = hit.normal + Vec3::random_unit();

                // catch degenerate scatter direction
                if scatter_dir.near_zero() {
                    scatter_dir = hit.normal;
                }

                let scattered_ray = Ray::new(hit.point, scatter_dir);

                let size = 10.0;

                let sines = f64::sin(size * hit.point.x) * f64::sin(size * hit.point.y) * f64::sin(size * hit.point.z);
                let attenuation = if sines < 0.0 {odd} else {even};

                (attenuation, scattered_ray, true)
            },
        }
    }
}

fn reflect(vec: &Vec3, normal: &Vec3) -> Vec3{
    *vec - 2.0 * Vec3::dot(vec, normal) * *normal
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0-ref_idx) / (1.0+ref_idx)).powi(2);
    r0 + (1.0-r0)*(1.0 - cosine).powi(5)
}

fn refract(vec: &Vec3, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
    let costheta = f64::min(1.0, Vec3::dot(&(-(*vec)), normal));
    let r_out_perp = etai_over_etat * (*vec + costheta * *normal);
    let r_out_parallel = -(1.0 - r_out_perp.norm_sqared()).sqrt() * *normal;
    r_out_perp + r_out_parallel
}
