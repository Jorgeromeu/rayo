use std::time;

use image::{ImageBuffer, Rgb, RgbImage};
use material::Material;
use rayon;
use intersection::Hittable;
use rand;
use rayon::iter::*;
use indicatif::ProgressBar;
use crate::vec::Vec3;
use crate::ray::Ray;
use crate::color::Color;

mod camera;
mod intersection;
mod ray;
mod vec;
mod color;
mod material;

// image TODO read from cli
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMG_X: u32 = 1920;
const IMG_Y: u32 = (IMG_X as f64 / ASPECT_RATIO) as u32;
    
// anti aliasing
const NUM_SAMPLES: u32 = 1000;

// recursive max depth
const MAX_DEPTH: u32 = 30; 

fn ray_color(ray: &ray::Ray, scene: &intersection::Scene, depth: u32) -> Color {

    // if we have exceeded the depth limit no more light is gathered
    if depth > MAX_DEPTH {
        return Color::black();
    }

    // intersect scene
    let hit = scene.intersect(ray, 0.1, f64::MAX);

    match hit {
        Some(hit) => {
       
            let (attenuation, scattered_ray) = hit.material.scatter(ray, hit);
            
            // recurse
            attenuation * ray_color(&scattered_ray, scene, depth+1)

        },
        None => Color::sky(ray)
    }
}

fn construct_scene() -> intersection::Scene {
    // main sphere
    let sphere = intersection::Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Material {albedo: Color::new(0.8, 0.1, 0.1)}
    };

    let floor = intersection::Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Material {albedo: Color::new(0.1, 0.8, 0.1)}
    };

    // add spheres
    let mut scene = intersection::Scene::empty();
    scene.spheres.push(sphere);
    scene.spheres.push(floor);
    scene
}

fn main() {
    
    // Construct Camera
    let camera = camera::Camera::new(Vec3::zero(), 1.0, ASPECT_RATIO, 2.0);

    // Construct Scene
    let scene = construct_scene();

    // Initialize image
    let mut img: RgbImage = ImageBuffer::new(IMG_X, IMG_Y);

    // Enumerate the pixels
    let mut pixels: Vec<(u32, u32, &mut Rgb<u8>)> = img.enumerate_pixels_mut().collect();

    // Initialize progressbar
    let bar = ProgressBar::new((IMG_X * IMG_Y) as u64);


    let start_time = time::Instant::now();

    // main ray tracing loop 
    pixels.par_iter_mut().for_each(|tup| {

        let x = tup.0;
        let y = IMG_Y - 1 - tup.1; 

        // start with a black color
        let mut color = Color::black();

        // sample several times
        for _ in 0..NUM_SAMPLES {

            let u = ((x as f64) + rand::random::<f64>()) / (IMG_X - 1) as f64;
            let v = ((y as f64) + rand::random::<f64>() as f64) / (IMG_Y - 1) as f64;

            let secondary_ray = camera.generate_ray(u, v);
            color += ray_color(&secondary_ray, &scene, 0);
        }

        // write pixel to image buffer
        let final_pix = color.to_pixel(NUM_SAMPLES);
        *(tup.2) = final_pix;

        // increment progressbar 
        bar.inc(1);
    });
    
    let elapsed = start_time.elapsed();

    // finish progressbar
    bar.finish();

    println!("rendering took: {} seconds", elapsed.as_secs());

    // write image
    img.save("img.png").unwrap();
}
