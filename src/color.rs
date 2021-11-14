use std::ops;
use crate::ray;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color { r: self.r+rhs.r, g: self.g+rhs.g, b: self.b+rhs.b }
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Color {
        Color { r: self.r-rhs.r, g: self.g-rhs.g, b: self.b-rhs.b }
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color { r: self.r*rhs.r, g: self.g*rhs.g, b: self.b*rhs.b }
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        Color { r: self.r*rhs, g: self.g*rhs, b: self.b*rhs }
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color { r: rhs.r*self, g: rhs.g*self, b: rhs.b*self }
    }
}

impl ops::Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Color {
        Color { r: self.r/rhs, g: self.g/rhs, b: self.b/rhs }
    }
}

impl ops::AddAssign for Color {

    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
    
}

impl ops::DivAssign<f64> for Color {

    fn div_assign(&mut self, rhs: f64) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
    
}


impl Color {

    pub fn new_rgb(r: u8, g: u8, b: u8) -> Color {
        let rf = (r as f64) / 255.0;
        let gf = (g as f64) / 255.0;
        let bf = (b as f64) / 255.0;
        Color {r: rf, g: gf, b: bf}
    }

    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {r, g, b}
    }

    pub fn sky(ray: &ray::Ray) -> Color {
        let unit_dir = ray.dir.normalized();
        let t = 0.5 * (unit_dir.y + 1.0);
        (1.0-t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    pub fn white() -> Color {
        Color {r: 1.0, g: 1.0, b: 1.0}
    }
    
    pub fn black() -> Color {
        Color {r: 0.0, g: 0.0, b: 0.0}
    }

    pub fn to_pixel(&self, num_samples: u32) -> image::Rgb<u8> {

        let mut r = self.r;
        let mut g = self.g;
        let mut b = self.b;

        // Gamma correction + division
        let scale = 1.0 / (num_samples as f64);
        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        let ri = (r.clamp(0.0, 0.999) * 255.0) as u8;
        let gi = (g.clamp(0.0, 0.999) * 255.0) as u8;
        let bi = (b.clamp(0.0, 0.999) * 255.0) as u8;

        image::Rgb([ri, gi, bi])
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        return (self.r.abs() < s) && (self.g.abs() < s) && (self.b.abs() < s);
    }

    pub fn is_close(&self, vec: &Color) -> bool {
        (*self - *vec).near_zero()
    }
}
