extern crate image;
use image::{ImageBuffer, RgbImage};

mod vec;
mod ray;

fn get_final_color(ray: &ray::Ray) -> image::Rgb<u8> {
    image::Rgb([255, 255, 0])
}

fn main() {

    // image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMG_X: u32 = 500;
    const IMG_Y: u32 = (IMG_X as f64 / ASPECT_RATIO) as u32;

    // camera
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    const ORIGIN: vec::Vec3 = vec::Vec3 {x: 0.0, y: 0.0, z: 0.0};
    const HORIZONTAL: vec::Vec3 = vec::Vec3 {x: VIEWPORT_WIDTH, y:0.0, z: 0.0};
    const VERTICAL: vec::Vec3 = vec::Vec3 {x: 0.0, y: VIEWPORT_HEIGHT, z: 0.0};

    let lower_left_corner = ORIGIN - HORIZONTAL/2.0 - VERTICAL/2.0 - vec::Vec3{x: 0.0, y: 0.0, z: FOCAL_LENGTH};

    let mut img: RgbImage = ImageBuffer::new(IMG_X, IMG_Y);

    for x in 0..IMG_X {
        println!("Lines remaining: {}", IMG_X - x);
        for y in 0..IMG_Y {

            let u = x as f64 / (IMG_X-1) as f64;
            let v = y as f64 / (IMG_Y-1) as f64;
            let camera_ray = ray::Ray {
                t: 0.0,
                origin: ORIGIN, 
                direction: lower_left_corner + u*HORIZONTAL + u*VERTICAL - ORIGIN
            };

            *img.get_pixel_mut(x, y) = get_final_color(&camera_ray);
        }
    }

    img.save("img.png").unwrap();
}
