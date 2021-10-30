use crate::color::Color;
use crate::intersection::HitInfo;
use crate::ray::Ray;
use crate::vec::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub albedo: Color,
}

impl Material {
    pub fn scatter(
        self,
        _ray_in: &Ray,
        hit: HitInfo,
        attenuation: &mut Color,
        scattered_ray: &mut Ray,
    ) -> bool {
        let mut scatter_dir = hit.normal + Vec3::random_unit();

        // catch degenerate scatter direction
        if scatter_dir.near_zero() {
            scatter_dir = hit.normal;
        }

        *scattered_ray = Ray::new(hit.point, scatter_dir);
        *attenuation = self.albedo;
        true
    }
}

// impl Material for Metal {

//     fn scatter(self, ray_in: &Ray, hit: HitInfo, attenuation: &mut Color, scattered_ray: &mut Ray) -> bool {

//         // calculate the reflected ray
//         let reflection_dir = ray_in.dir.reflected(hit.normal);

//         *scattered_ray = Ray::new(hit.point, reflection_dir);
//         *attenuation = self.albedo;

//         // only reflect if ???
//         dotprod(scattered_ray.dir, hit.normal) > 0.0
//     }
// }
