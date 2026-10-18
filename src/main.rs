#![allow(clippy::all, clippy::pedantic)]

mod brightness;
mod commandline;
mod matrix;
mod photo;
mod prelude;
mod terminal;
mod video;

use commandline::CommandLine;

use crate::{photo::render_photo, prelude::ColorMode, video::render_video};

fn main() {
    let cl = CommandLine::new();

    let brightness_method = cl.str_arg("m", "average");

    let invert = cl.is_present("i");

    let color_mode: ColorMode = if cl.is_present("c") {
        ColorMode::TrueColor
    } else if cl.is_present("g") {
        ColorMode::Green
    } else {
        ColorMode::Mono
    };

    // Resize to fit terminal (accounting for 3x horizontal stretch and ~2:1 character aspect ratio)
    // Target: ~120-150 characters wide, so resize to ~40-50 pixels wide
    let target_width = 50;
    let target_height = 40;

    let brightness_method = brightness::get_brightness_method(&brightness_method);

    // handle videos here
    if cl.is_present("v") {
        let video_path = cl.str_arg("v", "./assets/test_video.mp4");

        render_video(
            &video_path,
            target_width,
            target_height,
            brightness_method,
            invert,
            &color_mode,
        );
        return;
    }

    let image_path = cl.str_arg("p", "./assets/kaiji_kun.jpg");
    render_photo(
        &image_path,
        target_width,
        target_height,
        brightness_method,
        invert,
        &color_mode,
    );
}
