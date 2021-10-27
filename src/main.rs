extern crate image;

use image::{ImageBuffer, RgbImage};
use intersection::Hittable;

mod vec;
mod ray;
mod intersection;

fn get_final_color(ray: &ray::Ray, scene: &intersection::Scene) -> image::Rgb<u8> {

    // intersect scene
    let hit_info = scene.intersect(ray);

    // shade (using fake shading)
    if hit_info.is_hit {
        let red = (hit_info.normal.x * 200.0) as u8;
        let green = (hit_info.normal.y * 200.0) as u8;
        let blue = (hit_info.normal.z * 200.0) as u8;
        return image::Rgb([red, green, blue])
    } 

    // no hit
    return image::Rgb([0, 0, 0])
}

fn construct_scene() -> intersection::Scene {
    
    let mut scene = intersection::empty_scene();

    // main sphere
    let sphere = intersection::Sphere {
        center: vec::Vec3 {x: 0.0, y: 0.0, z: -1.0},
        radius: 0.5
    };
    scene.spheres.push(sphere);
   
    // floor sphere
    let floor = intersection::Sphere {
        center: vec::Vec3 {
            x: 0.0,
            y: -100.5,
            z: -1.0
        },
        radius: 100.0
    };
    scene.spheres.push(floor);

    scene
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

    // Construct Scene
    let scene = construct_scene();

    // Initialize image
    let mut img: RgbImage = ImageBuffer::new(IMG_X, IMG_Y);

    for y in (0..IMG_Y).rev() {

        println!("y is: {}", y);

        for x in 0..IMG_X {

            let u = x as f64 / (IMG_X-1) as f64;
            let v = y as f64 / (IMG_Y-1) as f64;

            let camera_ray = ray::Ray {
                t: 0.0,
                origin: ORIGIN, 
                dir: lower_left_corner + u*HORIZONTAL + v*VERTICAL - ORIGIN
            };

            *img.get_pixel_mut(x, y) = get_final_color(&camera_ray, &scene);
        }
    }

    img.save("img.png").unwrap();
}
