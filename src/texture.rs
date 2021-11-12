use crate::{color::Color, vec::Vec3};

#[derive(Debug, Clone, Copy)]
pub enum Texture {
    Constant { color: Color },
    Checker { odd: Color, even: Color }
}

impl Texture {

    pub fn value(self, u: f64, v: f64, p: Vec3) -> Color {
        match self {
            Texture::Constant { color } => {
                color
            },
            Texture::Checker { odd, even } => {
                let sines = f64::sin(10.0*p.x) * f64::sin(10.0*p.y) * f64::sin(10.0*p.z);
                if sines < 0.0 {
                    odd
                } else {
                    even
                }
            }
        }
    }
    
}