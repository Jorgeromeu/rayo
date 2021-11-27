use std::fs;

use image::{buffer::EnumeratePixelsMut, ImageBuffer, Rgba, RgbaImage};
use piston::{Button, ButtonArgs, ButtonState, Event, Input, Key, WindowSettings};
use piston_window::{PistonWindow, TextureSettings};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::{
    camera::Camera, cli::CliArgs, color::Color, intersection::scene::Scene, parsing, ray_color,
};

struct App {
    pub scene: Scene,
    pub camera: Camera,
    pub opts: CliArgs,
    pub paused: bool,
    pub moved: bool,
    pub num_samples: u32,
    pub frame: RgbaImage,
}

pub fn run_gui(opts: &CliArgs) {
    // create window
    let mut window: PistonWindow = WindowSettings::new("rayo", [opts.img_x, opts.img_y])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    // read scene
    let scene_json = fs::read_to_string(&opts.scene_file).unwrap();
    let (scene, camera) = parsing::parse_scene(scene_json, opts.aspect_ratio);

    // initialize frame
    let frame = ImageBuffer::new(opts.img_x, opts.img_y);

    // construct app
    let mut app = App {
        scene,
        camera,
        opts: opts.clone(),
        moved: true,
        paused: false,
        num_samples: 0,
        frame,
    };

    app.run(&mut window);
}

impl App {
    // run the application on the window
    pub fn run(&mut self, window: &mut PistonWindow) {
        while let Some(event) = window.next() {
            self.update(event.clone());
            self.render(event.clone(), window);
        }
    }

    // process input and update state
    pub fn update(&mut self, event: Event) {
        match self.moved {
            true => {
                self.compute_initial_sample();
                self.num_samples = 1;
                self.moved = false;
            }
            false => {
                if self.num_samples < 500 {
                    self.compute_additional_sample();
                    self.num_samples += 1;
                }

                // println!("samples: {:?}", self.num_samples);
            }
        }

        match event {
            Event::Input(
                Input::Button(ButtonArgs {
                    state: ButtonState::Press,
                    button: Button::Keyboard(Key::P),
                    scancode: _,
                }),
                _,
            ) => {
                self.paused = !self.paused;
            }
            _ => {}
        }
    }

    // render to window
    pub fn render(&mut self, event: Event, window: &mut PistonWindow) {
        let mut texture_ctx = window.create_texture_context();
        let texture_settings = TextureSettings::new();

        if self.paused {
            return;
        }

        window.draw_2d(&event, |ctx, graphics, _device| {
            // clear graphics
            piston_window::clear([1.0; 4], graphics);

            // load texture from frame
            let texture = piston_window::Texture::from_image(
                &mut texture_ctx,
                &self.frame,
                &texture_settings,
            )
            .unwrap();

            // draw image to window
            piston_window::image(&texture, ctx.transform, graphics)
        });
    }

    fn compute_additional_sample(&mut self) {
        // iterate over pixels
        // let frame = &mut (self.frame);
        let mut img = self.frame.clone();
        let mut pixels: Vec<(u32, u32, &mut Rgba<u8>)> = img.enumerate_pixels_mut().collect();

        // parallelized ray tracing loop
        pixels.par_iter_mut().for_each(|tup| {
            let x = tup.0;
            let y = self.opts.img_y - 1 - tup.1;

            // start with previous color
            let running_avg = Color::from_pixel_rgba(*(tup.2));

            let u = ((x as f64) + rand::random::<f64>()) / (self.opts.img_x - 1) as f64;
            let v = ((y as f64) + rand::random::<f64>() as f64) / (self.opts.img_y - 1) as f64;

            let ray = self.camera.generate_ray(u, v);
            let color = ray_color(&ray, &(self.scene), 0, self.opts.max_depth);

            let n = self.num_samples + 1;

            let final_color = running_avg + (color - running_avg) / (n as f64);

            // write pixel to image buffer
            let final_pix = final_color.to_pixel_rgba(1);
            *(tup.2) = final_pix;
        });

        self.frame = img;
    }

    fn compute_initial_sample(&mut self) {
        // iterate over pixels
        let mut img = self.frame.clone();
        let mut pixels: Vec<(u32, u32, &mut Rgba<u8>)> = img.enumerate_pixels_mut().collect();

        // parallelized ray tracing loop
        pixels.par_iter_mut().for_each(|tup| {
            let x = tup.0;
            let y = self.opts.img_y - 1 - tup.1;

            let u = (x as f64) / (self.opts.img_x - 1) as f64;
            let v = (y as f64) / (self.opts.img_y - 1) as f64;

            let ray = self.camera.generate_ray(u, v);
            let color = ray_color(&ray, &(self.scene), 0, self.opts.max_depth);

            // write pixel to image buffer
            let final_pix = color.to_pixel_rgba(1);
            *(tup.2) = final_pix;
        });

        self.frame = img;
    }
}
