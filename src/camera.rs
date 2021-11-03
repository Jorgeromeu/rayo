use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub vfov: f64,
    pub focal_length: f64,
    pub aspect_ratio: f64,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(
        origin: Vec3,
        vfov: f64,
        focal_length: f64,
        aspect_ratio: f64,
    ) -> Camera {

        let theta = vfov.to_radians();
        let h = (theta/2.0).tan();
        
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        // compute horizontal and vertical vectors
        let horizontal = Vec3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };

        let vertical = Vec3 {
            x: 0.0,
            y: viewport_height,
            z: 0.0,
        };

        // compute lower left corner
        let lower_left_corner = origin
            - horizontal / 2.0
            - vertical / 2.0
            - Vec3 {
                x: 0.0,
                y: 0.0,
                z: focal_length,
            };

        Camera {
            origin,
            vfov,
            focal_length,
            aspect_ratio,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn generate_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            t: 0.0,
            origin: self.origin,
            dir: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        }
    }
}
