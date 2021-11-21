use crate::ray::Ray;
use crate::texture::Texture;
use crate::{cli::SubCommandArgs, color::Color};
use camera::Camera;
use image::{ImageBuffer, Rgb, RgbImage, Rgba, RgbaImage};
use indicatif::ProgressBar;
use intersection::{scene::Scene, Hittable};
use piston::WindowSettings;
use piston_window::{G2dTexture, PistonWindow, TextureSettings};
use rayon::iter::*;
use std::{fs, time};

mod camera;
mod cli;
mod color;
mod gui;
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

    // intersect scene
    let hit = scene.intersect(ray, 0.1, f64::MAX);

    match hit {
        // if hit, scatter
        Some(hit) => {
            // get hit emmision from hit
            let emitted = hit.material.emmit(hit.u, hit.v, hit.point);

            // scatter
            match hit.material.scatter(ray, hit) {
                // if material scatters, scatter
                Some((attenuation, scattered_ray)) => {
                    emitted + attenuation * ray_color(&scattered_ray, scene, depth + 1, max_depth)
                }

                // else illuminate scene
                None => emitted,
            }
        }

        // if no hit return sky color
        // None => Color::white(),
        None => Color::black(),
    }
}

// fn pixel_color_incremental(
//     running_avg: Color,
//     samples_so_far: u32,
//     u: f64,
//     v: f64,
//     max_depth: u32,
//     cam: &Camera,
//     scene: &Scene,
// ) -> Color {
//     let ray = cam.generate_ray(u, v);
//     let this_color = ray_color(&ray, &scene, 0, max_depth)

// }

fn main() {
    // read CLI args
    let opts = cli::read_cli();
    let subcmd = opts.subcmd_args.clone();

    match subcmd {
        SubCommandArgs::ImgArgs {
            num_samples,
            output_file,
        } => {
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
                for _ in 0..num_samples {
                    let u = ((x as f64) + rand::random::<f64>()) / (opts.img_x - 1) as f64;
                    let v = ((y as f64) + rand::random::<f64>() as f64) / (opts.img_y - 1) as f64;

                    let ray = camera.generate_ray(u, v);
                    color += ray_color(&ray, &scene, 0, opts.max_depth);
                }

                // write pixel to image buffer
                let final_pix = color.to_pixel(num_samples);
                *(tup.2) = final_pix;

                // increment progressbar
                bar.inc(1);
            });

            let elapsed = start_time.elapsed();

            // finish progressbar
            bar.finish();
            println!("rendering took: {} seconds", elapsed.as_secs());

            // write image to file
            img.save(output_file).unwrap();
        }
        SubCommandArgs::DbgArgs { pixel_x, pixel_y } => {
            // Construct Scene
            let scene_json = fs::read_to_string(&opts.scene_file).unwrap();
            let (scene, camera) = parsing::parse_scene(scene_json, opts.aspect_ratio);

            let u = ((pixel_x as f64) + rand::random::<f64>()) / (opts.img_x - 1) as f64;
            let v = ((pixel_y as f64) + rand::random::<f64>() as f64) / (opts.img_y - 1) as f64;

            let ray = camera.generate_ray(u, v);
            ray_color(&ray, &scene, 0, opts.max_depth);
        }
        SubCommandArgs::GuiArgs {} => gui::run_gui(opts),
    }
}
