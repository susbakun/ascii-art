#![allow(clippy::all, clippy::pedantic)]

use image::{ImageReader, imageops::FilterType};
use getopts::Options;

const ASCII_CHARS: &'static str = "`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

struct Color {
    r: u8,
    g: u8,
    b: u8
}

enum BrightnessMethod {
    Average,
    MinMax,
    Luminosity
}

fn get_brightness_method(method: &str) -> BrightnessMethod {
    match method {
        "min_max" => BrightnessMethod::MinMax,
        "luminosity" => BrightnessMethod::Luminosity,
        _ => BrightnessMethod::Average,
    }
}

fn brightness_map(color: &Color, method: &BrightnessMethod) -> u8 {
    let Color {r , g, b} = color;

    // converting channels to f32 (preventing overflow)
    let r = *r as f32;
    let g = *g as f32;
    let b = *b as f32;

    match method {
        BrightnessMethod::Average => ((r + g + b) / 3.0) as u8,
        BrightnessMethod::MinMax => {
            let min = [r, g, b].into_iter()
                .min_by(|a, b| a.partial_cmp(b)
                .unwrap_or(std::cmp::Ordering::Equal)
            ).unwrap_or(r);

            let max = [r, g, b].into_iter()
                .max_by(|a, b| a.partial_cmp(&b)
                .unwrap_or(std::cmp::Ordering::Equal)
            ).unwrap_or(r);

            ((min + max) / 2.0) as u8 
        },
        BrightnessMethod::Luminosity => (0.21 * r + 0.72 * g + 0.07 * b) as u8
    }
}

// Map brightness value (0-255) to ASCII character index (0-63)
fn linear_map(value: usize, invert: bool) -> usize {
    let value = if invert {
        255 - value
    }else {
        value
    };

    (value * 63) / 255
}

fn print_image(image: &Vec<Vec<char>>) {
    image.iter().for_each(|row| {
        row.iter().for_each(|pixel| {
            // Printing 3 times to stretch image back
            print!("{pixel}");
            print!("{pixel}");
            print!("{pixel}");
        });
        println!()
    });
}

fn print_usage(opts: &Options) {
    let brief = format!("Usage: [options]");
    println!("{}", opts.usage(&brief));
}

fn main() {
    let mut opts = getopts::Options::new();

    opts.optopt("m", "method", "Brightness method",
        "Minmax or average");
    opts.optopt("f", "file", "Image file name", 
        "File Name");

    opts.optflag("i", "invert", "Invert color");
    opts.optflag("h", "help", "Print help menu");

    let matches = opts.parse(std::env::args().skip(1))
        .unwrap_or_else(|f| {
            eprintln!("Error: {}", f);
            print_usage(&opts);
            std::process::exit(1);
        });
    
    if matches.opt_present("h"){
        print_usage(&opts);
        return;
    }

    let str_arg = |flag: &str, default: &str| -> String {
        matches.opt_str(flag).unwrap_or(default.to_string())
    };


    let brightness_method_str = str_arg("m", "average");
    let brightness_method = get_brightness_method(&brightness_method_str);

    let file_path = str_arg("f", "./kaiji_kun.jpg");

    let invert = matches.opt_present("i");

    let img = ImageReader::open(file_path)
        .expect("Couldn't read the file");

    // Resize to fit terminal (accounting for 3x horizontal stretch and ~2:1 character aspect ratio)
    // Target: ~120-150 characters wide, so resize to ~40-50 pixels wide
    let target_width = 50;
    let target_height = 40;
    
    let img_buffer = img
        .decode()
        .expect("Failed to get decode the image")
        .resize(target_width, target_height,
            FilterType::Lanczos3)
        .to_rgb8();

    let mut pixel_matrix = vec![];
    for row in img_buffer.rows(){
        let mut row_pixels = vec![];
        row.for_each(|pixel| {
            row_pixels.push(pixel.0);
        });
        pixel_matrix.push(row_pixels);
    }

    let mut brightness_matrix = vec![];
    pixel_matrix.iter().for_each(|row| {
        let mut row_pixels = vec![];
        row.iter().for_each(|pixel| {
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            let color = Color { r, g, b };

            let brightness = brightness_map(&color, &brightness_method);
            row_pixels.push(brightness);
        });
        brightness_matrix.push(row_pixels);
    });

    let mut ascii_matrix = vec![];
    brightness_matrix.iter().for_each(|row| {
        let mut row_asciis = vec![];
        row.iter().for_each(|brightness| {
            let mapped_value_index = linear_map(*brightness as usize, invert);
            let ascii_char = ASCII_CHARS.chars().nth(mapped_value_index)
                .unwrap_or('a');
            row_asciis.push(ascii_char);
        });
        ascii_matrix.push(row_asciis)
    });

    let dimensions = img_buffer.dimensions();

    println!("Successfully loaded image!");
    println!("Image size: {} x {}", dimensions.0, dimensions.1);

    print_image(&ascii_matrix);
}
