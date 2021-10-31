use crate::color::Color;
use crate::intersection::HitInfo;
use crate::ray::Ray;
use crate::vec::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub albedo: Color,
}

impl Material {
    pub fn scatter(self, _ray_in: &Ray, hit: HitInfo) -> (Color, Ray) {
        let mut scatter_dir = hit.normal + Vec3::random_unit();

        // catch degenerate scatter direction
        if scatter_dir.near_zero() {
            scatter_dir = hit.normal;
        }

        let scattered_ray = Ray::new(hit.point, scatter_dir);
        let attenuation = self.albedo;
        (attenuation, scattered_ray)
    }
}
