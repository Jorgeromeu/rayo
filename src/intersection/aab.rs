use std::ptr::NonNull;

use crate::{material::Material, ray::Ray, vec::Vec3};
use super::{Hittable, hitinfo::HitInfo};

#[derive(Debug, Clone, Copy)]
pub struct AxisAlignedBox {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material
}

impl Hittable for AxisAlignedBox {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {

        // components of the min, max, ray.origin and ray.dir vectors
        let a_comp = self.min.components();
        let b_comp = self.max.components();
        let og_comp = ray.origin.components();
        let dir_comp = ray.dir.components();
        
        let mut t_in = 0.0;
        let mut t_out = f64::MAX;
        let mut plane_in = 0;
        let mut plane_out = 0;
        let mut axis_in = 0;
        let mut axis_out = 0;

        for ax in 0..3 {

            let t_a_ax = (a_comp[ax] - og_comp[ax]) / dir_comp[ax];
            let t_b_ax = (b_comp[ax] - og_comp[ax]) / dir_comp[ax];
            
            // keep track of which face of the axis the intersections were on
            let mut plane_in_ax = 0;
            let mut plane_out_ax = 1;
            let mut t_in_ax = t_a_ax;
            let mut t_out_ax = t_b_ax;
            if t_b_ax < t_a_ax {
                plane_in_ax = 0;
                plane_out_ax = 1;
                t_in_ax = t_b_ax;
                t_out_ax = t_a_ax;
            }

            if t_in_ax > t_in {
                t_in = t_in_ax;
                axis_in = ax;
                plane_in = plane_in_ax;
            }
            
            if t_out_ax < t_out {
                t_out = t_out_ax;
                axis_out = ax;
                plane_out = plane_out_ax;
            }
        }

        // check for ray miss
        if t_in > t_out || t_out < 0.0 {
            return None;
        }
    
        let mut t = t_in;
        let mut axis = axis_in;
        let mut plane = plane_in;
        if t_out < t_in {
            t = t_out;
            axis = axis_out;
            plane = plane_out;
        }

        // check for out of range
        if t < t_min || t_max < t {
            return None;
        }

        // TODO: replace with lookup table
        let normal = match (axis, plane) {
            (0, 0) => Vec3::new(1.0, 0.0, 0.0),
            (0, 1) => Vec3::new(-1.0, 0.0, 0.0),
            (1, 0) => Vec3::new(0.0, 1.0, 0.0),
            (1, 1) => Vec3::new(0.0, -1.0, 0.0),
            (2, 0) => Vec3::new(0.0, 0.0, 1.0),
            (2, 1) => Vec3::new(0.0, 0.0, -1.0),
            _ => panic!()
        };

        // let normal = Vec3::new(0.0, 1.0, 0.0);

        Some(HitInfo::new(t, ray, normal, self.material, 0.0, 0.0))
    }
}
