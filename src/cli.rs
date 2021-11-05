pub struct CliOptions {
    pub img_x: u32,
    pub img_y: u32,
    pub max_depth: u32,
    pub num_samples: u32,
    pub output_file: String,
    pub scene_file: String,
    pub silent: bool,
    pub aspect_ratio: f64,
}

pub fn read_cli() -> CliOptions {

    // validates wether or not a number is a unsigned int
    fn is_uint_validator(s: &str) -> Result<(), String> {
        let re = regex::Regex::new(r"^[0-9]+$").unwrap();

        if re.is_match(&s) {
            Ok(())
        } else {
            Err(String::from("Expected input of the form: 16/9"))
        }
    }

    // define CLI arguments
    let matches = clap::App::new("rayo")
        .version("0.1")
        .about("render beautiful images")
        .author("Jorge Romeu. <jorge.romeu.huidobro@gmail.com>")
        // silent flag
        .arg(
            clap::Arg::with_name("silent")
                .short("s")
                .long("silent")
                .required(false)
                .takes_value(false)
                .help("If set, do not print progressbar or render duration"),
        )
        // output file
        .arg(
            clap::Arg::with_name("output-file")
                .short("o")
                .long("out")
                .value_name("FILE")
                .help("Rendered image path")
                .takes_value(true)
                .default_value("render.png"),
        )
        // scene file
        .arg(
            clap::Arg::with_name("scene-file")
                .value_name("SCENE")
                .help("The Scene JSON file")
                .required(true),
        )
        // resolution
        .arg(
            clap::Arg::with_name("resolution")
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
            clap::Arg::with_name("aspect")
                .short("a")
                .long("aspect")
                .value_name("ASPECT-RATIO")
                .help("Aspect ratio")
                .default_value("16/9")
                .validator(|s| {
                    let re = regex::Regex::new(r"^[0-9]+/[0-9]$").unwrap();

                    if re.is_match(&s) {
                        Ok(())
                    } else {
                        Err(String::from("Expected input of the form: 16/9"))
                    }
                }),
        )
        // max recursion depth
        .arg(
            clap::Arg::with_name("max-depth")
                .short("d")
                .long("depth")
                .value_name("MAX-DEPTH")
                .help("Maximum recursion depth")
                .default_value("30")
                .validator(|s| is_uint_validator(&s)),
        )
        // number of smaples per pixel
        .arg(
            clap::Arg::with_name("num-samples")
                .short("n")
                .long("num-samples")
                .value_name("NUM-SAMPLES")
                .help("Number of samples per pixel")
                .default_value("100")
                .validator(|s| is_uint_validator(&s)),
        )
        .get_matches();

    // otuput file
    let output_file_name = matches.value_of("output-file").unwrap_or_default();
    let output_file = String::from(output_file_name);

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

    // num samples
    let num_samples: u32 = matches
        .value_of("num-samples")
        .unwrap_or_default()
        .parse()
        .unwrap();

    // silent
    let silent: bool = matches.is_present("silent");

    CliOptions {
        output_file,
        scene_file,
        img_x,
        img_y,
        max_depth,
        num_samples,
        silent,
        aspect_ratio,
    }
}
