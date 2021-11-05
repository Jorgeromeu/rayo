use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub vfov: f64,
    pub focal_length: f64,
    pub aperture: f64,
    pub aspect_ratio: f64,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        focal_length: f64,
        aperture: f64,
        aspect_ratio: f64,
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalized();
        let u = Vec3::cross(&vup, &w).normalized();
        let v = Vec3::cross(&w, &u);

        let origin = lookfrom;
        let horizontal = focal_length * viewport_width * u;
        let vertical = focal_length * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focal_length * w;

        Camera {
            origin,
            vfov,
            focal_length,
            aspect_ratio,
            horizontal,
            vertical,
            lower_left_corner,
            aperture,
            u,
            v,
        }
    }

    pub fn generate_ray(&self, s: f64, t: f64) -> Ray {
        let rd = (self.aperture / 2.0) * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray {
            t: 0.0,
            origin: self.origin + offset,
            dir: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
        }
    }
}
