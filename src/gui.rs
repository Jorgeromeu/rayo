use std::fs;

use image::{ImageBuffer, Rgba, RgbaImage};
use piston::{Event, RenderEvent, WindowSettings};
use piston_window::{PistonWindow, TextureSettings};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::{
    camera::Camera, cli::CliArgs, color::Color, intersection::scene::Scene, parsing, ray_color,
};

struct App {
    pub scene: Scene,
    pub camera: Camera,
    pub opts: CliArgs,
}

pub fn run_gui(opts: CliArgs) {
    // create window
    let mut window: PistonWindow = WindowSettings::new("rayo", [opts.img_x, opts.img_y])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    // read scene
    let scene_json = fs::read_to_string(&opts.scene_file).unwrap();
    let (scene, camera) = parsing::parse_scene(scene_json, opts.aspect_ratio);

    // construct app
    let mut app = App {
        scene,
        camera,
        opts,
    };

    app.run(&mut window);
}

impl App {
    pub fn run(&mut self, window: &mut PistonWindow) {
        while let Some(event) = window.next() {
            self.update(event.clone());
            self.render(event.clone(), window);
        }
    }

    pub fn update(&mut self, event: Event) {}

    pub fn render(&mut self, event: Event, window: &mut PistonWindow) {
        // TODO handle event

        let mut texture_ctx = window.create_texture_context();
        let texture_settings = TextureSettings::new();

        window.draw_2d(&event, |ctx, graphics, _device| {
            // clear graphics
            piston_window::clear([1.0; 4], graphics);

            // Initialize image
            let mut img: RgbaImage = ImageBuffer::new(self.opts.img_x, self.opts.img_y);
            let mut pixels: Vec<(u32, u32, &mut Rgba<u8>)> = img.enumerate_pixels_mut().collect();

            // parallelized ray tracing loop
            pixels.par_iter_mut().for_each(|tup| {
                let x = tup.0;
                let y = self.opts.img_y - 1 - tup.1;

                // start with a black color
                let mut color = Color::black();

                let u = ((x as f64) + rand::random::<f64>()) / (self.opts.img_x - 1) as f64;
                let v = ((y as f64) + rand::random::<f64>() as f64) / (self.opts.img_y - 1) as f64;

                let ray = self.camera.generate_ray(u, v);
                color += ray_color(&ray, &(self.scene), 0, self.opts.max_depth);

                // write pixel to image buffer
                let final_pix = color.to_pixel_rgba(1);
                *(tup.2) = final_pix;
            });

            // load texture from image
            let texture = piston_window::Texture::from_image(
                &mut texture_ctx,
                &img,
                &texture_settings,
            )
            .unwrap();

            // draw image to window
            piston_window::image(&texture, ctx.transform, graphics)
        });
    }
}
