extern crate image;

use image::{ImageBuffer, Pixel, RgbImage};
use intersection::Hittable;
use rand::Rng;
use crate::vec::{Vec3};
use crate::color::{Color};

mod camera;
mod intersection;
mod ray;
mod vec;
mod color;

fn get_final_color(ray: &ray::Ray, scene: &intersection::Scene) -> Color {
    // intersect scene
    let hit_info = scene.intersect(ray);

    // shade (using fake shading)
    if hit_info.is_hit {
        let red = (hit_info.normal.x * 200.0) as u32;
        let green = (hit_info.normal.y * 200.0) as u32;
        let blue = (hit_info.normal.z * 200.0) as u32;

        return Color::new(red, green, blue);
    }

    // no hit
    return Color::black();
}

fn construct_scene() -> intersection::Scene {
    // main sphere
    let sphere = intersection::Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };

    // add spheres
    let mut scene = intersection::empty_scene();
    scene.spheres.push(sphere);
    scene
}

fn main() {
    let mut rng = rand::thread_rng();

    // image TODO read from cli
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMG_X: u32 = 500;
    const IMG_Y: u32 = (IMG_X as f64 / ASPECT_RATIO) as u32;

    // anti aliasing
    const NUM_AA_RAYS: u32 = 5;

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

            let camera_ray = camera.generate_ray(u, v);
            let mut color = get_final_color(&camera_ray, &scene);

            // Anti aliasing rays
            for _ in 0..NUM_AA_RAYS {
                let dx = rng.gen_range(0..10) as f64 / 3000.0;
                let dy = rng.gen_range(0..10) as f64 / 3000.0;

                let secondary_ray = camera.generate_ray(u+dx, v+dy);
                color += get_final_color(&secondary_ray, &scene);
            }
            color /= NUM_AA_RAYS+1;

            color.rescale();
            *img.get_pixel_mut(x, y) = color.to_pixel();
        }
    }

    img.save("img.png").unwrap();
}
