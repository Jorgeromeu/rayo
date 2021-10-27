use crate::ray;
use crate::vec;

pub struct Camera {
    pub origin: vec::Vec3,
    pub focal_length: f64,
    pub aspect_ratio: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    horizontal: vec::Vec3,
    vertical: vec::Vec3,
    lower_left_corner: vec::Vec3,
}

impl Camera {
    pub fn new(
        origin: vec::Vec3,
        focal_length: f64,
        aspect_ratio: f64,
        viewport_height: f64,
    ) -> Camera {
        let viewport_width = aspect_ratio * viewport_height;

        // compute horizontal and vertical vectors
        let horizontal = vec::Vec3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };

        let vertical = vec::Vec3 {
            x: 0.0,
            y: viewport_height,
            z: 0.0,
        };

        // compute lower left corner
        let lower_left_corner = origin
            - horizontal / 2.0
            - vertical / 2.0
            - vec::Vec3 {
                x: 0.0,
                y: 0.0,
                z: focal_length,
            };

        Camera {
            origin: origin,
            focal_length: focal_length,
            aspect_ratio: aspect_ratio,
            viewport_height: viewport_height,
            viewport_width: viewport_width,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
        }
    }

    pub fn generate_ray(&self, u: f64, v: f64) -> ray::Ray {
        ray::Ray {
            t: 0.0,
            origin: self.origin,
            dir: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        }
    }
}
