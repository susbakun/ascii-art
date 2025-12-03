#![allow(clippy::all, clippy::pedantic)]

use image::{ImageReader, imageops::FilterType};
use std::env;

const ASCII_CHARS: &'static str = "`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

// Map brightness value (0-255) to ASCII character index (0-63)
fn linear_map(value: usize) -> usize {
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

fn main() {
    let file_path =  env::args()
        .skip(1)
        .next()
        .expect("You should provide a valid path");

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
            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;
            let average = ((r + g + b) / 3.0).floor() as u8;
            row_pixels.push(average);
        });
        brightness_matrix.push(row_pixels);
    });

    let mut ascii_matrix = vec![];
    brightness_matrix.iter().for_each(|row| {
        let mut row_asciis = vec![];
        row.iter().for_each(|brightness| {
            let mapped_value = linear_map(*brightness as usize);
            let ascii_char = ASCII_CHARS.chars().nth(mapped_value)
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
