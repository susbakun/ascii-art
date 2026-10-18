use image::{ImageReader, imageops::FilterType};

use crate::{brightness::BrightnessMethod, matrix, prelude::ColorMode, terminal};

pub fn render_photo(
    path: &str,
    w: usize,
    h: usize,
    brightness_method: BrightnessMethod,
    invert: bool,
    color_mode: &ColorMode,
) {
    let img = ImageReader::open(path).expect("Couldn't read the file");
    let img_buffer = img
        .decode()
        .expect("Failed to get decode the image")
        .resize(w as u32, h as u32, FilterType::Lanczos3)
        .to_rgb8();

    let pixel_matrix = matrix::get_pixel_matrix(&img_buffer);
    let brightness_matrix = matrix::get_brightness_matrix(&pixel_matrix, &brightness_method);
    let ascii_matrix = matrix::get_ascii_matrix(&brightness_matrix, invert);

    let dimensions = img_buffer.dimensions();

    println!("Successfully loaded image!");
    println!("Image size: {} x {}", dimensions.0, dimensions.1);

    terminal::print_image(&ascii_matrix, &brightness_matrix, &pixel_matrix, color_mode)
        .expect("Couldn't print the image");
}
