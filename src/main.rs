use crate::color::Color;
use crate::ray::Ray;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use intersection::{scene::Scene, Hittable};
use rayon::iter::*;
use std::{fs, time};

mod camera;
mod cli;
mod color;
mod intersection;
mod material;
mod parsing;
mod ray;
mod texture;
mod vec;

fn ray_color(ray: &Ray, scene: &Scene, depth: u32, max_depth: u32) -> Color {
    // if we have exceeded the depth limit no more light is gathered
    if depth > max_depth {
        return Color::black();
    }

    let t_min = 0.1;
    let t_max = f64::MAX;

    // intersect scene
    match scene.intersect(ray,t_min, t_max) {

        // if hit, scatter
        Some(hit) => match hit.material.scatter(ray, hit) {
            Some((attenuation, scattered_ray)) => {
                attenuation * ray_color(&scattered_ray, scene, depth + 1, max_depth)
            }
            None => Color::black(),
        },

        // if no intersection return sky color
        None => Color::sky(ray),
    }
}

fn main() {
    // read CLI args
    let opts = cli::read_cli();

    // Construct Scene
    let scene_json = fs::read_to_string(&opts.scene_file).unwrap();
    let (scene, camera) = parsing::parse_scene(scene_json, opts.aspect_ratio);

    // Initialize image
    let mut img: RgbImage = ImageBuffer::new(opts.img_x, opts.img_y);

    let mut pixels: Vec<(u32, u32, &mut Rgb<u8>)> = img.enumerate_pixels_mut().collect();

    // Initialize progressbar with number of pixels
    let bar = ProgressBar::new((opts.img_x * opts.img_y) as u64);

    let start_time = time::Instant::now();

    // parallelized ray tracing loop
    pixels.par_iter_mut().for_each(|tup| {
        let x = tup.0;
        let y = opts.img_y - 1 - tup.1;

        // start with a black color
        let mut color = Color::black();

        // sample several times
        for _ in 0..opts.num_samples {
            let u = ((x as f64) + rand::random::<f64>()) / (opts.img_x - 1) as f64;
            let v = ((y as f64) + rand::random::<f64>() as f64) / (opts.img_y - 1) as f64;

            let secondary_ray = camera.generate_ray(u, v);
            color += ray_color(&secondary_ray, &scene, 0, opts.max_depth);
        }

        // write pixel to image buffer
        let final_pix = color.to_pixel(opts.num_samples);
        *(tup.2) = final_pix;

        // increment progressbar
        bar.inc(1);
    });

    let elapsed = start_time.elapsed();

    // finish progressbar
    bar.finish();
    println!("rendering took: {} seconds", elapsed.as_secs());

    // write image to file
    img.save(opts.output_file).unwrap();
}
