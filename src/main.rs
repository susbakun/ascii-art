#![allow(clippy::all, clippy::pedantic)]

mod prelude;
mod terminal;
mod brightness;
mod commandline;
mod matrix;

use image::{ImageReader, imageops::FilterType};
use commandline::CommandLine;


fn main() {
    let cl =  CommandLine::new();

    let brightness_method = cl.str_arg("m", "average");
    let file_path = cl.str_arg("f", "./kaiji_kun.jpg");

    let invert = cl.is_present("i");
    let in_green = cl.is_present("g");

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

    let brightness_method = brightness::get_brightness_method(
            &brightness_method);

    let pixel_matrix = 
        matrix::get_pixel_matrix(&img_buffer);
    let brightness_matrix = 
        matrix::get_brightness_matrix(&pixel_matrix, &brightness_method);
    let ascii_matrix = 
        matrix::get_ascii_matrix(&brightness_matrix, invert);

    let dimensions = img_buffer.dimensions();

    println!("Successfully loaded image!");
    println!("Image size: {} x {}", dimensions.0, dimensions.1);

    terminal::print_image(&ascii_matrix, &brightness_matrix, in_green)
        .expect("Couldn't print the image");
}
