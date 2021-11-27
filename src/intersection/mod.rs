use crate::ray::Ray;

use self::hitinfo::HitInfo;

pub mod hitinfo;
pub mod sphere;
pub mod scene;

pub trait Hittable {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo>;
}
