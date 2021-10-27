use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color { r: self.r+rhs.r, g: self.g+rhs.g, b: self.b+rhs.b }
    }
}

impl ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl ops::DivAssign<u32> for Color {
    fn div_assign(&mut self, rhs: u32) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Color {
        Color { r: self.r-rhs.r, g: self.g-rhs.g, b: self.b-rhs.b }
    }
}

impl ops::Mul<Color> for u32 {

    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color { r: rhs.r*self, g: rhs.g*self, b: rhs.b*self }
    }
}

impl ops::Div<u32> for Color {

    type Output = Color;

    fn div(self, rhs: u32) -> Color {
        Color { r: self.r/rhs, g: self.g/rhs, b: self.b/rhs }
    }
}

// Color

impl Color {

    pub fn new(r: u32, g: u32, b: u32) -> Color {
        Color {r, g, b}
    }

    pub fn black() -> Color {
        Color {r: 0, g: 0, b: 0}
    }

    pub fn to_pixel(&self) -> image::Rgb<u8> {
        image::Rgb([self.r as u8, self.g as u8, self.b as u8])
    }
    
    pub fn rescale(&mut self) {
        self.r = self.r.clamp(0, 255);
        self.g = self.g.clamp(0, 255);
        self.b = self.b.clamp(0, 255);
    }

}

