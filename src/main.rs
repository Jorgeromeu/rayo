extern crate image;

use image::{ImageBuffer, RgbImage};
use intersection::Hittable;
use rand::Rng;
use crate::vec::Vec3;
use crate::ray::Ray;
use crate::color::Color;

mod camera;
mod intersection;
mod ray;
mod vec;
mod color;

// image TODO read from cli
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMG_X: u32 = 250;
const IMG_Y: u32 = (IMG_X as f64 / ASPECT_RATIO) as u32;
    
// anti aliasing
const NUM_SAMPLES: u32 = 100;

// recursive max depth
const MAX_DEPTH: u32 = 40;

fn get_final_color(ray: &ray::Ray, scene: &intersection::Scene, depth: u32) -> Color {

    // if we have exceeded the depth limit no more light is gathered
    if depth > MAX_DEPTH {
        return Color::new(0.0, 0.0, 0.0);
    }

    // intersect scene
    let hit = scene.intersect(ray, 0.0, f64::MAX);

    if hit.is_hit {
        let target = hit.point + hit.normal + Vec3::random_in_unit_sphere();
        let recursive_ray = Ray::new(hit.point, target - hit.point);
        return 0.5 * get_final_color(&recursive_ray, scene, depth+1);

        // fake shading
        // let vec_color = 0.6 * (hit_info.normal + Vec3::new(1.0, 1.0, 1.0));
        // return Color::new(vec_color.x, vec_color.y, vec_color.z);
    }

    // no hit
    return Color::sky(ray);
}

fn construct_scene() -> intersection::Scene {
    // main sphere
    let sphere = intersection::Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };

    let floor = intersection::Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0
    };

    // add spheres
    let mut scene = intersection::Scene::empty();
    scene.spheres.push(sphere);
    scene.spheres.push(floor);
    scene
}

fn main() {
    let mut rng = rand::thread_rng();

    // Construct Camera
    let camera = camera::Camera::new(Vec3::zero(), 1.0, ASPECT_RATIO, 2.0);

    // Construct Scene
    let scene = construct_scene();

    // Initialize image
    let mut img: RgbImage = ImageBuffer::new(IMG_X, IMG_Y);

    // Main ray tracing loop
    for y in (0..IMG_Y).rev() {
        println!("y is: {}", y);
        for x in 0..IMG_X {
            // compute normalized pixel positions
            let u = x as f64 / (IMG_X - 1) as f64;
            let v = y as f64 / (IMG_Y - 1) as f64;

            let mut color = Color::black();

            // sample several times
            for _ in 0..NUM_SAMPLES {
                let dx = rng.gen_range(0..10) as f64 / 3000.0;
                let dy = rng.gen_range(0..10) as f64 / 3000.0;
                let secondary_ray = camera.generate_ray(u+dx, v+dy);
                color += get_final_color(&secondary_ray, &scene, 0);
            }

            // set pixel
            *img.get_pixel_mut(x, IMG_Y-1-y) = color.to_pixel(NUM_SAMPLES);
        }
    }

    img.save("img.png").unwrap();
}
