use clap::{App, Arg, SubCommand};

pub struct CliArgs {
    pub img_x: u32,
    pub img_y: u32,
    pub max_depth: u32,
    pub scene_file: String,
    pub aspect_ratio: f64,
    pub subcmd_args: SubCommandArgs,
}

#[derive(Clone)]
pub enum SubCommandArgs {
    ImgArgs {
        num_samples: u32,
        output_file: String,
    },
    DbgArgs {
        pixel_x: u32,
        pixel_y: u32,
    },
    GuiArgs {}
}

pub fn read_cli() -> CliArgs {
    // validates wether or not a number is a unsigned int
    fn is_uint_validator(s: &str) -> Result<(), String> {
        let re = regex::Regex::new(r"^[0-9]+$").unwrap();

        if re.is_match(&s) {
            Ok(())
        } else {
            Err(String::from("Expected input of the form: 16/9"))
        }
    }

    let matches = App::new("rayo")
        .arg(
            Arg::with_name("scene-file")
                .value_name("SCENE")
                .help("The Scene JSON file")
                .required(true),
        )
        // resolution
        .arg(
            Arg::with_name("resolution")
                .short("r")
                .long("resolution")
                .value_name("RESOLUTION")
                .help("Horizontal image resolution")
                .default_value("480")
                .takes_value(true)
                .validator(|s| is_uint_validator(&s)),
        )
        // aspect ratio
        .arg(
            Arg::with_name("aspect")
                .short("a")
                .long("aspect")
                .value_name("ASPECT-RATIO")
                .help("Aspect ratio")
                .default_value("16/9")
                .validator(|s| {
                    let re = regex::Regex::new(r"^[0-9]+/[0-9]+$").unwrap();

                    if re.is_match(&s) {
                        Ok(())
                    } else {
                        Err(String::from("Expected input of the form: 16/9"))
                    }
                }),
        )
        // max recursion depth
        .arg(
            Arg::with_name("max-depth")
                .short("d")
                .long("depth")
                .value_name("MAX-DEPTH")
                .help("Maximum recursion depth")
                .default_value("30")
                .validator(|s| is_uint_validator(&s)),
        )
        .subcommand(
            SubCommand::with_name("img")
                .about("Render scene to an image")
                // number of smaples per pixel
                .arg(
                    Arg::with_name("num-samples")
                        .short("n")
                        .long("num-samples")
                        .value_name("NUM-SAMPLES")
                        .help("Number of samples per pixel")
                        .default_value("100")
                        .validator(|s| is_uint_validator(&s)),
                )
                .arg(
                    Arg::with_name("output-file")
                        .short("o")
                        .long("out")
                        .value_name("OUTPUT-FILE")
                        .help("Output file")
                        .default_value("render.png"),
                ),
        )
        .subcommand(
            App::new("dbg").about("Debug rayo").arg(
                Arg::with_name("pixel")
                    .value_name("PIXEL-COORDS")
                    .help("The pixel coords for which to run the raytracer: x,y")
                    .default_value("0,0")
                    .validator(|s| {
                        let re = regex::Regex::new(r"^[0-9]+,[0-9]+$").unwrap();

                        if re.is_match(&s) {
                            Ok(())
                        } else {
                            Err(String::from("Expected input of the form: x/y with x and y being integers"))
                        }
                    }),
            ),
        )
        .subcommand(
            App::new("gui").about("Run with GUI")
        )
        .get_matches();

    // otuput file
    let scene_file_name = matches.value_of("scene-file").unwrap_or_default();
    let scene_file = String::from(scene_file_name);

    // aspect ratio
    let aspect: Vec<&str> = matches
        .value_of("aspect")
        .unwrap_or_default()
        .split("/")
        .collect();

    let aspect_x: f64 = aspect[0].parse().unwrap();
    let aspect_y: f64 = aspect[1].parse().unwrap();
    let aspect_ratio = aspect_x / aspect_y;

    // image dimensions
    let img_x: u32 = matches
        .value_of("resolution")
        .unwrap_or_default()
        .parse()
        .unwrap();

    let img_y: u32 = (img_x as f64 / aspect_ratio) as u32;

    // max depth
    let max_depth: u32 = matches
        .value_of("max-depth")
        .unwrap_or_default()
        .parse()
        .unwrap();

    // read matches
    let subcmd_args = match matches.subcommand() {
        ("img", Some(img_matches)) => {
            // num samples
            let num_samples: u32 = img_matches
                .value_of("num-samples")
                .unwrap_or_default()
                .parse()
                .unwrap();

            // otuput file
            let output_file_name = img_matches.value_of("output-file").unwrap_or_default();
            let output_file = String::from(output_file_name);

            SubCommandArgs::ImgArgs {
                num_samples,
                output_file,
            }
        }
        ("dbg", Some(dbg_matches)) => {
            let pixel: Vec<&str> = dbg_matches
                .value_of("pixel")
                .unwrap_or_default()
                .split(",")
                .collect();

            let pixel_x: u32 = pixel[0].parse().unwrap();
            let pixel_y: u32 = pixel[1].parse().unwrap();

            SubCommandArgs::DbgArgs { pixel_x, pixel_y }
        }
        ("gui", Some(gui_matches)) => {
            SubCommandArgs::GuiArgs {}
        }
        _ => panic!(),
    };

    CliArgs {
        img_x,
        img_y,
        max_depth,
        aspect_ratio,
        scene_file,
        subcmd_args,
    }
}
