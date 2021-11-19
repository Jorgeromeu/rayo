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
        let min_comp = self.min.components();
        let max_comp = self.max.components();
        let og_comp = ray.origin.components();
        let dir_comp = ray.dir.components();

        // t and axis value for box in and out point
        let mut tin = f64::MIN;
        let mut tout = f64::MAX;
        
        let mut axis_in = 0;
        let mut axis_out = 0;

        // for each axis a
        for a in 0..3 {

            // get t-value for intersection with the min/max planes on axis a
            let tmin_a = (min_comp[a] - og_comp[a]) / dir_comp[a];
            let tmax_a = (max_comp[a] - og_comp[a]) / dir_comp[a];

            // figure out which one of the points is the in and out point
            let tin_a = f64::min(tmin_a, tmax_a);
            let tout_a = f64::max(tmin_a, tmax_a);

            // figure out global t in/out as well as the dimension of in/out
            if tin_a > tin {
                tin = tin_a;
                axis_in = a;
            }

            if tout_a < tout {
                tout = tout_a;
                axis_out = a;
            }
        }

        // check for ray miss
        if tin > tout || tout < 0.0 {
            return None;
        }

        // get t-value as well as the axi
        let (t, hit_axis) = if tin < tout {
            (tin, axis_in)
        } else {
            (tout, axis_out)
        };

        // check for t in valid range
        if t < t_min || t > t_max {
            return None;
        }

        let normal = match (true, hit_axis) {
            (true, 0) => Vec3::new(0.0, 0.0, 1.0),
            (true, 1) => Vec3::new(1.0, 0.0, 0.0),
            (true, 2) => Vec3::new(0.0, 1.0, 0.0),
            (false, 0) => Vec3::new(0.0, 0.0, -1.0),
            (false, 1) => Vec3::new(-1.0, 0.0, 0.0),
            (false, 2) => Vec3::new(0.0, -1.0, 0.0),
            _ => panic!()
        };

        // return the hit
        let hit = HitInfo::new(t, ray, normal, self.material, 0.0, 0.0);
        Some(hit)
    }
}
