extern crate image;
use image::{ImageBuffer, RgbImage};

mod vec;

fn get_final_color(x: u32, y: u32) -> image::Rgb<u8> {
    image::Rgb([255, 255, 0])
}

fn main() {

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMG_X: u32 = 500;
    const IMG_Y: u32 = (IMG_X as f64 / ASPECT_RATIO) as u32;

    let mut img: RgbImage = ImageBuffer::new(IMG_X, IMG_Y);

    for x in 0..IMG_X {
        println!("Lines remaining: {}", IMG_X - x);
        for y in 0..IMG_Y {
            let pixel = img.get_pixel_mut(x, y);
            *pixel = get_final_color(x, y);
        }
    }

    img.save("img.png").unwrap();
}
