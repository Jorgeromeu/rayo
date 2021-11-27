use crate::{color::Color, vec::Vec3};

#[derive(Debug, Clone, Copy)]
pub enum Texture {
    Constant {
        color: Color,
    },
    Checkered {
        even: Color,
        odd: Color,
        size: f64,
        round: f64,
    },
}

impl Texture {
    pub fn new_constant(r: f64, g: f64, b: f64) -> Texture {
        Texture::Constant {
            color: Color { r, g, b },
        }
    }

    pub fn value(self, point: Vec3) -> Color {
        match self {
            Texture::Constant { color } => color,
            Texture::Checkered {
                even,
                odd,
                size,
                round,
            } => {
                let sines = f64::sin(size * point.x) * f64::sin(size * point.z);
                if sines < round {
                    odd
                } else {
                    even
                }
            }
        }
    }
}
