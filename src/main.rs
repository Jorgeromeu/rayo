use core::num;
use std::{clone, time};

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
use clap;

mod camera;
mod intersection;
mod ray;
mod vec;
mod color;
mod material;

fn ray_color(ray: &ray::Ray, scene: &intersection::Scene, depth: u32, max_depth: u32) -> Color {

    // if we have exceeded the depth limit no more light is gathered
    if depth > max_depth {
        return Color::black();
    }

    // intersect scene
    let hit = scene.intersect(ray, 0.1, f64::MAX);

    match hit {
        Some(hit) => {
       
            let (attenuation, scattered_ray, should_scatter) = hit.material.scatter(ray, hit);
            
            if should_scatter {

                // recurse
                attenuation * ray_color(&scattered_ray, scene, depth+1, max_depth)
            } else {
                Color::black()
            }

        },
        None => Color::sky(ray)
    }
}

fn construct_scene() -> intersection::Scene {
    // main sphere
    let sphere = intersection::Sphere {
        center: Vec3::new(-0.5, 0.0, -1.0),
        radius: 0.4,
        material: Material::Lambertian {albedo: Color::new(0.8, 0.1, 0.1)}
    };
    
    let metalic = intersection::Sphere {
        center: Vec3::new(0.5, 0.0, -1.0),
        radius: 0.4,
        material: Material::Metal {albedo: Color::new(0.9, 0.9, 0.9), fuzz: 0.5}
    };

    let floor = intersection::Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Material::Lambertian {albedo: Color::new(0.1, 0.8, 0.1)}
    };

    // add spheres
    let mut scene = intersection::Scene::empty();
    scene.spheres.push(sphere);
    scene.spheres.push(metalic);
    scene.spheres.push(floor);
    scene
}

struct CliOptions {
    img_x: u32,
    img_y: u32,
    aspect_ratio: f64,
    max_depth: u32,
    num_samples: u32,
    output_file: String
}

fn read_cli() -> CliOptions {

    // define CLI arguments
    let matches = clap::App::new("rayo")
        .version("0.1")
        .about("render beautiful images")
        .author("Jorge Romeu. <jorge.romeu.huidobro@gmail.com>")

        // output file
        .arg(clap::Arg::with_name("output-file")
            .short("o")
            .long("out")
            .value_name("FILE")
            .help("Rendered image path")
            .takes_value(true)
            .default_value("render.png")
            .required(false))

        // resolution
        .arg(clap::Arg::with_name("resolution")
            .short("r")
            .long("resolution")
            .value_name("RESOLUTION")
            .help("Horizontal image resolution")
            .default_value("480")
            .takes_value(true))

        // aspect ratio
        .arg(clap::Arg::with_name("aspect")
            .short("a")
            .long("aspect")
            .value_name("ASPECT-RATIO")
            .help("Aspect ratio")
            .default_value("16/9"))

        // max recursion depth
        .arg(clap::Arg::with_name("max-depth")
            .short("d")
            .long("depth")
            .value_name("MAX-DEPTH")
            .help("Maximum recursion depth")
            .default_value("30"))
        
        // number of smaples per pixel
        .arg(clap::Arg::with_name("num-samples")
            .short("n")
            .long("num-samples")
            .value_name("NUM-SAMPLES")
            .help("Number of samples per pixel")
            .default_value("100"))
        .get_matches();

    // otuput file
    let output_file_name = matches.value_of("output-file").unwrap_or_default();
    let output_file = String::from(output_file_name);
   
    // aspect ratio
    let aspect: Vec<&str> = matches.value_of("aspect").unwrap_or_default().split("/").collect();
    let aspect_x: f64 = aspect[0].parse().unwrap();
    let aspect_y: f64 = aspect[1].parse().unwrap();
    let aspect_ratio = aspect_x / aspect_y;
   
    // image dimensions
    let img_x: u32 = matches.value_of("resolution").unwrap_or_default().parse().unwrap();
    let img_y: u32 = (img_x as f64 / aspect_ratio) as u32;

    // max depth
    let max_depth: u32 = matches.value_of("max-depth").unwrap_or_default().parse().unwrap();

    // num samples
    let num_samples: u32 = matches.value_of("num-samples").unwrap_or_default().parse().unwrap();

    CliOptions {
        output_file,
        img_x,
        img_y,
        aspect_ratio,
        max_depth,
        num_samples
    }
}

fn main() {

    // read CLI args
    let opts = read_cli();

    // Construct Camera
    let camera = camera::Camera::new(Vec3::zero(), 1.0, opts.aspect_ratio, 2.0);

    // Construct Scene
    let scene = construct_scene();

    // Initialize image
    let mut img: RgbImage = ImageBuffer::new(opts.img_x, opts.img_y);

    let mut pixels: Vec<(u32, u32, &mut Rgb<u8>)> = img.enumerate_pixels_mut().collect();

    // Initialize progressbar
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

    // write image
    img.save(opts.output_file).unwrap();
}
