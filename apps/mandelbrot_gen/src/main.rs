extern crate image;
extern crate ndarray;
// NOTE: This loads macros from the external crate after
#[macro_use]
extern crate serde_derive;
extern crate serde;
// NOTE: As in C++ the import order counts (docopt must come after the two
// above)
extern crate docopt;
extern crate rendering;

use ndarray::arr1;
use std::fs::File;
use std::path::Path;

use docopt::Docopt;
use serde::Deserialize;

use rendering::mandelbrot::compute;

// TODO Add/use actual default values
const USAGE: &'static str = "
 Usage:
     mandelbrot [--iter=<max_iter> \
                 --width=<image_w> \
                 --height=<image_h> \
                 --x0=<from_real> \
                 --x1=<to_real> \
                 --y0=<from_imaginary> \
                 --y1=<to_imaginary>]

     mandelbrot [--help]
 Options:
     --help                    Show this screen.
     --iter=<max_iter>         Maximum number of iterations [default: 50].
     --width=<image_w>         Image width [default: 1024] 
     --height=<image_h>        Image height [default: 768]
     --x0=<from_real>          Maximum in the real axis [default: -2.0].
     --x1=<to_real>            Maximum in the real axis [default: 1.0].
     --y0=<from_imaginary>     Maximum in the imaginary axis [default: -1.2].
     --y1=<to_imaginary>       Maximum in the imaginary axis [default: 1.2].";

#[derive(Debug, Deserialize)]
struct Args {
    flag_help: bool,
    flag_iter: u32,
    flag_width: u32,
    flag_height: u32,
    flag_x0: Option<f64>,
    flag_x1: Option<f64>,
    flag_y0: Option<f64>,
    flag_y1: Option<f64>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_help {
        println!("{}", USAGE);
        return;
    }
    println!("> Computing with arguments: {:?}", args);

    let max_iter: u32 = args.flag_iter;
    let image_size = arr1(&[args.flag_width, args.flag_height]);
    let range_x = arr1(&[args.flag_x0.unwrap(), args.flag_x1.unwrap()]);
    let range_y = arr1(&[args.flag_y0.unwrap(), args.flag_y1.unwrap()]);

    // TODO Learn more about reference/value/move in Rust (e.g. ownership)
    let image = compute(max_iter, image_size, range_x, range_y);
    let ref mut file_out = File::create(&Path::new("fractal.png")).unwrap();
    let _result = image::ImageRgba8(image).save(file_out, image::PNG);
    println!("> Mandelbrot Saved!");
}
