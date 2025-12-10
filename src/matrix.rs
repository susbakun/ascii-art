use image::{ImageBuffer, Rgb as ImageRGB};
use crate::brightness::{self, BrightnessMethod};
use crate::prelude::*;

pub fn get_pixel_matrix(
    img_buffer: &ImageBuffer<ImageRGB<u8>, Vec<u8>>
    ) -> Vec<Vec<[u8;3]>> 
{
    let mut pixel_matrix = vec![];
    for row in img_buffer.rows(){
        let mut row_pixels = vec![];
        row.for_each(|pixel| {
            row_pixels.push(pixel.0);
        });
        pixel_matrix.push(row_pixels);
    };

    pixel_matrix
}

pub fn get_brightness_matrix(
    pixel_matrix: &Vec<Vec<[u8;3]>>, 
    brightness_method: &BrightnessMethod
    ) -> Vec<Vec<u8>> 
{
    let mut brightness_matrix = vec![];
    pixel_matrix.iter().for_each(|row| {
        let mut row_pixels = vec![];
        row.iter().for_each(|pixel| {
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            let color = Rgb{ r, g, b };

            let brightness = brightness::brightness_map(&color, brightness_method);
            row_pixels.push(brightness);
        });
        brightness_matrix.push(row_pixels);
    });

    brightness_matrix
}

pub fn get_ascii_matrix(
    brightness_matrix: &Vec<Vec<u8>>,
    invert: bool
    ) -> Vec<Vec<char>> 
{
    let mut ascii_matrix = vec![];
    brightness_matrix.iter().for_each(|row| {
        let mut row_asciis = vec![];
        row.iter().for_each(|brightness| {
            let mapped_value_index = brightness::linear_map(
                *brightness as usize, invert);
            let ascii_char = ASCII_CHARS.chars().nth(mapped_value_index)
                .unwrap_or('a');
            row_asciis.push(ascii_char);
        });
        ascii_matrix.push(row_asciis)
    });

    ascii_matrix
}