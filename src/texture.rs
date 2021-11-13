use crate::{color::Color, vec::Vec3};

#[derive(Debug, Clone, Copy)]
pub enum Texture {
    Constant { color: Color },
    Checkered { even: Color, odd: Color, size: f64 },
}

impl Texture {

    pub fn new_constant(r: f64, g: f64, b: f64) -> Texture {
        Texture::Constant { color: Color {r, g, b}} 
    }

    pub fn value(self, point: Vec3) -> Color {
        match self {
            Texture::Constant { color } => color,
            Texture::Checkered { even, odd , size} => {

                let sines = f64::sin(size * point.x)
                    // * f64::sin(size * point.y)
                    * f64::sin(size * point.z);
                
                if sines < 0.0 {odd} else {even}
            },
        }
    }

}
