use std::ptr::NonNull;

use crate::color::Color;
use crate::intersection::hitinfo::HitInfo;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec::Vec3;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Texture },
    Metal { albedo: Texture, fuzz: f64 },
    Dielectric { ior: f64, color: Texture },
    DiffuseLight { texture: Texture },
    Debug
}

impl Material {
    pub fn scatter(self, ray_in: &Ray, hit: HitInfo) -> Option<(Color, Ray)> {
        match self {
            Material::Lambertian {albedo} => {
                let mut scatter_dir = hit.normal + Vec3::random_unit();

                // catch degenerate scatter direction
                if scatter_dir.near_zero() {
                    scatter_dir = hit.normal;
                }

                let scattered_ray = Ray::new(hit.point, scatter_dir);
                let attenuation = albedo.value(hit.point);
                Some((attenuation, scattered_ray))
            }
            Material::Metal {albedo, fuzz} => {
                
                let mut reflected = reflect(&ray_in.dir, &hit.normal).normalized();

                // fuzz the reflection
                reflected += fuzz * Vec3::random_unit();

                let scattered_ray = Ray::new(hit.point, reflected);
                let attenuation = albedo.value(hit.point);

                let should_scatter = Vec3::dot(&scattered_ray.dir, &hit.normal) > 0.0;

                if should_scatter {
                    Some((attenuation, scattered_ray))
                } else {
                    None
                }
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
                let attenuation = color.value(hit.point);

                Some((attenuation, scattered))
            }
            Material::DiffuseLight { texture: _ } => None,
            Material::Debug => None
        }
    }
    pub fn emmit(self, _u: f64, _v: f64, p: Vec3) -> Color {
        match self {
            Material::DiffuseLight { texture }=> {
                texture.value(p)
            },
            _ => Color::black()
        }
    }

}


// private helper functions

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
